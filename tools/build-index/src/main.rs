use clap::Parser;
use std::path::PathBuf;

const DEFAULT_ROOT_PATH: &str = "../..";
const DEFAULT_SCHEMA_VERSION: u32 = 1;
const DEFAULT_OUTPUT_PATH: &str = "vehicles.jsonl";

#[derive(Parser)]
#[command(
    version,
    about = "Build an index file (vehicles.jsonl) from a database directory",
    long_about = None
)]
struct Cli {
    /// Database root directory containing versioned schema folders (v1, v2, ...)
    #[arg(short, long = "root", value_name = "ROOT", default_value = DEFAULT_ROOT_PATH)]
    root_dir: PathBuf,

    /// Database schema version
    #[arg(short, long, value_name = "SCHEMA", default_value_t = DEFAULT_SCHEMA_VERSION)]
    schema: u32,

    #[arg(
        short,
        long = "output",
        value_name = "OUTPUT",
        default_value = DEFAULT_OUTPUT_PATH,
        help = "Output index file path",
        long_help = "Output index file path\nRelative values resolve under ROOT/vSCHEMA\nWith defaults, this resolves to ../../v1/vehicles.jsonl"
    )]
    output_file: PathBuf,
}

fn parse_args() -> anyhow::Result<(u32, PathBuf, PathBuf)> {
    let cli = Cli::parse();

    let schema_version = cli.schema;

    let schema_dir = std::path::absolute(&cli.root_dir)?.join(format!("v{}", schema_version));
    anyhow::ensure!(
        schema_dir.exists(),
        "directory not found at schema v{} path: {:?}",
        schema_version,
        schema_dir
    );
    anyhow::ensure!(
        schema_dir.is_dir(),
        "resolved schema v{} path is not a directory: {:?}",
        schema_version,
        schema_dir
    );
    let schema_dir = std::fs::canonicalize(schema_dir)?;

    let output_file_path = {
        let candidate = PathBuf::from(&cli.output_file);
        if candidate.is_absolute() {
            candidate
        } else {
            schema_dir.join(candidate)
        }
    };
    anyhow::ensure!(
        output_file_path.extension() == Some(std::ffi::OsStr::new("jsonl")),
        "output file path must have a .jsonl extension: {:?}",
        output_file_path
    );

    Ok((schema_version, schema_dir, output_file_path))
}

/// Recursively scans & collects all paths relative to `scan_root_dir`.
fn scan_file_paths(
    scan_root_dir: &std::path::Path,
    current_dir: &std::path::Path,
    file_paths: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            scan_file_paths(scan_root_dir, &path, file_paths)?;
        } else {
            let relative_path = path.strip_prefix(scan_root_dir)?;
            file_paths.push(relative_path.to_path_buf());
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let (schema_version, scan_root_dir, output_file_path) = parse_args()?;
    let schema_prefix = PathBuf::from(format!("v{}", schema_version));

    println!("Building index from directory {}", scan_root_dir.display());

    // Scan for vehicle files
    let mut file_paths = Vec::new();
    scan_file_paths(&scan_root_dir, &scan_root_dir, &mut file_paths)?;
    println!("Found {} files", file_paths.len());

    // Write to file
    let mut output_file = std::fs::File::create(&output_file_path)?;
    match schema_version {
        1 => {
            let mut entries = Vec::with_capacity(file_paths.len());
            for relative_file_path in &file_paths {
                let schema_file_path = schema_prefix.join(relative_file_path);
                // Skip non-UTF8 paths
                let Some(schema_file_path_str) = schema_file_path.to_str() else {
                    println!("Skipping non-UTF8 path: {:?}", schema_file_path);
                    continue;
                };
                // Skip non-YAML files
                let Some(schema_path_without_ext) = schema_file_path_str.strip_suffix(".yaml")
                else {
                    println!("Skipping non-YAML file: {:?}", schema_file_path_str);
                    continue;
                };
                // Skip files that don't parse as valid VehicleSchemaV1 paths
                if schema_path_without_ext
                    .parse::<fastsim_schema::VehicleSchemaV1>()
                    .is_err()
                {
                    println!("Skipping unparsable file: {:?}", schema_file_path_str);
                    continue;
                }
                // Create index entry
                let index_entry = schema_file_path_str.parse::<fastsim_schema::IndexEntryV1>()?;
                entries.push(index_entry);
            }
            println!("Created {} index entries", entries.len());

            // Sort entries alphabetically by path
            entries.sort_by(|a, b| a.path.cmp(&b.path));

            // Write entries to output file in JSONL format
            fastsim_schema::write_jsonl_v1(&mut output_file, &entries)?;
        }
        _ => anyhow::bail!("unsupported schema version: {}", schema_version),
    };
    println!("Index saved to {}", output_file_path.display());

    Ok(())
}
