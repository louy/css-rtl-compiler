use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_css_parser::parser::ParserConfig;
use swc_css_parser::{lexer::Lexer, parser::Parser};
use swc_css_visit::FoldWith;

use swc_css_codegen::{
    writer::basic::{BasicCssWriter, BasicCssWriterConfig},
    CodeGenerator, CodegenConfig, Emit,
};

mod visitor;
use visitor::*;

pub fn convert_css(input: &str) -> Result<String, String> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let comments: swc_common::comments::SingleThreadedComments = Default::default();

    let config: ParserConfig = Default::default();

    let fm = cm.new_source_file(
        swc_common::FileName::Custom("input.css".to_owned()),
        input.to_string(),
    );
    let lexer = Lexer::new(StringInput::from(&*fm), Some(&comments), config);
    let mut parser = Parser::new(lexer, config);

    let stylesheet = parser.parse_all().expect("failed to parser module");

    let mut visitor = CSSRTLCompilerVisitor::new();
    let result = stylesheet.fold_with(&mut visitor);

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

    match codegenerator.emit(&result) {
        Ok(_) => {}
        Err(e) => return Err(format!("{:?}", e)),
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_parsing() {
        assert_eq!(
            convert_css("body { color: red; }").unwrap(),
            "body {\n  color: red;\n}"
        );
    }
}