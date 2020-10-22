
extern crate sdl2;
use std::path::Path;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Engine {
    fn play(&self) -> Result<(), String>;
}

pub struct GraphicsEngine {
}

impl GraphicsEngine {
    pub fn new() -> GraphicsEngine {
        GraphicsEngine {
        }
    }
}

impl Engine for GraphicsEngine {    
    fn play(&self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
    
        //let dimension = (1280, 720);
        let dimension = (640, 480);
        let window = video_subsystem.window("hurricane", dimension.0, dimension.1)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
    
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
    
        let mut timer = sdl_context.timer()?;
    
        let mut event_pump = sdl_context.event_pump()?;
    
        // images source,
        // sky : http://clipart-library.com/clipart/pTodkpnGc.htm
        // plane 1 : https://www.pngwave.com/png-clip-art-bzlan
        // plane 2 : https://www.pngwave.com/png-clip-art-wfjsq
        // bullet : https://opengameart.org/content/lasers-and-beams
    
        let texture_creator = canvas.texture_creator();
        let _ = sdl2::image::init(InitFlag::PNG)?;
    
        let png = Path::new("assets/images/sky.png");
        let sky = texture_creator.load_texture(&png)?;
    
        let png = Path::new("assets/images/plane/1.png");
        let plane_1 = texture_creator.load_texture(&png)?;
    
        let png = Path::new("assets/images/plane/2.png");
        let plane_2 = texture_creator.load_texture(&png)?;
    
        let png = Path::new("assets/images/bullet.png");
        let bullet = texture_creator.load_texture(&png)?;
    
        let screen_width : u32 = canvas.viewport().width();
        let screen_height : u32 = canvas.viewport().height();
        let plane_dimension : u32 = 128;
        let bullet_dimension : u32 = plane_dimension / 4;
    
        let mut dest_plane_1 = Rect::new(
            ((screen_width - plane_dimension) / 2) as i32,
            (screen_height - plane_dimension) as i32,
            plane_dimension, plane_dimension);
        let mut dest_plane_2 = Rect::new(
            ((screen_width - plane_dimension) / 2) as i32,
            0, plane_dimension, plane_dimension);
        let mut dest_bullet = Rect::new(
                ((screen_width - plane_dimension) / 2) as i32 - 4,
                (screen_height - plane_dimension) as i32 - 4,
                bullet_dimension, bullet_dimension);
    
        let keystroke_delta = 10;
    
        let sleep_time = 5;
    
        'mainloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit{..} |
                    Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} => {
                        break 'mainloop;
                    }
                    Event::KeyDown { keycode: Option::Some(Keycode::Up), .. } => {
                        if dest_plane_1.y > 0 {
                           dest_plane_1.y -= keystroke_delta;
                        }
                    }
                    Event::KeyDown { keycode: Option::Some(Keycode::Down), .. } => {
                        if dest_plane_1.y < (screen_height - plane_dimension) as i32 {
                           dest_plane_1.y += keystroke_delta;
                        }
                    }
                    Event::KeyDown { keycode: Option::Some(Keycode::Left), .. } => {
                        if dest_plane_1.x > 0 {
                           dest_plane_1.x -= keystroke_delta;
                        }
                    }
                    Event::KeyDown { keycode: Option::Some(Keycode::Right), .. } => {
                        if dest_plane_1.x < (screen_width - plane_dimension) as i32 {
                           dest_plane_1.x += keystroke_delta;
                        }
                    }
                    _ => {}
                }
            }
    
            dest_bullet.x = dest_plane_1.x + (plane_dimension * 3 / 8) as i32;
            dest_bullet.y = dest_plane_1.y - (plane_dimension / 4) as i32;
    
            dest_plane_2.y = ((timer.ticks() / sleep_time) % screen_height) as i32;
    
            // copy the frame to the canvas
            canvas.copy(&sky, None, None)?;
            canvas.copy_ex(&plane_1, None, Some(dest_plane_1), 0.0, None, false, false)?;
            canvas.copy_ex(&bullet, None, Some(dest_bullet), 0.0, None, false, false)?;
            canvas.copy_ex(&plane_2, None, Some(dest_plane_2), 0.0, None, false, true)?;
            canvas.present();
            std::thread::sleep(Duration::from_millis(sleep_time as u64));
        }
    
        canvas.clear();
        Ok(())
    }
}