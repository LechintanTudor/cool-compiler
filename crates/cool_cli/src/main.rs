mod args;

use cool_driver::{p0_parse, Crate, Project};
use cool_resolve::{ResolveContext, TyConfig};

fn main() -> anyhow::Result<()> {
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

    let project = Project {
        crates: vec![
            Crate {
                name: "test".into(),
                path: "../programs/test/src".into(),
                deps: vec![1],
            },
            Crate {
                name: "libc".into(),
                path: "../packages/libc/1.0.0/src".into(),
                deps: vec![],
            },
        ],
    };

    let mut context = ResolveContext::new(ty_config);
    p0_parse(&project, &mut context);
    println!("{:#?}", context);

    Ok(())
}
