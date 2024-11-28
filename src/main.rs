
use std::{
    thread,
    time::Duration,
    io::{stdout,Write},
    env::args,
    process::exit,
};
use rand::Rng;

use crossterm::{
    terminal::{
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
        Clear, ClearType::Purge,
        size,
    },
    cursor::{
        Hide, Show
    },
    event::{
        poll, read,
        Event,
    },
    ExecutableCommand,
};


fn printtoaster(pos: (u16, u16), frame: i32) {
    let f1 = [
        ",~~,",
        " \x1b[4m\\  \\\x1b[0m,~~,\x1b[0m",
        "|\\\x1b[4m==/  /=\x1b[0m\\ ",
        "| | \x1b[4m    \x1b[0m |",
        " \\|______|",
    ];
    let f2 = [
        "",
        " \x1b[4m,~~,\x1b[0m____\x1b[0m",
        "|\\\x1b[4m==,~~,=\x1b[0m\\ ",
        "| | \x1b[4m    \x1b[0m |",
        " \\|______|"
    ];
    let f3 = [
        "",
        " ________\x1b[0m",
        "|\\\x1b[4m=======\x1b[0m\\ ",
        "| | /  / |",
        " \\|\x1b[4m'~~'\x1b[0m__|"
    ];
    match frame % 4 {
        0 => {
            for i in 0..5 {
                print!("\x1b[{};{}H{}", i+1+pos.1, pos.0, f1[i as usize]);
            }
        },
        1 => {
            for i in 0..5 {
                print!("\x1b[{};{}H{}", i+1+pos.1, pos.0, f2[i as usize]);
            }
        },
        2 => {
            for i in 0..5 {
                print!("\x1b[{};{}H{}", i+1+pos.1, pos.0, f3[i as usize]);
            }
        },
        3 => {
            for i in 0..5 {
                print!("\x1b[{};{}H{}", i+1+pos.1, pos.0, f2[i as usize]);
            }
        },
        _ => {}
    };
}
fn toasters() { 
    let mut stdout = stdout();

    let mut toasters: Vec<(u16, u16, f32)> = vec![];
    let tsize = size().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Clear(Purge)).unwrap();
    stdout.execute(Hide).unwrap();
    enable_raw_mode().unwrap();

    loop {
        stdout.execute(Clear(Purge)).unwrap();
        for toaster in toasters.clone() {
            printtoaster((toaster.0, toaster.1), toaster.2.floor() as i32);
            let idx = toasters.iter().position(|&r| r == toaster).unwrap();
            toasters[idx] = (toasters[idx].0 - 1, toasters[idx].1, toasters[idx].2 + 0.5);
            if toasters[idx].0 == 0 {
                toasters.remove(idx);
            }
        }
        if rand::thread_rng().gen_range(0..16) == 1 {
            toasters.push((tsize.0-10, rand::thread_rng().gen_range(1..tsize.1-5), 0.0));
        }
        if poll(Duration::from_millis(0)).unwrap() {
            let read = read().unwrap();
            if let Event::Key(_) = read {
                break; 
            }
        }
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(70));
    }
    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(Show).unwrap();
    disable_raw_mode().unwrap();
}
fn dvd() {
    // Basic variables
    let mut pos;
    let mut vel = (rand::thread_rng().gen_range(-2..=2), rand::thread_rng().gen_range(-2..=2));
    
    // cool while loop
    while vel.0 == 0 || vel.1 == 0 {
        vel = (rand::thread_rng().gen_range(-2..=2), rand::thread_rng().gen_range(-2..=2)); // Randomizes until no values are 0
    }
    
    // terminal size
    let tsize = size().unwrap();

    // basic elementry school math
    pos = (tsize.0 as i32 / 2, tsize.1 as i32 / 2);
        

    // STDOUT WOO
    let mut stdout = stdout();
    
    // Corner counter
    let mut cornercount: f32 = 0.0;
    let colours = ["\x1b[38;2;255;61;61m","\x1b[38;2;255;200;61m","\x1b[38;2;255;255;61m","\x1b[38;2;61;255;61m","\x1b[38;2;61;255;255m","\x1b[38;2;61;61;255m","\x1b[38;2;255;61;255m"];
    let mut colouridx = 0;

    // boiler plate
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Clear(Purge)).unwrap();
    stdout.execute(Hide).unwrap();
    enable_raw_mode().unwrap();
    loop { 
        // Add the velocity to the position
        pos = (pos.0+vel.0, pos.1+vel.1); 
        // Clear the last dvd
        print!("\x1b[{};{}H                ", pos.1 - vel.1, pos.0 - vel.0);
        print!("\x1b[{};{}H                ", pos.1 - vel.1+1, pos.0 - vel.0);
        print!("\x1b[{};{}H                ", pos.1 - vel.1+2, pos.0 - vel.0);
        print!("\x1b[{};{}H                ", pos.1 - vel.1+3, pos.0 - vel.0);
        print!("\x1b[{};{}H                ", pos.1 - vel.1+4, pos.0 - vel.0);
        print!("\x1b[{};{}H                ", pos.1 - vel.1+5, pos.0 - vel.0);
        // Print the new one
        print!("\x1b[{};{}H{} _____   _____  ", pos.1, pos.0, colours[colouridx]);
        print!("\x1b[{};{}H|   \\ \\ / /   \\ ", pos.1+1, pos.0);
        print!("\x1b[{};{}H| |) \\ V /| |) |", pos.1+2, pos.0);
        print!("\x1b[{};{}H|___/ \\_/ |___/ ", pos.1+3, pos.0);
        print!("\x1b[{};{}H▄▄▄▄█▀▀▀▀▀▀█▄▄▄▄", pos.1+4, pos.0);
        print!("\x1b[{};{}H    ▀▀▀▀▀▀▀▀    \x1b[0m", pos.1+5, pos.0);
        
        stdout.flush().unwrap(); // flush the stdout

        // check if the dvd hit an edge
        if pos.0 >= tsize.0 as i32 - 16 || pos.0 <= 1 {
            vel = (-vel.0, vel.1);
            cornercount += 0.5;
            colouridx = rand::thread_rng().gen_range(0..7);
        } 
        if pos.1 >= tsize.1 as i32 - 5 || pos.1 <= 1 {
            vel = (vel.0, -vel.1);
            cornercount += 0.5;
            colouridx = rand::thread_rng().gen_range(0..7);
        }
        cornercount = cornercount.floor();
        print!("\x1b[1;1HCorner Hits: {:.0}", cornercount);

        // BOILER PLATE
        if poll(Duration::from_millis(0)).unwrap() {
            let read = read().unwrap();
            if let Event::Key(_) = read {
                break; 
            }
        }
        // Wait
        thread::sleep(Duration::from_millis(70));
    }
    
    // boiler plate
    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(Show).unwrap();
    disable_raw_mode().unwrap();
}
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        println!("Insufficient arguments. Usage:\nasciisavers dvd/toasters/random");
        exit(0);
    }
    match args[1].as_str() {
        "toasters" => { toasters() },
        "dvd" => { dvd() },
        "random" => {
            match rand::thread_rng().gen_range(0..2) {
                1 => {
                    dvd()
                },
                2 => {
                    toasters()
                },
                _ => {

                }
            }
        },
        _ => { println!("Incorrect Argument. Available arguments:\n dvd, toasters, random"); } 
    }
}
