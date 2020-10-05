extern crate sdl2;

use std::path::Path;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("hurricane", 1280, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    // images source,
    // sky : http://clipart-library.com/clipart/pTodkpnGc.htm
    // plane 1 : https://www.pngwave.com/png-clip-art-bzlan
    // plane 2 : https://www.pngwave.com/png-clip-art-wfjsq

    let texture_creator = canvas.texture_creator();
    let _ = sdl2::image::init(InitFlag::PNG)?;

    let png = Path::new("assets/images/sky.png");
    let sky = texture_creator.load_texture(&png)?;

    let png = Path::new("assets/images/plane/1.png");
    let plane_1 = texture_creator.load_texture(&png)?;

    let png = Path::new("assets/images/plane/2.png");
    let plane_2 = texture_creator.load_texture(&png)?;

    let plane_dimension : u32 = 128;
    let dest_plane_1 = Rect::new(
        ((canvas.viewport().width() - plane_dimension) / 2) as i32,
        (canvas.viewport().height() - plane_dimension) as i32,
        plane_dimension, plane_dimension);
    let dest_plane_2 = Rect::new(
        ((canvas.viewport().width() - plane_dimension) / 2) as i32,
        0, plane_dimension, plane_dimension);

    canvas.copy(&sky, None, None)?;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

        // copy the frame to the canvas
        canvas.copy_ex(&plane_1, None, Some(dest_plane_1), 0.0, None, false, false)?;
        canvas.copy_ex(&plane_2, None, Some(dest_plane_2), 0.0, None, false, true)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(100));
    }

    canvas.clear();
    Ok(())
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
