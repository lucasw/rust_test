mod utility;

use byteorder::{BigEndian, WriteBytesExt};
use pixels::{Pixels, SurfaceTexture};
use std::{thread, time};
use utility::{create_window, get_data, get_filename, make_plot, Image};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

fn draw(image: &Image, screen: &mut [u8]) {
    for (count, mut pix) in screen.chunks_exact_mut(4).enumerate() {
        let val = image.buffer[count];
        // pix.copy_from_slice(&val);
        pix.write_u32::<BigEndian>(val).unwrap();
    }
}

fn main() {
    let width: usize = 800;
    let height: usize = 600;
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Pixels Render", &event_loop, width, height);
    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let mut pixels = Pixels::new(width as u32, height as u32, surface_texture).unwrap();
    let mut image = Image {
        buffer: vec![0; width * height],
        width,
        height,
    };
    let filename = get_filename();
    let data = get_data(&filename);
    let mut x_scale = 8.0;
    let mut y_scale = 50.0;
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    make_plot(&data, &mut image, x_scale, y_scale, x_offset, y_offset);
    let screen = pixels.get_frame();
    draw(&image, screen);

    let mut replot = false;
    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            let mut screen = pixels.get_frame();
            // if replot {
            {
                // TODO(lucasw) faster to just draw directly into the screen data,
                // avoid duplication here?  This runs pretty slow when
                // called every update
                let start = time::Instant::now();
                // this takes near 1-2 ms (--release)
                make_plot(&data, &mut image, x_scale, y_scale, x_offset, y_offset);
                let dur1 = start.elapsed();
                let start = time::Instant::now();
                // about 1-2 ms
                draw(&image, screen);
                let dur2 = start.elapsed();
                println!("draw calls {:?} {:?}", dur1, dur2);
                replot = false;
            }

            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            thread::sleep(time::Duration::from_millis(33));
        }
        if input.update(&event) {
            window.request_redraw();
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window- if this doesn't happen a resize will crash the program
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
                // println!("new width {} height {}", size.width, size.height);
            }

            // app specific- move to function
            if input.key_held(VirtualKeyCode::Left) || input.key_held(VirtualKeyCode::A) {
                replot = true;
                x_offset -= 10.0;
            }
            if input.key_held(VirtualKeyCode::Right) || input.key_held(VirtualKeyCode::D) {
                replot = true;
                x_offset += 11.0;
            }

            if input.key_held(VirtualKeyCode::Up) || input.key_held(VirtualKeyCode::W) {
                replot = true;
                y_offset += 10.0;
            }
            if input.key_held(VirtualKeyCode::Down) || input.key_held(VirtualKeyCode::S) {
                replot = true;
                y_offset -= 11.0;
            }

            if input.key_held(VirtualKeyCode::F) {
                replot = true;
                x_scale *= 1.1;
                y_scale *= 1.1;
            }
            if input.key_held(VirtualKeyCode::G) {
                replot = true;
                x_scale *= 0.95;
                y_scale *= 0.95;
            }

            window.request_redraw();
        }
    });
}
