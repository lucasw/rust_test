mod utility;

use byteorder::{BigEndian, WriteBytesExt};
use pixels::{Pixels, SurfaceTexture};
use std::{thread, time};
use utility::{create_window, get_filename, make_plot, Image};
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
    make_plot(&mut image, &filename, 10.0, 50.0);
    let mut screen = pixels.get_frame();
    draw(&image, screen);

    // TODO(lucasw) resizing crashes, probably forgot to copy the right code into this loop

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            let mut screen = pixels.get_frame();
            if false {
                draw(&image, screen);
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
    });
}
