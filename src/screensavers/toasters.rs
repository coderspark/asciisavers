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

struct Toaster {
    pub pos: (u16, u16),
    frame: f32
}
impl Toaster {
    pub fn new() -> Toaster {
        let tsize = size().unwrap();
        Toaster {
            pos: (tsize.0-10, rand::thread_rng().gen_range(1..tsize.1-5)),
            frame: 0.0,   
        } 
    }
    pub fn update(&mut self) {
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
        self.frame += 0.5;
        self.pos = (self.pos.0 - 1, self.pos.1);
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
}


pub fn toasters() { 
    let mut stdout = stdout();

    let mut toasters: Vec<Toaster> = vec![];
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Clear(Purge)).unwrap();
    stdout.execute(Hide).unwrap();
    enable_raw_mode().unwrap();

    loop {
        stdout.execute(Clear(Purge)).unwrap();
        for i in 0..toasters.len() {
            toasters[i].update();
            if toasters[i].pos.0 <= 1 {
                toasters.remove(i);
            }
        }
        if rand::thread_rng().gen_range(0..16) == 1 {
            toasters.push(Toaster::new());
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
