mod config;
mod schema;

use std::path::PathBuf;

use clap::Parser;
use config::Config;
use miette::IntoDiagnostic;
use schema::file::raw::RawSchemaFile;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Generate type-safe deserializable TS classes from JSON/JS Object - defining schemas
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> miette::Result<()> {
    let Args { verbose, config } = Args::parse();

    init_logging(verbose);

    let config_path = config.unwrap_or(
        std::env::current_dir()
            .into_diagnostic()?
            .join("desergen.toml"),
    );
    let config_root_dir: PathBuf = config_path.parent().map(Into::into).ok_or(miette::miette!(
        "Couldn't get the parent directory of {config_path:?}"
    ))?;

    let Config {
        src_root,
        desergen_root,
        src_output_root,
        schemas,
    } = Config::from_path(config_path)?;

    let desergen_root = config_root_dir.join(desergen_root);
    let schemas_root_dir = desergen_root.join("schemas");

    if !schemas_root_dir.exists() {
        return Err(miette::miette!(
            "Schemas root path {schemas_root_dir:?} doesn't exist"
        ));
    }

    let raw_schema_files = schemas
        .into_iter()
        .map(|schema_mod_path| RawSchemaFile::open(&schemas_root_dir, schema_mod_path))
        .collect::<Result<Vec<_>, _>>()?;

    tracing::info!("Raw Schema Files: {raw_schema_files:#?}");

    Ok(())
}

fn init_logging(verbose: bool) {
    std::env::set_var(
        "RUST_LOG",
        match cfg!(debug_assertions) || verbose {
            true => "debug",
            false => "info",
        },
    );

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
