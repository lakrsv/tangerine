extern crate sdl2;

use rust_embed::{EmbeddedFile, RustEmbed};
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;

pub fn run(image: EmbeddedFile) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("Enjoy your Tangerine", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(&image.data)?;

    canvas.copy(&texture, None, None)?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let tangerine = Asset::get("tangerine.jpg").unwrap();
    run(tangerine).unwrap();
    Ok(())
}

#[derive(RustEmbed)]
#[folder = "images/"]
struct Asset;
