use byteorder::{WriteBytesExt, BigEndian};
use pixels::{Error, Pixels, SurfaceTexture};
use std::{cmp, thread, time};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

// TODO(lucasw) consider nalgrebra in the future

struct Point {
    x: f32,
    y: f32,
}

// TODO(lucasw) later lines are indices into a Point vector?
struct Line {
    p0: Point,
    p1: Point,
    color: u32,
}

#[derive(Debug)]
struct Rot2D {
    c00: f32,
    c01: f32,
    c10: f32,
    c11: f32,
    c20: f32,
    c21: f32,
}

impl Rot2D {
    fn new() -> Self {
        Self {
            c00: 0.0,
            c01: 0.0,
            c10: 0.0,
            c11: 0.0,
            c20: 0.0,
            c21: 0.0,
        }
    }

    fn set(&mut self, x: f32, y: f32, angle: f32) {
        self.c00 = angle.cos();
        self.c01 = angle.sin();
        self.c10 = -angle.sin();
        self.c11 = angle.cos();
        self.c20 = x;
        self.c21 = y;
    }

    fn project(&self, pt: &Point) -> Point {
        /*
        let x = pt.x as f32;
        let y = pt.y as f32;

        Point {
            x: (self.c00 * x + self.c10 * y + self.c20) as i32,
            y: (self.c01 * x + self.c11 * y + self.c21) as i32,
        }
        */
        let x = pt.x + self.c20;
        let y = pt.y + self.c21;

        Point {
            x: (self.c00 * x + self.c10 * y),
            y: (self.c01 * x + self.c11 * y),
        }

    }
}

struct View {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    x: f32,
    y: f32,
    angle: f32,
    rot2d: Rot2D,
}

impl View {
    fn new(width: usize, height: usize, x: f32, y: f32) -> Self {
        let mut rot2d = Rot2D::new();
        let angle = 0.0;
        rot2d.set(x, y, angle);
        Self {
            buffer: vec![0; width * height],
            width,
            height,
            x,
            y,
            angle,
            rot2d,
        }
    }

    // TODO(lucasw) combine x and y move into one method
    fn move_x(&mut self, dx: f32) {
        let mut rot2d = Rot2D::new();
        rot2d.set(0.0, 0.0, -self.angle);
        let offset = rot2d.project(&Point { x: -dx, y: 0.0 });
        // TODO(lucasw) Point add ops overloading
        self.x += offset.x;
        self.y += offset.y;
    }

    fn move_y(&mut self, dy: f32) {
        let mut rot2d = Rot2D::new();
        rot2d.set(0.0, 0.0, -self.angle);
        let offset = rot2d.project(&Point { x: 0.0, y: -dy });
        // TODO(lucasw) Point add ops overloading
        self.x += offset.x;
        self.y += offset.y;
    }

    fn draw_point(&mut self, point: &Point, color: &u32) {
        if point.x < 0.0 { return; }
        if point.y < 0.0 { return; }

        let x = point.x as usize;
        let y = point.y as usize;
        if x >= self.width { return; }
        if y >= self.height { return; }
        let ind = (y * self.width + x) as usize;
        self.buffer[ind] = *color;
    }

    fn draw_line(&mut self, line: &Line) {
        let p0 = self.rot2d.project(&line.p0);
        let p1 = self.rot2d.project(&line.p1);

        let width = self.width as f32;
        let height = self.height as f32;
        let x0 = p0.x as f32 + width / 2.0;
        let x1 = p1.x as f32 + width / 2.0;
        let y0 = p0.y as f32 + height / 2.0;
        let y1 = p1.y as f32 + height / 2.0;

        let dx = x1 - x0;
        let dy = y1 - y0;

        // bresenham
        // TODO(lucasw) factor out algorithm into function and swap x and y parameters as needed
        if dx.abs() > dy.abs() {
            let incr = dy as f32 / dx as f32;
            let mut y;

            let xs;
            let xf;
            if x0 < x1 {
                xs = x0 as i32;
                y = y0;
                xf = x1 as i32;
            } else {
                xs = x1 as i32;
                y = y1;
                xf = x0 as i32;
            }

            for x in xs..xf {
                let pt = Point { x: x as f32, y };
                self.draw_point(&pt, &line.color);
                y += incr;
            }
        } else {
            let incr = dx as f32 / dy as f32;
            let mut x;

            let ys;
            let yf;
            if y0 < y1 {
                ys = y0 as i32;
                x = x0;
                yf = y1 as i32;
            } else {
                ys = y1 as i32;
                x = x1;
                yf = y0 as i32;
            }

            for y in ys..yf {
                let pt = Point { x, y: y as f32};
                self.draw_point(&pt, &line.color);
                x += incr;
            }
        }
    }

