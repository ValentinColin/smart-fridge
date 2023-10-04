use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use log::debug;
use smart_fridge::API_PREFIX;
use std::string::String;
use uuid::Uuid;

/// Simple program to greet a person.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Subcommand of the cli.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Subcommand, Debug)]
enum Commands {
    /// Check the health of the server.
    Healthcheck,
    /// Get food.
    Get { uuid: Uuid },
    /// List all food.
    List,
    /// Add food.
    Add {
        name: String,
        expiration_date: NaiveDate,
    },
    /// Delete food.
    Delete { uuid: Uuid },
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let cli = Cli::parse();

    let addr = format!(
        "http://{}:{}{}",
        dotenvy::var("WEB_APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenvy::var("WEB_APP_PORT").unwrap_or("80".to_string()),
        API_PREFIX,
    );

    let client = reqwest::Client::new();

    match &cli.command {
        Commands::Healthcheck => {
            debug!("Healthcheck");
            let response = client.get(format!("{addr}/healthcheck")).send().await?;
            println!("{}", response.text().await?);
        }
        Commands::Get { uuid } => {
            debug!("Get {uuid}");
            let response = client.get(format!("{addr}/food/{uuid}")).send().await?;
            println!("{}", response.text().await?);
        }
        Commands::List => {
            debug!("List");
            let response = client.get(format!("{addr}/food")).send().await?;
            println!("{}", response.text().await?);
        }
        Commands::Add {
            name,
            expiration_date,
        } => {
            debug!("Add: {name} ({expiration_date})");
            let response = client
                .post(format!("{addr}/food"))
                .form(&[
                    ("name", name),
                    ("expiration_date", &expiration_date.to_string()),
                ])
                .send()
                .await?;
            println!("{}", response.text().await?);
        }
        Commands::Delete { uuid } => {
            debug!("Delete {uuid}");
            let response = client.delete(format!("{addr}/food/{uuid}")).send().await?;
            println!("{}", response.text().await?);
        }
    };

    Ok(())
}
