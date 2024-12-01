mod screensavers;
use screensavers::{
    toasters::toasters,
    ball::ball,
    dvd::dvd,
};

use std::{ 
    env::args,
    process::exit,
};
use rand::Rng;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        eprintln!("Insufficient arguments. Usage:\nasciisavers dvd/toasters/ball/random");
        exit(1);
    }
    match args[1].as_str() {
        "toasters" => { toasters() },
        "dvd" => { dvd() },
        "ball" => { ball() },
        "random" => {
            match rand::thread_rng().gen_range(0..3) {
                0 => {
                    dvd()
                },
                1 => {
                    toasters()
                },
                2 => {
                    ball()
                },
                _ => {

                }
            }
        },
        _ => { println!("Incorrect Argument. Available arguments:\n dvd, toasters, ball, random"); } 
    }
}
