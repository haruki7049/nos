use clap::Parser;
use nostr_sdk::{Client, Event, EventBuilder, Keys, Kind, SecretKey, ToBech32};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let args: CLIArgs = CLIArgs::parse();
    let config: NosConfig = match args.config {
        Some(ref path) => confy::load_path(path)?,
        None => confy::load("nos", "config")?,
    };

    if args.generate_key {
        generate_key()?;
        return Ok(());
    }

    let client: Client = setup_client(&config).await?;
    let message = &args.message.expect("No message. Enter your message");

    send(client, message, &config).await?;
    println!("Sent your message to Nostr relays: \"{}\"", message);

    Ok(())
}

fn generate_key() -> Result<(), Box<dyn std::error::Error>> {
    // Generate new random keys
    let keys = Keys::generate();
    let pubkey = keys.public_key();
    let seckey = keys.secret_key();
    println!("Your public key: {}", pubkey.to_bech32()?);
    println!("Your secret key: {}", seckey.to_bech32()?);

    Ok(())
}

async fn send(
    client: Client,
    message: &String,
    nosconfig: &NosConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let seckey = SecretKey::parse(&nosconfig.seckey)?;
    let keys = Keys::new(seckey);

    let event: Event = EventBuilder::new(Kind::TextNote, message).sign_with_keys(&keys)?;

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

    #[arg(long, default_value_t = false)]
    generate_key: bool,

    #[arg(short, long)]
    message: Option<String>,
}
