pub mod screensavers;
pub use screensavers::{
    toasters::toasters,
    ball::ball,
    dvd::dvd,
    pipes::pipes,
};

pub use clap::{Parser, Subcommand, CommandFactory};

use std::process::exit;

#[derive(Subcommand)]
enum Screensaver {
    #[command(about = "The DVD screeensaver")]
    DVD {
        #[arg(short = 'C', long = "disablecount", help = "Toggle the corner counter")]
        cornercounter: bool,
        #[arg(short = 'd', long = "delay", help = "The delay between frames in milliseconds", value_name = "DELAY", default_value = "70")]
        delay: u64,
    },

    #[command(about = "Flying Toasters from afterdark")]
    Toasters {

    },

    #[command(about = "Spinoff of the windows ball screensavers")]
    Ball {
        #[arg(short = 'd', long = "delay", help = "The delay between frames in milliseconds", value_name = "DELAY", default_value = "30")]
        delay: u64,
        #[arg(short = 'f', long = "fancy", help = "Fancy mode. Requires a Nerd Font")]
        fancy: bool,
        #[arg(short = 'r', long = "reset", help = "Amount of characters needed until a reset", value_name = "CHARS", default_value = "2000")]
        reset: u64,
    },

    #[command(about = "The windows 95 pipes screensavers")]
    Pipes {
        #[arg(short = 't', long = "type", action = clap::ArgAction::Append, help = "The type of pipes used (Can be used multiple times)", value_name = "TYPE", default_values_t = vec![0])]
        types: Vec<usize>,
        #[arg(short = 'p', long = "pipes", help = "The amount of pipes", value_name = "AMOUNT", default_value = "1")]
        amount: u32,
        #[arg(short = 'd', long = "delay", help = "The delay between movements", value_name = "DELAY", default_value = "20")]
        delay: u64,
        #[arg(short = 'R', long = "randomize", help = "Randomize the starting position of the pipes")]
        randomize: bool,
        #[arg(short = 'c', long = "colour", help = "Colours 0-7 (Can be used multiple times)", action = clap::ArgAction::Append, value_name = "COLOR", default_values_t = vec![0, 1, 2, 3, 4, 5, 6, 7])]
        colours: Vec<usize>,
        #[arg(short = 's', long = "stats", help = "Disables the stats in the corner")]
        stats: bool,
    },
}

#[derive(Parser)]
#[command(author, about, long_about = None, version, disable_version_flag = true)]
pub struct Cli {
    /// Subcommand to specify the screensaver type
    #[command(subcommand)]
    command: Option<Screensaver>,
    
    #[arg(short = 'v', long = "version", help = "Print the version and exit")]
    version: bool,
}

fn main() {
    let cli = Cli::parse();
    if cli.version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }
    if let Some(command) = cli.command {
        match command {
            Screensaver::Toasters {

            } => { 
                toasters() 
            },
            Screensaver::DVD      { 
                cornercounter, delay
            } => { 
                dvd(cornercounter, delay)
            },
            Screensaver::Ball     {
                delay, fancy, reset
            } => {
                ball(delay, fancy, reset) 
            },
            Screensaver::Pipes    {
                types, amount, delay, randomize, colours, stats
            } => { 
                pipes(types, amount, delay, randomize, colours, stats) 
            },
        }
    }
    else {
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        println!();
        exit(1);
    }
}
