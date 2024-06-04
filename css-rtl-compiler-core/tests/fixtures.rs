use std::{fs, path::PathBuf};

use pretty_assertions::assert_eq;
use swc_common::{input::StringInput, BytePos};
use swc_core::testing::fixture;
use swc_css_codegen::{
    writer::basic::{BasicCssWriter, BasicCssWriterConfig},
    CodeGenerator, CodegenConfig, Emit,
};
use swc_css_parser::{
    lexer::Lexer,
    parser::{Parser, ParserConfig},
};

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

#[fixture("tests/**/*.input.css")]
// #[test]
fn pass(file: PathBuf) {
    println!("file: {:?}", file);

    let mut output_file = file.clone();
    output_file.set_file_name(file.file_stem().unwrap());
    output_file = output_file.with_extension("output.css");

    let input = fs::read_to_string(&file).expect("failed to read input file");
    let output = fs::read_to_string(output_file).expect("failed to read output file");

    assert_eq!(
        css_rtl_compiler_core::convert_css(&print_css(&input)).unwrap(),
        print_css(&output)
    );
}
