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
    f32::consts::PI,
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn getraycastangle(dir: f32, idx: u16, tsize: (u16, u16)) -> f32 {
    dir - 45.0 + idx as f32 * (90.0 / (tsize.0 as f32 - 1.0)) // Basic grade 9 math
}

fn raycast(map: Vec<Vec<u32>>, pos: (usize, usize), angle: f32) -> (usize, f32) {
    let motion = (
        (angle * PI / 180.0).sin() / 100.0,
        -(angle * PI / 180.0).cos() / 100.0,
    );
    let mut currentpos = (pos.0 as f32, pos.1 as f32);
    let mut dist: f32 = 0.0;
    while map[currentpos.1.floor() as usize][currentpos.0.floor() as usize] == 0 {
        currentpos = (currentpos.0 + motion.0, currentpos.1 + motion.1);
        dist += 0.01;
    }

    (
        map[currentpos.1.floor() as usize][currentpos.0.floor() as usize]
            .try_into()
            .unwrap(),
        dist,
    )
}

fn fulldraw(map: &Vec<Vec<u32>>, playerpos: (usize, usize), playerdir: f32, tsize: (u16, u16)) {
    let mut stdout = stdout();

    print!("\x1b[48;2;31;31;31m");
    stdout.execute(Clear(Purge)).unwrap();
    print!("\x1b[0m");
    for i in 0..tsize.1 / 2 {
        print!(
            "\x1b[{};1H\x1b[48;2;61;61;61m{}\x1b[0m",
            i + 1,
            " ".repeat(tsize.0 as usize)
        );
    }

    let colours = [
        (000, 000, 000),
        (255, 061, 061),
        (061, 255, 061),
        (061, 061, 255),
        (255, 255, 255),
        (255, 255, 061),
    ];

    let heightthreashhold = (tsize.1 as f32 * 1.5) as i32;
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
        for vert in 0..=(heightthreashhold / ray.1 as i32) {
            print!(
                "\x1b[{};{horizontal}H\x1b[38;2;{};{};{}m{}",
                tsize.1 as i32 / 2 - ((heightthreashhold / ray.1 as i32) / 2) + vert,
                (255.0 / ray.1 * colours[ray.0].0 as f32 / 255.0) as i32,
                (255.0 / ray.1 * colours[ray.0].1 as f32 / 255.0) as i32,
                (255.0 / ray.1 * colours[ray.0].2 as f32 / 255.0) as i32,
                "█".repeat(width as usize)
            );
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
    enable_raw_mode().unwrap();
    let map = [
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,3,3,3,3,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
        [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,3,3,3,3,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    let factor = 1;
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
