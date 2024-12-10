/*
Cool bouncing ball animation
*/

// Import stuff
use rand::Rng;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType::Purge, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

static mut CHARS: u64 = 0;

pub struct Ball {
    pos: (i32, i32),
    vel: (i32, i32),
    colouridx: usize,
}
impl Ball {
    pub fn new() -> Ball {
        let mut vel = (
            rand::thread_rng().gen_range(-1..=1) * 2,
            rand::thread_rng().gen_range(-1..=1),
        );

        // cool while loop
        while vel.0 == 0 || vel.1 == 0 {
            vel = (
                rand::thread_rng().gen_range(-1..=1) * 2,
                rand::thread_rng().gen_range(-1..=1),
            ); // Randomizes until no values are 0
        }
        let tsize = size().unwrap();

        // basic elementry school math
        let pos = (
            rand::thread_rng().gen_range(2..(tsize.0 - 1) / 2) as i32 * 2,
            rand::thread_rng().gen_range(2..tsize.1 - 1) as i32,
        );
        Ball {
            pos,
            vel,
            colouridx: 0,
        }
    }
    pub fn update(&mut self, tsize: (u16, u16), fancy: bool) {
        // big ass colours array
        let colours = [
            "\x1b[38;2;255;0;0m",
            "\x1b[38;2;255;15;0m",
            "\x1b[38;2;255;30;0m",
            "\x1b[38;2;255;45;0m",
            "\x1b[38;2;255;61;0m",
            "\x1b[38;2;255;76;0m",
            "\x1b[38;2;255;91;0m",
            "\x1b[38;2;255;107;0m",
            "\x1b[38;2;255;122;0m",
            "\x1b[38;2;255;137;0m",
            "\x1b[38;2;255;153;0m",
            "\x1b[38;2;255;168;0m",
            "\x1b[38;2;255;183;0m",
            "\x1b[38;2;255;198;0m",
            "\x1b[38;2;255;214;0m",
            "\x1b[38;2;255;229;0m",
            "\x1b[38;2;255;244;0m",
            "\x1b[38;2;249;255;0m",
            "\x1b[38;2;234;255;0m",
            "\x1b[38;2;219;255;0m",
            "\x1b[38;2;203;255;0m",
            "\x1b[38;2;188;255;0m",
            "\x1b[38;2;173;255;0m",
            "\x1b[38;2;158;255;0m",
            "\x1b[38;2;142;255;0m",
            "\x1b[38;2;127;255;0m",
            "\x1b[38;2;112;255;0m",
            "\x1b[38;2;96;255;0m",
            "\x1b[38;2;81;255;0m",
            "\x1b[38;2;66;255;0m",
            "\x1b[38;2;51;255;0m",
            "\x1b[38;2;35;255;0m",
            "\x1b[38;2;20;255;0m",
            "\x1b[38;2;5;255;0m",
            "\x1b[38;2;0;255;10m",
            "\x1b[38;2;0;255;25m",
            "\x1b[38;2;0;255;40m",
            "\x1b[38;2;0;255;56m",
            "\x1b[38;2;0;255;71m",
            "\x1b[38;2;0;255;86m",
            "\x1b[38;2;0;255;102m",
            "\x1b[38;2;0;255;117m",
            "\x1b[38;2;0;255;132m",
            "\x1b[38;2;0;255;147m",
            "\x1b[38;2;0;255;163m",
            "\x1b[38;2;0;255;178m",
            "\x1b[38;2;0;255;193m",
            "\x1b[38;2;0;255;209m",
            "\x1b[38;2;0;255;224m",
            "\x1b[38;2;0;255;239m",
            "\x1b[38;2;0;255;255m",
            "\x1b[38;2;0;239;255m",
            "\x1b[38;2;0;224;255m",
            "\x1b[38;2;0;209;255m",
            "\x1b[38;2;0;193;255m",
            "\x1b[38;2;0;178;255m",
            "\x1b[38;2;0;163;255m",
            "\x1b[38;2;0;147;255m",
            "\x1b[38;2;0;132;255m",
            "\x1b[38;2;0;117;255m",
            "\x1b[38;2;0;102;255m",
            "\x1b[38;2;0;86;255m",
            "\x1b[38;2;0;71;255m",
            "\x1b[38;2;0;56;255m",
            "\x1b[38;2;0;40;255m",
            "\x1b[38;2;0;25;255m",
            "\x1b[38;2;0;10;255m",
            "\x1b[38;2;5;0;255m",
            "\x1b[38;2;20;0;255m",
            "\x1b[38;2;35;0;255m",
            "\x1b[38;2;50;0;255m",
            "\x1b[38;2;66;0;255m",
            "\x1b[38;2;81;0;255m",
            "\x1b[38;2;96;0;255m",
            "\x1b[38;2;112;0;255m",
            "\x1b[38;2;127;0;255m",
            "\x1b[38;2;142;0;255m",
            "\x1b[38;2;158;0;255m",
            "\x1b[38;2;173;0;255m",
            "\x1b[38;2;188;0;255m",
            "\x1b[38;2;204;0;255m",
            "\x1b[38;2;219;0;255m",
            "\x1b[38;2;234;0;255m",
            "\x1b[38;2;249;0;255m",
            "\x1b[38;2;255;0;244m",
            "\x1b[38;2;255;0;229m",
            "\x1b[38;2;255;0;214m",
            "\x1b[38;2;255;0;198m",
            "\x1b[38;2;255;0;183m",
            "\x1b[38;2;255;0;168m",
            "\x1b[38;2;255;0;152m",
            "\x1b[38;2;255;0;137m",
            "\x1b[38;2;255;0;122m",
            "\x1b[38;2;255;0;107m",
            "\x1b[38;2;255;0;91m",
            "\x1b[38;2;255;0;76m",
            "\x1b[38;2;255;0;61m",
            "\x1b[38;2;255;0;45m",
            "\x1b[38;2;255;0;30m",
            "\x1b[38;2;255;0;15m",
            "\x1b[38;2;255;0;15m",
        ];

        // move the ball
        self.pos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);
        // draw the ball

