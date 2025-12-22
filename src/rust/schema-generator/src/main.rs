use std::env;
use std::path::PathBuf;

fn main() -> schema_generator::Result<()> {
    let (schema_dir, out_dir) = parse_args(env::args().skip(1))?;
    let result = schema_generator::generate(&schema_dir, &out_dir)?;
    println!("Generated {}", result.output_path.display());
    Ok(())
}

fn parse_args<I>(mut args: I) -> schema_generator::Result<(PathBuf, PathBuf)>
where
    I: Iterator<Item = String>,
{
    let mut schema_dir = PathBuf::from(".");
    let mut out_dir = PathBuf::from("generated");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--schema-dir" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--schema-dir requires a value".to_string())?;
                schema_dir = PathBuf::from(value);
            }
            "--out-dir" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--out-dir requires a value".to_string())?;
                out_dir = PathBuf::from(value);
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other => {
                return Err(format!("Unknown argument: {other}").into());
            }
        }
    }

    Ok((schema_dir, out_dir))
}

fn print_usage() {
    println!(
        "schema-generator --schema-dir <path> --out-dir <path>\n\
         Defaults: schema-dir=., out-dir=generated"
    );
}
