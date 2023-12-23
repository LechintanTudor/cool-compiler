mod args;

use crate::args::Args;
use anyhow::bail;
use clap::Parser;
use cool_driver::pass;
use cool_resolve::{ResolveContext, TyConfig};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let data = match pass::read_project(&args.crates) {
        Ok(data) => data,
        Err((_, errors)) => {
            for error in errors {
                println!("{}\n", error);
            }

            bail!("Failed to read project");
        }
    };

    let mut context = ResolveContext::new(TyConfig {
        i8_align: 1,
        i16_align: 2,
        i32_align: 4,
        i64_align: 8,
        i128_align: 8,
        f32_align: 4,
        f64_align: 8,
        ptr_size: 8,
    });

    let project = pass::parse_project(&data, &mut context);
    println!("{:#?}", project);

    Ok(())
}
