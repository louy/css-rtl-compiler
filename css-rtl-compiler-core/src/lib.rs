use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_css_parser::parser::ParserConfig;
use swc_css_parser::{lexer::Lexer, parser::Parser};
use swc_css_visit::VisitMutWith;

use swc_css_codegen::{
    writer::basic::{BasicCssWriter, BasicCssWriterConfig},
    CodeGenerator, CodegenConfig, Emit,
};

#[macro_use]
extern crate lazy_static;

mod visitor;
use visitor::*;

pub fn convert_css(input: &str) -> Result<String, String> {
    let cm: Lrc<SourceMap> = Default::default();
    let _handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let comments: swc_common::comments::SingleThreadedComments = Default::default();

    let config: ParserConfig = Default::default();

    let fm = cm.new_source_file(
        swc_common::FileName::Custom("input.css".to_owned()),
        input.to_string(),
    );
    let lexer = Lexer::new(StringInput::from(&*fm), Some(&comments), config);
    let mut parser = Parser::new(lexer, config);

    let mut stylesheet = parser.parse_all().expect("failed to parser module");

    let mut visitor = CSSRTLCompilerVisitor::new();
    stylesheet.visit_mut_with(&mut visitor);

    let mut output = String::new();

    let writer = BasicCssWriter::new(
        &mut output,
        Default::default(),
        BasicCssWriterConfig {
            indent_type: swc_css::codegen::writer::basic::IndentType::Space,
            indent_width: 2,
            linefeed: swc_css::codegen::writer::basic::LineFeed::LF,
        },
    );
    let mut codegenerator = CodeGenerator::new(writer, CodegenConfig { minify: false });

    match codegenerator.emit(&stylesheet) {
        Ok(_) => {}
        Err(e) => return Err(format!("{:?}", e)),
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use swc_common::BytePos;

    fn print_css(input: &str) -> String {
        let config: ParserConfig = Default::default();

        let lexer = Lexer::new(
            StringInput::new(&input, BytePos(0), BytePos(input.len().try_into().unwrap())),
            None,
            config,
        );
        let mut parser = Parser::new(lexer, config);

        let stylesheet = parser.parse_all().expect("failed to parser module");

        let mut output = String::new();

        let writer = BasicCssWriter::new(
            &mut output,
            Default::default(),
            BasicCssWriterConfig {
                indent_type: swc_css::codegen::writer::basic::IndentType::Space,
                indent_width: 2,
                linefeed: swc_css::codegen::writer::basic::LineFeed::LF,
            },
        );
        let mut codegenerator = CodeGenerator::new(writer, CodegenConfig { minify: false });

        match codegenerator.emit(&stylesheet) {
            Ok(_) => {}
            Err(e) => panic!("{:?}", e),
        }

        output
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            print_css(&convert_css("body { color: red; }").unwrap()),
            print_css("body { color: red; }")
        );
    }

    #[test]
    fn test_simple_rule() {
        assert_eq!(
            print_css(
                &convert_css(
                    r#"
body {
    color: white;
    right: 0;
    left: 1;
    padding: 1px 2em 3rem calc(4);
    direction: ltr;
}
"#
                )
                .unwrap()
            ),
            print_css(
                r#"
body {
    color: white;
    &:where([dir=ltr], [dir=ltr] *) {
        right: 0;
        left: 1;
        padding: 1px 2em 3rem calc(4);
        direction: ltr;
    }
    &:where([dir=rtl], [dir=rtl] *) {
        left: 0;
        right: 1;
        padding: 1px calc(4) 3rem 2em;
        direction: rtl;
    }
}
"#
            )
        );
    }

    #[test]
    fn test_media_query() {
        assert_eq!(
            print_css(
                &convert_css(
                    r#"
@media (min-width: 600px) {
    #selector { padding-left: 11px; }
}
"#
                )
                .unwrap()
            ),
            print_css(
                r#"
@media (min-width: 600px) {
    #selector { 
        &:where([dir=ltr], [dir=ltr] *) {
            padding-left: 11px; 
        }
        &:where([dir=rtl], [dir=rtl] *) {
            padding-right: 11px; 
        }
    }
}
"#
            )
        );

        assert_eq!(
            print_css(
                &convert_css(
                    r#"
tag {
    @media (min-width: 600px) {
        margin: 1 2 3 4;
    }
}
"#
                )
                .unwrap()
            ),
            print_css(
                r#"
tag {
    @media (min-width: 600px) {
        &:where([dir=ltr], [dir=ltr] *) {
            margin: 1 2 3 4;
        }
        &:where([dir=rtl], [dir=rtl] *) {
            margin: 1 4 3 2;
        }
    }
}
"#
            )
        );
    }

    #[test]
    fn test_pseduo_class() {
        assert_eq!(
            print_css(
                &convert_css(
                    r#"
    :root {
        direction: ltr;
    }
    "#
                )
                .unwrap()
            ),
            print_css(
                r#"
    :root {
        &:where([dir=ltr], [dir=ltr] *) {
            direction: ltr;
        }
        &:where([dir=rtl], [dir=rtl] *) {
            direction: rtl;
        }
    }
    "#
            )
        );
    }
}
