use clap::Parser;
use url::Url;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use nostr_sdk::{Event, EventBuilder, Kind, Timestamp, Keys, Client, Filter, PublicKey, SecretKey};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let args: CLIArgs = CLIArgs::parse();
    let config: NosConfig = match args.config {
        Some(ref path) => confy::load_path(path)?,
        None => confy::load("nos", "config")?,
    };

    let client: Client = setup_client(&config).await?;

    send(client, &args, &config).await?;

    Ok(())
}

async fn send(client: Client, args: &CLIArgs, nosconfig: &NosConfig) -> Result<(), Box<dyn std::error::Error>> {
    let pubkey = PublicKey::parse(&nosconfig.pubkey)?;
    let seckey = SecretKey::parse(&nosconfig.seckey)?;
    let keys = Keys::new(seckey);

    let event: Event = EventBuilder::new(Kind::TextNote, &args.message)
        .build(pubkey)
        .sign_with_keys(&keys)?;

    client.send_event(&event).await?;

    Ok(())
}

async fn setup_client(nosconfig: &NosConfig) -> Result<Client, Box<dyn std::error::Error>> {
    // Reads keys from your seckey written on config.toml
    let keys: Keys = Keys::parse(&nosconfig.seckey)?;

    // Generates client
    let client: Client = Client::new(keys);

    // Add relays
    for url in &nosconfig.relays {
        client.add_relay(url).await?;
    }

    let subscription = Filter::new()
        .since(Timestamp::now());

    client.subscribe(subscription, None).await?;

    client.connect().await;

    Ok(client)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct NosConfig {
    relays: Vec<Url>,
    pubkey: String,
    seckey: String,
}

#[derive(Debug, Clone, Parser)]
struct CLIArgs {
    #[arg(short, long)]
    config: Option<PathBuf>,

    message: String,
}
