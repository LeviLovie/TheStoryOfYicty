#![allow(non_snake_case)]

use minifb::{Key, Window, WindowOptions};
use std::{time::{Duration, Instant}, thread};
use std::sync::Mutex;
use rayon::prelude::*;

const WIDTH: usize = 240;
const HEIGHT: usize = 160;
const SCALE: usize = 5;
const FPS: u32 = 24;
const TITLE: &str = "The Story of Yicty";

const TILE_SIZE: usize = 8;
const TEST_TILE: [u32; TILE_SIZE * TILE_SIZE] = [
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_FF_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
    0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_FF_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF, 0xFF_00_00_FF,
];

fn update(buffer: &mut [u32]) {
    draw_rectangle(buffer, 0, 0, HEIGHT, WIDTH, 0xFF18_18_18);
    draw_sprite(buffer, &TEST_TILE, TILE_SIZE, TILE_SIZE, 155, 10)
}

fn main() {
    let mut window = create_window(HEIGHT * SCALE, WIDTH * SCALE, TITLE);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let max_update_time = Duration::from_millis(1000) / FPS;
    let max_update_time_as_micros = max_update_time.as_micros();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time_start = Instant::now();
        update(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        let elapsed = time_start.elapsed();
        if elapsed < max_update_time {
            let sleep_time = max_update_time - elapsed;
            thread::sleep(sleep_time);
            window.set_title(format!("{} - {:>6}|{:<6}us", TITLE, elapsed.as_micros(), max_update_time_as_micros).as_str());
        } else {
            println!("\x1b[31mFrame took too long\x1b[0m: \x1b[33m{:>6}\x1b[0m|\x1b[33m{:<6}\x1b[0mus", elapsed.as_micros(), max_update_time_as_micros);
        }
    }
}

fn draw_rectangle(buffer: &mut [u32], y: usize, x: usize, size_y: usize, size_x: usize, color: u32) {
    let buffer_mutex = Mutex::new(buffer);

    (y..size_y + SCALE).into_par_iter().for_each(|y| {
        let mut buffer_guard = buffer_mutex.lock().unwrap();
        for x in x..size_x + SCALE {
            if y >= HEIGHT || x >= WIDTH {
                continue;
            }
            let buffer_index = y * WIDTH + x;
            buffer_guard[buffer_index] = color;
        }
    });
}

fn draw_sprite(buffer: &mut [u32], sprite: &[u32], sprite_size_y: usize, sprite_size_x: usize, pos_y: usize, pos_x: usize) {
    let buffer_width = WIDTH;
    let buffer_mutex = Mutex::new(buffer);

    (0..sprite_size_y).into_par_iter().for_each(|y| {
        let buffer_row_start = (y + pos_y) * buffer_width;
        let sprite_row_start = y * sprite_size_x;

        for x in 0..sprite_size_x {
            if y >= HEIGHT || x >= WIDTH {
                continue;
            }
            if x + pos_x >= WIDTH || y + pos_y >= HEIGHT {
                break;
            }

            let buffer_index = buffer_row_start + (x + pos_x);
            let sprite_index = sprite_row_start + x;

            let mut buffer_guard = buffer_mutex.lock().unwrap();
            buffer_guard[buffer_index] = sprite[sprite_index];
        }
    });
}

fn create_window(height: usize, width: usize, title: &str) -> Window {
    return Window::new(
        title,
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    })
}
