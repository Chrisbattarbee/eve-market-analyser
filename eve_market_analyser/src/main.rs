use clap::Parser;
use clap::Subcommand;
use eve_api::apis::configuration::Configuration;

/// Interact with Eve Online Market APIs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    sub_command: SubCommands,

    /// The location of the eve api server
    #[clap(short, long, value_parser, default_value = "https://esi.evetech.net/latest")]
    eve_api_location: String,
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
            let first_page_items = eve_api::apis::market_api::get_markets_region_id_types(&configuration, region_code, None, None, None).await.unwrap();
            println!("You want to download data from {:?}", first_page_items)
        }
    }
}

fn generate_api_client_configuration(eve_api_location: String) -> Configuration {
    Configuration {
        base_path: eve_api_location,
        user_agent: Some("eve-market-analyser/0.0.1".to_owned()),
        client: reqwest::Client::new(),
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
    let configuration = generate_api_client_configuration(args.eve_api_location);
    process_sub_commands(args.sub_command, configuration).await;
    Ok(())
}