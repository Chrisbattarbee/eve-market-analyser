extern crate core;

use clap::Parser;
use clap::Subcommand;
use eve_api::apis::configuration::Configuration;
use eve_api::apis::market_api::{get_markets_region_id_types};

/// Interact with Eve Online Market APIs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    sub_command: SubCommands,

    /// The location of the eve api server
    #[clap(short, long, value_parser, default_value = "https://esi.evetech.net/latest")]
    eve_api_location: String,

    /// The location of the https proxy to use with eve analyser
    #[clap(short, long, value_parser)]
    https_proxy_location: Option<String>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    /// Downloads market data for interrogation
    Download {
        #[clap(subcommand)]
        download_sub_command: DownloadSubCommand,
    }
}

#[derive(Subcommand, Debug)]
enum DownloadSubCommand {
    /// Downloads all market items in a region
    MarketItems {
        /// Region code, defaults to Jita
        #[clap(short, long, value_parser, default_value_t = 10000002)]
        region_code: i32
    }
}


async fn process_sub_commands(args: SubCommands, configuration: Configuration) {
    match args {
        SubCommands::Download { download_sub_command } => {
            process_download_sub_command(download_sub_command, configuration).await;
        }
    }
}

async fn process_download_sub_command(download_sub_command: DownloadSubCommand, configuration: Configuration) {
    match download_sub_command {
        DownloadSubCommand::MarketItems { region_code } => {
            let mut all_items = Vec::new();
            let mut page_number = None;
            loop {
                match get_markets_region_id_types(&configuration, region_code, None, None, page_number).await {
                    Ok(items) => {
                        let len = items.len();
                        items.into_iter().for_each(|item| all_items.push(item));
                        page_number = match page_number {
                            None => Some(2),
                            Some(x) => Some(x + 1),
                        };
                        match len {
                            // API always returns pages of 1000, if we attempt to get a page after this it throws an internal error
                            1000 => {}
                            _ => break,
                        }
                    }
                    Err(e) => panic!("HTTP request failed {}", e)
                }
            }
            println!("Items in region {}: {:?}", region_code, all_items);
        }
    }
}

fn generate_api_client_configuration(eve_api_location: String, https_proxy_location: Option<String>) -> Configuration {
    let mut client_builder = reqwest::Client::builder();
    client_builder = match https_proxy_location {
        None => client_builder,
        Some(location) => client_builder.proxy(reqwest::Proxy::https(location).expect("Could not build proxy"))
    };

    Configuration {
        base_path: eve_api_location,
        user_agent: Some("eve-market-analyser/0.0.1".to_owned()),
        client: client_builder.build().expect("Could not build api client"),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    let args = Args::parse();
    let configuration = generate_api_client_configuration(args.eve_api_location, args.https_proxy_location);
    process_sub_commands(args.sub_command, configuration).await;
    Ok(())
}