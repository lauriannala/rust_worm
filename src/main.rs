extern crate sdl2;

mod config;
mod worm;
mod apple;

use apple::Apple;
use config::{HEIGHT, WIDTH, WINDOW_MULTIPLIER, WORM_INIT_LENGTH, WORM_INIT_X, WORM_INIT_Y};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use worm::Worm;
use std::convert::TryInto;
use std::time::Duration;
use rand::Rng;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut rng = rand::thread_rng();

    let window = video_subsystem
        .window("rust_worm", WIDTH * WINDOW_MULTIPLIER, HEIGHT * WINDOW_MULTIPLIER)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut worm = Worm::new(WORM_INIT_X, WORM_INIT_Y, WORM_INIT_LENGTH);
    let mut apple = Apple::new(0, 0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    worm.set_direction(worm::MoveDirection::UP);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    worm.set_direction(worm::MoveDirection::DOWN);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    worm.set_direction(worm::MoveDirection::RIGHT);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    worm.set_direction(worm::MoveDirection::LEFT);
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if worm.is_set(&x, &y) && apple.is_set(x, y) {
                    worm.grow();
                    apple.reset(
                        rng.gen_range(0..(WIDTH - 1)),
                        rng.gen_range(0..(HEIGHT - 1))
                    );
                }
                if !worm.is_set(&x, &y) && !apple.is_set(x, y) {
                    continue;
                }

                let render = Rect::new(
                    (x * WINDOW_MULTIPLIER).try_into().unwrap(),
                    (y * WINDOW_MULTIPLIER).try_into().unwrap(),
                    WINDOW_MULTIPLIER,
                    WINDOW_MULTIPLIER);
                let _ = canvas.fill_rect(render);
            }
        }
        canvas.present();
        worm.move_next(WIDTH, HEIGHT);
    }

    Ok(())
}
