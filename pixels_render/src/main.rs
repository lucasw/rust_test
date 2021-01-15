use byteorder::{WriteBytesExt, BigEndian};
use pixels::{Error, Pixels, SurfaceTexture};
use std::{thread, time};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

struct Point {
    x: u32,
    y: u32,
}

struct Line {
    p0: Point,
    p1: Point,
    color: u32,
}

impl Line {
    fn draw(screen: &mut [u8]) {

    }
}

struct Scene {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Scene {
    fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    fn draw_background(&mut self) {
        for (count, pixel) in self.buffer.iter_mut().enumerate() {
            let x = (count % self.width) as u32;
            let y = (count / self.width) as u32;
            let r = y % 0xff;
            let g = (y / 2) % 0xff;
            let b = (y / 4) % 0xff;
            // let color = [0, r, g, b];
            *pixel = r << 24;  // | g << 16 | b << 8;
            // *pixel = 0x00ffffff;
        }
    }

    fn render(&self, screen: &mut [u8], frame_count: u32) {
        // TODO(lucasw) use a zip here with the buffer contents instead of buffer[count]
        for (count, mut pix) in screen.chunks_exact_mut(4).enumerate() {
            // downsample
            let val = self.buffer[count] & 0xf8f8f8f8;
            // pix.copy_from_slice(&val);
            pix.write_u32::<BigEndian>(val).unwrap();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    // TODO(lucasw) see if berylium sdl is better with keyboard input
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Pixels Render", &event_loop);
    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap();

    let mut paused = false;
    let mut frame_count: u32 = 0;
    let mut right_key = false;
    let mut scene = Scene::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
    scene.draw_background();

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            scene.render(pixels.get_frame(), frame_count);

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

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }
            if input.key_pressed(VirtualKeyCode::Left) {
                right_key = true;
            }
            if input.key_released(VirtualKeyCode::Left) {
                right_key = false;
            }
            if input.key_held(VirtualKeyCode::Right) {
                frame_count += 1;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            window.request_redraw();
        }

        if right_key {
            frame_count -= 1;
        }
    });
}


/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

