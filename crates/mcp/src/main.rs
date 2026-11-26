mod hook;
mod server;

use std::{env, path::PathBuf};

use anyhow::{Context, Result, bail};
use rmcp::{ServiceExt, transport::stdio};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("AFK: {error:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let mut args = env::args_os().skip(1);
    let explicit_root = match args.next().as_deref().and_then(|arg| arg.to_str()) {
        None => None,
        Some("--root") => Some(PathBuf::from(
            args.next().context("--root requires a directory")?,
        )),
        Some("--hook") => {
            hook::run();
            return Ok(());
        }
        Some(_) => bail!("expected stdio mode, --root <directory>, or internal --hook mode"),
    };
    if args.next().is_some() {
        bail!("unexpected argument");
    }
    let server = server::Server::new(explicit_root);
    server.initialize_explicit_root().await?;
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
