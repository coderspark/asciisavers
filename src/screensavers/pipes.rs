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

struct Pipe {
    pub pos: (i32, i32),
    vel: (i32, i32),
    colouridx: usize,
    ptype: usize,
}
impl Pipe {
    pub fn new(types: &Vec<usize>) -> Pipe {
        let tsize = size().unwrap(); 
        let vels = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let vel = vels[rand::thread_rng().gen_range(0..4)];
        
        Pipe {
            pos: (rand::thread_rng().gen_range(2..tsize.0) as i32, rand::thread_rng().gen_range(2..tsize.1) as i32),
            vel,
            colouridx: rand::thread_rng().gen_range(0..7),
            ptype: types[rand::thread_rng().gen_range(0..types.len())]
        }
    }

    pub fn update(&mut self) {
        self.pos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);
        let charset = [
            "  ┓┛  ┏┗┗┛  ┏┓  ━┃T",        // T H I C C
            "  ╮╯  ╭╰╰╯  ╭╮  ─│T",        // R O U N D
            "  ┐┘  ┌└└┘  ┌┐  ─│T",        // n o r m a l
            "  ╗╝  ╔╚╚╝  ╔╗  ═║T",        // D O U B L E
            "  ++  ++++  ++  -|T",        // ASCII
            "  \\//  //\\\\//  //\\  -|T",// OTHER ASCII
            "  ..  ....  ..  ..T",        // ...
            "  oo  oooo  oo  ..T",        // o.o
            "  \\//  //\\\\//  //\\  |-T",// TRAIN TRACKS
            "  █▀  █▀▀▀  ██  ▀█T",           // b o x
        ];
        let chars = charset[self.ptype].chars().collect::<Vec<char>>();
        // let colours = ["\x1b[38;2;255;61;61m","\x1b[38;2;255;220;120m","\x1b[38;2;255;255;61m","\x1b[38;2;120;255;120m","\x1b[38;2;120;255;255m","\x1b[38;2;120;120;255m","\x1b[38;2;255;120;255m"];
        let colours = ["\x1b[30m","\x1b[31m","\x1b[32m","\x1b[33m","\x1b[34m","\x1b[35m","\x1b[36m","\x1b[37m"];
        if rand::thread_rng().gen_range(0..8) == 0 {
            let vels = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let mut turn = rand::thread_rng().gen_range(0..4);
            let cidx = vels.iter().position(| &r | r == self.vel).unwrap();
            while turn == cidx || (vels[turn].0 != 0 && self.vel.0 != 0) || (vels[turn].1 != 0 && self.vel.1 != 0) {
                turn = rand::thread_rng().gen_range(0..4);
            } 
            if rand::thread_rng().gen_range(0..100) == 0 {
                print!("{}\x1b[{};{}H{}", colours[self.colouridx], self.pos.1, self.pos.0, chars[18]);
            } 
            else {
                print!("{}\x1b[{};{}H{}", colours[self.colouridx], self.pos.1, self.pos.0, chars[turn + cidx*4]);
            }
            self.vel = vels[turn];
        }
        else {
            if self.vel.0 != 0 {
                print!("{}\x1b[{};{}H{}", colours[self.colouridx], self.pos.1, self.pos.0, chars[16]);
            }
            else {
                print!("{}\x1b[{};{}H{}", colours[self.colouridx], self.pos.1, self.pos.0, chars[17]);
            }
        }
    }
}

pub fn pipes(types: Vec<usize>, amount: u32, delay: u64) {
    // terminal size
    let tsize = size().unwrap();

    
    // STDOUT WOO
    let mut stdout = stdout();
    let mut pipes: Vec<Pipe> = vec![];
    for _ in 0..amount {
        pipes.push(Pipe::new(&types));
    }
    // boiler plate
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Clear(Purge)).unwrap();
    stdout.execute(Hide).unwrap();
    enable_raw_mode().unwrap();
    loop {
        for i in 0..pipes.len() {
            pipes[i].update();
            if pipes[i].pos.0 == 1 || pipes[i].pos.0 as u16 == tsize.0 || pipes[i].pos.1 == 1 || pipes[i].pos.1 as u16 == tsize.1 {
                pipes.remove(i);
                pipes.push(Pipe::new(&types));
            }
        }
        stdout.flush().unwrap();
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
