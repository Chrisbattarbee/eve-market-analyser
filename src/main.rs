use clap::Parser;
use clap::Subcommand;

/// Interact with Eve Online Market APIs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    sub_command: SubCommands,
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
        #[clap(short, long, value_parser, default_value = "10000002")]
        region_code: String
    }
}


fn process_sub_commands(args: Args) {
    match args.sub_command {
        SubCommands::Download { download_sub_command } => {
            process_download_sub_command(download_sub_command);
        }
    }
}

fn process_download_sub_command(download_sub_command: DownloadSubCommand) {
    match download_sub_command {
        DownloadSubCommand::MarketItems { region_code } => {
            println!("You want to download data from {region_code}")
        }
    }
}

fn main() {
    let args = Args::parse();
    process_sub_commands(args);
}
