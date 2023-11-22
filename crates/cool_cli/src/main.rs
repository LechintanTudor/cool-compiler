mod args;

use crate::args::*;
use clap::Parser as _;
use cool_driver::{passes, ErrorLocation};
use cool_resolve::TyConfig;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

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

    let (mut context, parsed_crate, errors) =
        passes::p0_parse(ty_config, &args.crate_name, &args.crate_path);

    for error in errors {
        println!("Error: {}.", error.error);

        match error.location {
            ErrorLocation::File(path) => println!("Error: {}", path.display()),
            ErrorLocation::Source((source_id, span)) => {
                let file = &parsed_crate.files[source_id];
                let path = file.paths.path.as_path().display();
                let (line, column) = file.line_offsets.get_position(span.start);
                println!(" -> '{path}', line {line}, column {column}");
            }
        }

        println!()
    }

    let defined_crate = passes::p1_define_items(parsed_crate, &mut context)?;
    passes::p2_generate_ast(&defined_crate, &mut context)?;

    Ok(())
}
