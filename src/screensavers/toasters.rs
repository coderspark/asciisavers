/*
THE FLYING TOASTERS WOOOOOOOOO
*/


// import all the garbage

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

// burn box
struct Toaster {
    pub pos: (u16, u16),
    frame: f32,
    istoast: bool,
}
impl Toaster {
    pub fn new() -> Toaster {
        // get the terminal size
        let tsize = size().unwrap();

        // return the burn box
        Toaster {
            pos: (tsize.0-10, rand::thread_rng().gen_range(1..tsize.1-5)),
            frame: 0.0,
            istoast: rand::thread_rng().gen_bool(0.5),
        } 
    }
    pub fn update(&mut self) {
        // Hand drawn ascii
        let f1 = [
            ",~~,",
            " \x1b[4m\\  \\\x1b[24m,~~,",
            "|\\\x1b[4m==/  /=\x1b[24m\\ ",
            "| | \x1b[4m    \x1b[24m |",
            " \\|______|",
        ];
        let f2 = [
            "",
            " \x1b[4m,~~,\x1b[24m____",
            "|\\\x1b[4m==,~~,=\x1b[24m\\ ",
            "| | \x1b[4m    \x1b[24m |",
            " \\|______|"
        ];
        let f3 = [
            "",
            " ________\x1b[24m",
            "|\\\x1b[4m=======\x1b[24m\\ ",
            "| | /  / |",
            " \\|\x1b[4m'~~'\x1b[24m__|"
        ];

        let toast = [
            "",
            " __________",
            "|\\ ....... \\",
            "\\ \\_\x1b[4m·····\x1b[0m___\\",
            " \\|_________|"
        ];

        // cool float stuff lmao
        self.frame += 0.5;
        // Move the toaster
        self.pos = (self.pos.0 - 1, self.pos.1);
        // Draw the correct frames
        if !self.istoast {
            match self.frame.floor() as i32 % 4 {
                0 => {
                    for i in 0..5 {
                        print!("\x1b[{};{}H{}", i+1+self.pos.1, self.pos.0, f1[i as usize]);
                    }
                },
                1 => {
                    for i in 0..5 {
                        print!("\x1b[{};{}H{}", i+1+self.pos.1, self.pos.0, f2[i as usize]);
                    }
                },
                2 => {
                    for i in 0..5 {
                        print!("\x1b[{};{}H{}", i+1+self.pos.1, self.pos.0, f3[i as usize]);
                    }
                },
                3 => {
                    for i in 0..5 {
                        print!("\x1b[{};{}H{}", i+1+self.pos.1, self.pos.0, f2[i as usize]);
                    }
                },
                _ => {}
            }; 
        }
        else {
            for i in 0..5 {
                print!("\x1b[{};{}H{}", i+1+self.pos.1, self.pos.0, toast[i as usize]);
            }
        }
    }
}


pub fn toasters() { 
    // Stdout i guess
    let mut stdout = stdout();

    // create the t o a s t e r s
    let mut toasters: Vec<Toaster> = vec![];

    // Put the terminal in 'fun mode'
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Clear(Purge)).unwrap();
    stdout.execute(Hide).unwrap();
    enable_raw_mode().unwrap();

    loop {
        stdout.execute(Clear(Purge)).unwrap();
        let mut tmpremoveidxs = vec![]; // I know it's a bad name. Fix it in a pull request
        // Update the toasters
        for i in 0..toasters.len() {
            toasters[i].update();
            if toasters[i].pos.0 <= 1 {
                tmpremoveidxs.push(i);
            }
        }
        // deletus
        for n in 0..tmpremoveidxs.len() {
            toasters.remove(tmpremoveidxs[n]);
        }
        let _ = tmpremoveidxs; // Drop it from memory
        
        // Occasionally make new toasters
        if rand::thread_rng().gen_range(0..16) == 1 {
            toasters.push(Toaster::new());
        }
        // Boiler plate
        if poll(Duration::from_millis(0)).unwrap() {
            let read = read().unwrap();
            if let Event::Key(_) = read {
                break; 
            }
        }

        // Flush and wait
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(70));
    }
    // Put the terminal out of 'fun mode'
    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(Show).unwrap();
    disable_raw_mode().unwrap();
}
