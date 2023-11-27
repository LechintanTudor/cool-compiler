mod args;

use crate::args::*;
use clap::Parser as _;
use cool_driver::{passes, ErrorLocation};
use cool_resolve::TyConfig;
use cooler_parser::{Parser, ParserData};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source = std::fs::read_to_string(args.crate_path)?;
    let mut parser_data = ParserData::default();

    let mut parser = Parser::new(&mut parser_data, &source);
    parser.parse_file_module_item().unwrap();
    println!("{:#?}", parser);

    return Ok(());

    let ty_config = TyConfig {
        i8_align: 1,
        i16_align: 2,
        i32_align: 4,
        i64_align: 8,
        i128_align: 8,
        f32_align: 4,
        f64_align: 8,
        ptr_size: 8,
    };

    let (parsed_crate, mut context, mut errors) =
        passes::p0_parse(ty_config, &args.crate_name, &args.crate_path);

    let defined_crate = passes::p1_define_items(parsed_crate, &mut context, &mut errors);

    passes::p2_generate_ast(&defined_crate, &mut context, &mut errors);

    for error in &errors {
        println!("Error: {}.", error.error);

        match error.location {
            ErrorLocation::File(ref path) => println!("Error: {}", path.display()),
            ErrorLocation::Source((source_id, span)) => {
                let file = &defined_crate.files[source_id];
                let path = file.paths.path.as_path().display();
                let (line, column) = file.line_offsets.get_position(span.start);
                println!(" -> '{path}', line {line}, column {column}");
            }
        }

        println!()
    }

    Ok(())
}
