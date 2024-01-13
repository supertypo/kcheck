use std::time::Duration;
use clap::Parser;
use kaspa_wrpc_client::{KaspaRpcClient, WrpcEncoding, result::Result, client::{ConnectOptions,ConnectStrategy}};
use kaspa_rpc_core::{api::rpc::RpcApi, GetServerInfoResponse};
use std::process::ExitCode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// wRPC url
    url: String,
    /// Optional encoding 'borsh' or 'json'
    #[arg(long)]
    encoding: Option<WrpcEncoding>,
    /// Optional verbose output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    verbose : Option<bool>,
    /// Optional timeout in milliseconds
    #[arg(long)]
    timeout: Option<u64>,
}

#[tokio::main]
async fn main() -> ExitCode {
    match check().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn check() -> Result<()> {

    let Cli {
        url,
        encoding,
        verbose,
        timeout
    } = Cli::parse();

    let encoding = encoding.unwrap_or(WrpcEncoding::Borsh);
    let verbose = verbose.unwrap_or(false);
    let timeout = timeout.unwrap_or(5_000);

    let client = KaspaRpcClient::new(encoding, url.as_str())?;

    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(timeout)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };

    client.connect(options).await?;

    let GetServerInfoResponse {
        is_synced,
        server_version,
        network_id,
        virtual_daa_score,
        ..
    } = client.get_server_info().await?;

    client.disconnect().await?;

    if verbose {
        let status = if is_synced { "synced" } else { "not-synced" };
        println!("kaspa/{server_version}/{network_id}/{virtual_daa_score}/{status}");
    }
    
    if is_synced {
        Ok(())
    } else {
        Err("node is not synced".into())
    }

}
