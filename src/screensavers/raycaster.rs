use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType::Purge, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn getraycastangle(dir: f32, idx: u16, tsize: (u16, u16)) -> f32 {
    dir - 45.0 / 2.0 + idx as f32 * (90.0 / (tsize.0 as f32 - 1.0)) // Basic grade 9 math
}

fn raycast(map: Vec<Vec<u32>>, pos: (usize, usize), angle: f32) -> (usize, f32, f32) {
    let motion = (
         angle.to_radians().sin() / 100.0,
        -angle.to_radians().cos() / 100.0,
    );
    let mut currentpos = (pos.0 as f32, pos.1 as f32);
    let mut dist: f32 = 0.0;
    let mut side = 2.0;

    while map[currentpos.1.floor() as usize][currentpos.0.floor() as usize] == 0 {
        if motion.0.abs() < motion.1.abs() {
            side = 1.0;
        } else {
            side = 1.7;
        }
        currentpos = (currentpos.0 + motion.0, currentpos.1 + motion.1);
        dist += 0.01;
    }

    (
        map[currentpos.1.floor() as usize][currentpos.0.floor() as usize]
            .try_into()
            .unwrap(),
        dist,
        side,
    )
}

fn fulldraw(map: &Vec<Vec<u32>>, playerpos: (usize, usize), playerdir: f32, tsize: (u16, u16)) {
    let mut stdout = stdout(); 

    let colours = [
        (000.0, 000.0, 000.0),
        (119.0, 7.0, 5.0),
    ];

    let heightthreashhold = tsize.1 as f32 * 4.0;
    let width = 1;

    let mut angles = vec![];
    for i in 0..=tsize.0 / width {
        angles.push(getraycastangle(playerdir, i, tsize));
    }
    let mut rays = vec![];
    for angle in angles {
        let ray = raycast(map.clone(), playerpos, angle);
        rays.push(ray);
    }

    let mut horizontal = 1;
    for ray in rays {
        for vert in 0..=tsize.1 as i32 {
            if vert >= tsize.1 as i32 / 2 - (heightthreashhold / ray.1) as i32 / 2 && vert <= tsize.1 as i32 / 2 + (heightthreashhold / ray.1) as i32 / 2 {
                print!(
                    "\x1b[{};{horizontal}H\x1b[38;2;{};{};{}m{}",
                    vert,
                    (255.0 / ray.2 * colours[ray.0].0 / 255.0).floor() as i32,
                    (255.0 / ray.2 * colours[ray.0].1 / 255.0).floor() as i32,
                    (255.0 / ray.2 * colours[ray.0].2 / 255.0).floor() as i32,
                    "█".repeat(width as usize)
                );
            }
            else {
                if vert <= tsize.1 as i32 / 2{
                    print!(
                        "\x1b[{};{horizontal}H\x1b[38;2;200;200;200m{}",
                        vert,
                        "█".repeat(width as usize)
                    );
                }
                else {
                    print!(
                        "\x1b[{};{horizontal}H\x1b[38;2;190;131;63m{}",
                        vert,
                        "█".repeat(width as usize)
                    );
                }
            }
        }
        horizontal += width;
    }
    stdout.flush().unwrap();
}

fn enlarge(factor: usize, map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut enlarged_map = Vec::new();

    for row in &map {
        // Enlarge each row horizontally
        let enlarged_row: Vec<u32> = row.iter().flat_map(|&val| vec![val; factor]).collect();

        // Enlarge vertically by repeating the enlarged row `factor` times
        for _ in 0..factor {
            enlarged_map.push(enlarged_row.clone());
        }
    }

    enlarged_map
}

pub fn raycaster() {
    let mut stdout = stdout();
    let tsize = size().unwrap();

    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();
    print!("\x1b[48;2;190;131;63m");
    stdout.execute(Clear(Purge)).unwrap();
    print!("\x1b[0m");
    for i in 0..tsize.1 / 2 {
        print!(
            "\x1b[{};1H\x1b[48;2;200;200;200m{}\x1b[0m",
            i + 1,
            " ".repeat(tsize.0 as usize)
        );
    }
    enable_raw_mode().unwrap();
    let map = [
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
        [
            1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    ];

    let factor = 2;
    let enlarged = enlarge(factor, map.iter().map(|r| r.to_vec()).collect());

    let playerpos = (enlarged[0].len() / 2, enlarged.len() / 2);
    let mut playerdir = 0.0;

    loop {
        fulldraw(&enlarged, playerpos, playerdir, tsize);
        playerdir += 1.0;
        if poll(Duration::from_millis(0)).unwrap() {
            let read = read().unwrap();
            if let Event::Key(_) = read {
                break;
            }
        }
        thread::sleep(Duration::from_millis(70));
    }

    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(Show).unwrap();
    disable_raw_mode().unwrap();
}
