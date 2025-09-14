mod repl;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// FoundationDB Directory Explorer CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Path to cluster file (defaults to platform default)
    #[arg(long)]
    cluster_file: Option<String>,

    /// Start in interactive (REPL) mode
    #[arg(long, short = 'i')]
    interactive: bool,

    /// Do not connect to FoundationDB (useful for --version/tests)
    #[arg(long)]
    no_connect: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List subdirectories at a path
    Ls {
        /// Directory path like /app/foo (root if omitted)
        path: Option<String>,
    },
    /// Scan key-values within a directory
    Scan {
        /// Directory path like /app/foo (root if omitted)
        path: Option<String>,
        /// Limit number of kv pairs
        #[arg(long, short = 'n', default_value_t = 50)]
        limit: usize,
        /// Optional raw byte prefix (supports \xHH escapes)
        #[arg(long, short = 'p')]
        prefix: Option<String>,
        /// Do not attempt tuple parsing for keys
        #[arg(long, short = 'r')]
        raw: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Only connect if needed
    let need_db = cli.interactive || matches!(cli.command, Some(_));
    let (network, db) = if need_db && !cli.no_connect {
        // Safety: we drop the handle at program end
        let network = unsafe { foundationdb::boot() };
        let db = match cli.cluster_file {
            Some(path) => foundationdb::Database::from_path(&path)?,
            None => foundationdb::Database::default()?,
        };
        (Some(network), Some(db))
    } else {
        (None, None)
    };

    if cli.interactive {
        let db = db.ok_or_else(|| anyhow::anyhow!("interactive mode requires a connection; omit --no-connect"))?;
        repl::run_repl(db).await?;
        drop(network);
        return Ok(());
    }

    match cli.command.unwrap() {
        Commands::Ls { path } => {
            let db = db.ok_or_else(|| anyhow::anyhow!("ls requires a connection; omit --no-connect"))?;
            util::ls_path(&db, util::parse_path(path.as_deref().unwrap_or("/"))).await?;
        }
        Commands::Scan { path, limit, prefix, raw } => {
            let db = db.ok_or_else(|| anyhow::anyhow!("scan requires a connection; omit --no-connect"))?;
            let prefix_bytes = if let Some(s) = prefix { Some(util::parse_bytes_literal(&s)?) } else { None };
            util::scan_path(&db, util::parse_path(path.as_deref().unwrap_or("/")), limit, prefix_bytes, raw).await?;
        }
    }
    drop(network);
    Ok(())
}
