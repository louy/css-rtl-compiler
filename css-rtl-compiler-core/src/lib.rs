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

#[derive(Debug)]
pub enum Error {
    ParserError(swc_css_parser::error::Error),
    CodegenError(std::fmt::Error),
}

pub fn convert_css(input: &str) -> Result<String, Error> {
    let cm: Lrc<SourceMap> = Default::default();
    let _handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let comments: swc_common::comments::SingleThreadedComments = Default::default();

    let config: ParserConfig = Default::default();

    let fm = cm.new_source_file(
        Lrc::new(swc_common::FileName::Custom("input.css".to_owned())),
        input.to_string(),
    );
    let lexer = Lexer::new(StringInput::from(&*fm), Some(&comments), config);
    let mut parser = Parser::new(lexer, config);

    let mut stylesheet = parser.parse_all().map_err(|e| Error::ParserError(e))?;

    let mut visitor = CSSRTLCompilerVisitor {};
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

    codegenerator
        .emit(&stylesheet)
        .map_err(|e| Error::CodegenError(e))?;

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
    fn test_retain_at_rule_order() {
        // Broken due to a bug in swc_css
        // https://github.com/swc-project/swc/issues/9986
        assert_eq!(
            print_css(
                r#"
:root {
  a: 1;
  &:where(.xyz) {
    b: 2;
  }
  .abc {
    c: 3;
  }
  d: 4;
}
"#
            ),
            r#"
:root {
  a: 1;
  &:where(.xyz) {
    b: 2;
  }
  .abc {
    c: 3;
  }
  d: 4;
}
"#
        );
    }
}