    fn draw_background(&mut self) {
        for (count, pixel) in self.buffer.iter_mut().enumerate() {
            // let x = count as u32 % self.width;
            let y = (count / self.width) as u32;
            let r = y % 0xff;
            let g = (y / 2) % 0xff;
            let b = (y / 4) % 0xff;
            // let color = [0, r, g, b];
            *pixel = r << 24 | g << 16 | b << 8;
            // *pixel = 0x00ffffff;
        }
    }

    fn update(&mut self) {
        self.rot2d.set(self.x, self.y, self.angle);
        // println!("{} {} {} {:?}", self.x, self.y, self.angle, self.rot2d);
    }

    fn render(&self, screen: &mut [u8], _frame_count: u32) {
        // TODO(lucasw) use a zip here with the buffer contents instead of buffer[count]
        for (count, mut pix) in screen.chunks_exact_mut(4).enumerate() {
            // downsample
            let val = self.buffer[count] & 0xf8f8f8f8;
            // pix.copy_from_slice(&val);
            pix.write_u32::<BigEndian>(val).unwrap();
        }
    }
}

struct Scene {
    view: View,
    lines: Vec<Line>,
}

impl Scene {
    fn new(width: usize, height: usize, x: f32, y: f32) -> Self {
        Self {
            view: View::new(width, height, x, y),
            lines: Scene::new_lines(),
        }
    }

    fn new_lines() -> Vec<Line> {
        let mut lines = Vec::new();

        lines.push(Line { p0: Point { x: -70.0, y: 70.0 }, p1: Point { x: 70.0, y: 70.0 }, color: 0xff000000 });
        lines.push(Line { p0: Point { x: 70.0, y: 70.0 }, p1: Point { x: 70.0, y: -70.0 }, color: 0x00ff0000 });
        lines.push(Line { p0: Point { x: 70.0, y: -70.0 }, p1: Point { x: -70.0, y: -70.0 }, color: 0x0000ff00});
        lines.push(Line { p0: Point { x: -70.0, y: -70.0 }, p1: Point { x: -70.0, y: 70.0 }, color: 0x0000ff00});

        lines
    }

    fn update(&mut self) {
        self.view.update();
    }

    fn draw(&mut self) {
        self.view.draw_background();
        for line in self.lines.iter_mut() {
            self.view.draw_line(&line);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Pixels Render", &event_loop);
    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap();

    let mut paused = false;
    let mut frame_count: u32 = 0;
    let mut scene = Scene::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, 0.0, 0.0);
    scene.view.draw_background();

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            scene.update();
            scene.draw();
            scene.view.render(pixels.get_frame(), frame_count);

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

            if input.key_held(VirtualKeyCode::Left) {
                scene.view.move_x(-2.0);
            }
            if input.key_held(VirtualKeyCode::Right) {
                scene.view.move_x(2.0);
            }

            if input.key_held(VirtualKeyCode::Up) {
                scene.view.move_y(-2.0);
            }
            if input.key_held(VirtualKeyCode::Down) {
                scene.view.move_y(2.0);
            }

            if input.key_held(VirtualKeyCode::Q) {
                scene.view.angle += 0.04;
            }
            if input.key_held(VirtualKeyCode::E) {
                scene.view.angle -= 0.04;
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

