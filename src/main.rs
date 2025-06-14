use clap::Parser;
use url::Url;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use nostr_sdk::{Event, EventBuilder, Kind, Keys, Client, SecretKey};

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
    println!("Sent your message to Nostr relays: \"{}\"", &args.message);

    Ok(())
}

async fn send(client: Client, args: &CLIArgs, nosconfig: &NosConfig) -> Result<(), Box<dyn std::error::Error>> {
    let seckey = SecretKey::parse(&nosconfig.seckey)?;
    let keys = Keys::new(seckey);

    let event: Event = EventBuilder::new(Kind::TextNote, &args.message)
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

    client.connect().await;

    Ok(client)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct NosConfig {
    relays: Vec<Url>,
    seckey: String,
}

#[derive(Debug, Clone, Parser)]
struct CLIArgs {
    #[arg(short, long)]
    config: Option<PathBuf>,

    message: String,
}
