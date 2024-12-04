mod screensavers;
use screensavers::{
    toasters::toasters,
    ball::ball,
    dvd::dvd,
    pipes::pipes,
};

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Screensaver {
    DVD {
        #[arg(short = 'C', long = "disablecount", help = "Toggle the corner counter")]
        cornercounter: bool,
        #[arg(short = 'd', long = "delay", help = "The delay between frames in milliseconds", value_name = "DELAY", default_value = "70")]
        delay: u64,
    },
    Toasters {

    },
    Ball {
        #[arg(short = 'd', long = "delay", help = "The delay between frames in milliseconds", value_name = "DELAY", default_value = "30")]
        delay: u64,
        #[arg(short = 'f', long = "fancy", help = "Fancy mode. Requires a Nerd Font")]
        fancy: bool,
        #[arg(short = 'r', long = "reset", help = "Amount of characters needed until a reset", value_name = "CHARS", default_value = "2000")]
        reset: u64,
    },
    Pipes {
        #[arg(short = 't', long = "type", action = clap::ArgAction::Append, help = "The type of pipes used (Can be used multiple times)", value_name = "TYPE", default_values_t = vec![0])]
        types: Vec<usize>,
        #[arg(short = 'p', long = "pipes", help = "The amount of pipes", value_name = "AMOUNT", default_value = "1")]
        amount: u32,
        #[arg(short = 'f', long = "framerate", help = "The delay between movements", value_name = "DELAY", default_value = "20")]
        delay: u64,
        #[arg(short = 'R', long = "randomize", help = "Randomize the starting position of the pipes")]
        randomize: bool
    },
}

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    /// Subcommand to specify the screensaver type
    #[command(subcommand)]
    command: Screensaver,
    
    #[arg(short, long, help = "Print version")]
    version: bool,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
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
            types, amount, delay, randomize
        } => { 
            pipes(types, amount, delay, randomize) 
        },
    }
}
