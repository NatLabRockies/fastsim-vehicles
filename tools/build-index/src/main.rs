use anyhow::{Context, ensure};
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
    ensure!(
        schema_dir.exists(),
        "directory not found at schema v{} path: {:?}",
        schema_version,
        schema_dir
    );
    ensure!(
        schema_dir.is_dir(),
        "resolved schema v{} path is not a directory: {:?}",
        schema_version,
        schema_dir
    );
    let schema_dir = std::fs::canonicalize(schema_dir)?;

    let output_file_path = {
        let candidate = std::path::PathBuf::from(&cli.output_file);
        if candidate.is_absolute() {
            candidate
        } else {
            schema_dir.join(candidate)
        }
    };
    ensure!(
        output_file_path.extension() == Some(std::ffi::OsStr::new("jsonl")),
        "output file path must have a .jsonl extension: {:?}",
        output_file_path
    );

    Ok((schema_version, schema_dir, output_file_path))
}

fn scan_file_paths(scan_root_dir: &std::path::Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();
    collect_relative_file_paths(scan_root_dir, scan_root_dir, &mut file_paths)?;
    Ok(file_paths)
}

fn collect_relative_file_paths(
    scan_root_dir: &std::path::Path,
    current_dir: &std::path::Path,
    file_paths: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_relative_file_paths(scan_root_dir, &path, file_paths)?;
        } else {
            let relative_path = path.strip_prefix(scan_root_dir)?;
            file_paths.push(relative_path.to_path_buf());
        }
    }

    Ok(())
}

fn get_date_from_path(path: &std::path::Path) -> Option<String> {
    Some("".to_string()) // Placeholder implementation
}

fn main() -> anyhow::Result<()> {
    let (schema_version, scan_root_dir, output_file_path) = parse_args()?;
    println!("Building index from directory {}", scan_root_dir.display());

    // Scan for vehicle files
    let file_paths = scan_file_paths(&scan_root_dir)?;

    // Write to file
    let mut output_file = std::fs::File::create(&output_file_path)?;
    match schema_version {
        1 => {
            let mut entries = Vec::with_capacity(file_paths.len());
            for path in &file_paths {
                // Skip files that don't parse as valid VehicleSchemaV1 paths
                if path
                    .to_str()
                    .with_context(|| format!("failed to convert path to string: {:?}", path))?
                    .strip_suffix(".yaml")
                    .with_context(|| format!("failed to strip .yaml suffix from path: {:?}", path))?
                    .parse::<fastsim_schema::VehicleSchemaV1>()
                    .is_err()
                {
                    continue;
                };
                // Create an IndexEntryV1
                let date_added = get_date_from_path(path);
                let entry = fastsim_schema::IndexEntryV1::new(
                    path.to_string_lossy(),
                    date_added.unwrap_or_default(),
                )?;
                entries.push(entry);
            }
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
