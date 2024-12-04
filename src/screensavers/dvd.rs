/*
DVD Screensaver

# Explanation
The dvd function will take argumets for disabling the corner counter and for setting the delay
It sets the position and velocity of the dvd randomizing the velocity until it's no longer 0 on any directions
It will then get the terminal size

It then adds a corner counter into memory that is a float for wierd flooring magic

It then uses crossterm stuff to do the following:
Enter an Alternate Screen,
Clear the Alternate Screen,
and then enables terminal Raw Mode

and then starts the loop

## In the loop
It will clear the last dvd by just printing spaces then prints the new dvd.
If the cornercounter bool is false it will then print the corner counter
It handels bounce detection by just checking it's position and for each axis it bounces on it increases cornercount by 0.5
Then it will floor the cornercounter so that it tracks corners correctly

then it flushes the standard output with `stdout.flush().unwrap()`

Then it checks for any terminal input with crossterm and then if it does then it exits the loop and returns back to the regular terminal

If there is no input then it will wait the delay it was given in milliseconds
*/

// Import shit
use std::{
    thread,
    time::Duration,
    io::{stdout,Write}, 
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

pub fn dvd(cornercounter: bool, delay: u64) {
    // Basic variables
    let mut pos;
    let mut vel = (rand::thread_rng().gen_range(-1..=1)*2, rand::thread_rng().gen_range(-1..=1));
    
    // cool while loop
    while vel.0 == 0 || vel.1 == 0 {
        vel = (rand::thread_rng().gen_range(-1..=1)*2, rand::thread_rng().gen_range(-1..=1)); // Randomizes until no values are 0
    }
    
    // terminal size
    let tsize = size().unwrap();

    // basic elementry school math
    pos = (rand::thread_rng().gen_range(2..tsize.0-17) as i32, rand::thread_rng().gen_range(2..tsize.1-6) as i32); 

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
        if !cornercounter {
            print!("\x1b[1;1HCorner Hits: {:.0}", cornercount);
        }
         stdout.flush().unwrap(); // flush the stdout

        // BOILER PLATE
        if poll(Duration::from_millis(0)).unwrap() {
            let read = read().unwrap();
            if let Event::Key(_) = read {
                break; 
            }
        }
        // Wait
        thread::sleep(Duration::from_millis(delay));
    }
    
    // boiler plate
    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(Show).unwrap();
    disable_raw_mode().unwrap();
}


