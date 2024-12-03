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
    },
    Pipes {

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
            delay
        } => {
            ball(delay) 
        },
        Screensaver::Pipes    {} => { pipes() },
    }
}