        if fancy {
            print!(
                "\x1b[{};{}H{}\x1b[0m",
                self.pos.1, self.pos.0, colours[self.colouridx]
            );
        } else {
            print!(
                "\x1b[{};{}H{}██\x1b[0m",
                self.pos.1, self.pos.0, colours[self.colouridx]
            );
        }
        unsafe {
            CHARS += 1;
        }

        // check if the ball hit an edge
        if self.pos.0 >= tsize.0 as i32 - 2 || self.pos.0 <= 1 {
            self.vel = (-self.vel.0, self.vel.1);
        }
        if self.pos.1 >= tsize.1 as i32 - 1 || self.pos.1 <= 1 {
            self.vel = (self.vel.0, -self.vel.1);
        }
        // cool thing
        self.colouridx += 1;
        self.colouridx %= 101;
    }
}

pub fn ball(delay: u64, fancy: bool, reset: u64) {
    // terminal size
    let tsize = size().unwrap();

    // STDOUT WOO
    let mut stdout = stdout();

    let mut balls: Vec<Ball> = vec![];
    balls.push(Ball::new());
    // boiler plate
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Clear(Purge)).unwrap();
    stdout.execute(Hide).unwrap();
    enable_raw_mode().unwrap();
    loop {
        for i in 0..balls.len() {
            balls[i].update(tsize, fancy);
        }
        if rand::thread_rng().gen_range(0..32) == 0 {
            balls.push(Ball::new());
        }
        unsafe {
            if CHARS >= reset {
                CHARS = 0;
                stdout.execute(Clear(Purge)).unwrap();
                balls.clear();
                balls.push(Ball::new());
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
