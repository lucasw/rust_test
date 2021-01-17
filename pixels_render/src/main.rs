use byteorder::{WriteBytesExt, BigEndian};
use pixels::{Pixels, SurfaceTexture};
use std::{fmt, thread, time};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

// TODO(lucasw) consider nalgrebra in the future

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {:0.3}, {:0.3} }}", self.x, self.y)
    }
}

impl Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    fn plus(&self, p1: &Point) -> Point {
        Point {
            x: self.x + p1.x,
            y: self.y + p1.y,
        }
    }
}

// TODO(lucasw) later lines are indices into a Point vector?
#[derive(Clone, Copy, Debug)]
struct Line {
    p0: Point,
    p1: Point,
    // TODO(lucasw) store) color in a different struct that contains a Line
    color: u32,
}

impl Line {
    fn new(p0: Point, p1: Point) -> Self {
        Self {
            p0,
            p1,
            color: 0xffffff00,
        }
    }

    fn reverse(&self) -> Line {
        Line::new(self.p1, self.p0)
    }

    fn delta(&self) -> Point {
        Point {
            x: self.p1.x - self.p0.x,
            y: self.p1.y - self.p0.y,
        }
    }

    fn scale(&self, scale: f32) -> Line {
        let delta = self.delta();
        let new_end = Point {
            x: scale * delta.x,
            y: scale * delta.y,
        };
        let mut rv = Line::new(self.p0, self.p0.plus(&new_end));
        rv.color = self.color;
        rv
    }

    fn dot(&self, line: &Line) -> f32 {
        let d0 = self.delta();
        let d1 = line.delta();
        d0.x * d1.x + d0.y * d1.y
    }

    // TODO(lucasw) could cache this
    fn len(&self) -> f32 {
        self.dot(self).sqrt()
    }

    // make a Line of length 1 out of self
    fn unit(&self) -> Line {
        self.scale(1.0 / self.len())
    }

    fn project(&self, unit_ray: &Line) -> Line {
        let dist_to_corner = self.dot(unit_ray);
        let base_ray = unit_ray.scale(dist_to_corner);
        base_ray
    }

    fn project_not_unit_ray(&self, not_unit_ray: &Line) -> Line {
        let unit_ray = not_unit_ray.unit();
        let dist_to_corner = self.dot(&unit_ray);
        let base_ray = unit_ray.scale(dist_to_corner);
        base_ray
    }

    fn project2(&self, unit_ray: &Line) -> (f32, Line, Line) {
        let dist_to_corner = self.dot(unit_ray);
        let base_ray = unit_ray.scale(dist_to_corner);
        let right_angle_ray = Line::new(base_ray.p1, self.p1);
        (dist_to_corner, base_ray, right_angle_ray)
    }

    // TODO(lucasw) return Option
    fn find_intersection(unit_ray: &Line, line: &Line) -> (bool, f32, Line) {
        let (d0, l0, right_angle_ray0) = Line::new(unit_ray.p0, line.p0).project2(unit_ray);
        let (d1, l1, right_angle_ray1) = Line::new(unit_ray.p0, line.p1).project2(unit_ray);
        // make sure one end of the line is in the same direction as the unit_ray,
        // and that the unit ray is between the rays to either line end
        let maybe_intersection = (l0.dot(unit_ray) > 0.0 || l1.dot(unit_ray) > 0.0) &&
            right_angle_ray0.dot(&right_angle_ray1) < 0.0;

        let r_dist0 = right_angle_ray0.len();
        let r_dist1 = right_angle_ray1.len();
        let intersection_dist;
        intersection_dist = d0 + (d1 - d0) * r_dist0 / (r_dist0 + r_dist1);
        /*
        if d1 > d0 {
            intersection_dist = d0 + (d1 - d0) * r_dist0 / (r_dist0 + r_dist1);
        } else {
            intersection_dist = d1 + (d0 - d1) * r_dist1 / (r_dist0 + r_dist1);
        }
        */
        let is_intersection = maybe_intersection && intersection_dist > 0.0;

        let mut intersection = unit_ray.scale(intersection_dist);
        intersection.color = 0x44ff8800;
        (
            is_intersection,
            intersection_dist,
            intersection,
            /*
               right_angle_ray0,
               right_angle_ray1,
               */
        )
    }
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

    fn set(&mut self, position: &Point, angle: f32) {
        // let angle = -angle;
        self.c00 = angle.cos();
        self.c01 = angle.sin();
        self.c10 = -angle.sin();
        self.c11 = angle.cos();
        self.c20 = -position.x;
        self.c21 = -position.y;
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

// TODO(lucasw) later make an entity object that contains a vector of lines
// and a position and angle, so it can move and rotate about its center
// within a scene
struct PosAngle {
    position: Point,
    // store rotation matrix instead, compute out angle when needed (which is much less often
    // than the matrix).
    angle: f32,
}

impl PosAngle {
    fn default() -> Self {
        Self {
            position: Point::default(),
            angle: 0.0,
        }
    }
}

struct View {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    position: Point,
    angle: f32,
    rot2d: Rot2D,
}

impl View {
    fn new(width: usize, height: usize, position: Point) -> Self {
        let mut rot2d = Rot2D::new();
        let angle = 0.0;
        rot2d.set(&position, angle);
        Self {
            buffer: vec![0; width * height],
            width,
            height,
            position,
            angle,
            rot2d,
        }
    }

    // TODO(lucasw) combine x and y move into one method
    fn move_xya(&mut self, dxya: &PosAngle) {
        let mut rot2d = Rot2D::new();
        rot2d.set(&Point::default(), self.angle);
        let offset = rot2d.project(&dxya.position);
        // TODO(lucasw) Point add ops overloading
        self.position.x += offset.x;
        self.position.y += offset.y;
        self.angle += dxya.angle;
    }

    // use screen coordinates here
    fn draw_point_screen(&mut self, point: &Point, color: &u32) {
        if point.x < 0.0 { return; }
        if point.y < 0.0 { return; }

        let x = point.x as usize;
        let y = point.y as usize;
        if x >= self.width { return; }
        if y >= self.height { return; }
        let ind = (y * self.width + x) as usize;
        self.buffer[ind] = *color;
    }

    // incoming line is map coordinates
    fn draw_line(&mut self, line: &Line) {
        let p0 = self.rot2d.project(&line.p0);
        let p1 = self.rot2d.project(&line.p1);

        let width = self.width as f32;
        let height = self.height as f32;
        // convert to screen coordinates
        let x0 = p0.x as f32 + width / 2.0;
        let x1 = p1.x as f32 + width / 2.0;
        let y0 = -p0.y as f32 + height / 2.0;
        let y1 = -p1.y as f32 + height / 2.0;

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
                self.draw_point_screen(&pt, &line.color);
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
                self.draw_point_screen(&pt, &line.color);
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
        self.rot2d.set(&self.position, self.angle);
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
    player: PosAngle,
}

impl Scene {
    fn new(width: usize, height: usize, position: Point) -> Self {
        Self {
            view: View::new(width, height, position),
            lines: Scene::new_lines(),
            player: PosAngle { position: Point::default(), angle: std::f32::consts::FRAC_PI_2 },
        }
    }

    fn new_lines() -> Vec<Line> {
        let mut lines = Vec::new();

        let color = 0xaaaaaa00;
        let sz = 50.0;
        lines.push(Line { p0: Point { x: -sz, y: sz }, p1: Point { x: sz, y: sz }, color });
        lines.push(Line { p0: Point { x: sz, y: sz }, p1: Point { x: sz, y: -sz }, color });
        lines.push(Line { p0: Point { x: sz, y: -sz }, p1: Point { x: -sz, y: -sz }, color });
        lines.push(Line { p0: Point { x: -sz, y: -sz }, p1: Point { x: -sz, y: sz }, color });

        lines
    }

    fn player_move(&mut self, dxya: &PosAngle) {
        // TODO(lucasw) need add ops, also matrix2d mult here
        let angle = self.player.angle;
        let player_y = Point { x: angle.cos(), y: angle.sin() };
        let player_x = Point { x: angle.sin(), y: -angle.cos() };

        let x = dxya.position.x;
        let y = -dxya.position.y;
        self.player.position.x += x * player_x.x + y * player_y.x;
        self.player.position.y += x * player_x.y + y * player_y.y;
        self.player.angle += dxya.angle;
    }

    fn update(&mut self) {
        // follow the player with the vew
        if true {
            let dx = self.player.position.x - self.view.position.x;
            let dy = self.player.position.y - self.view.position.y;
            let da = self.player.angle + self.view.angle - std::f32::consts::FRAC_PI_2;
            self.view.position.x += dx *0.1;
            self.view.position.y += dy *0.1;
            self.view.angle -= da *0.1;
            /*
            self.view.move_xya(&PosAngle{
                position: Point {x: dx * 0.1, y: dy * 0.1},
                angle: 0.0,  // da * 0.05,
            });
            */
        }

        self.view.update();
    }

    fn draw_player(&mut self) {
        // draw the player as a line pointing in their view direction
        let pos = self.player.position;
        let angle = self.player.angle;
        let len = 10.0;
        let player_dir = Line {
            p0: pos,
            p1: Point {
                x: pos.x + len * angle.cos(),
                y: pos.y + len * angle.sin(),
            },
            color: 0x0022ff32,
        };

        self.view.draw_line(&player_dir);

        // let i = 0;
        for i in -10..10
        {
            // TODO(lucasw) instead of regenerating these every loop, store in fixed
            // perspective and then rotate them as needed into the scene.
            let angle = angle + i as f32 * 0.08;
            let len = 100.0;
            let ray = Line {
                p0: pos,
                p1: Point {
                    x: pos.x + 1.0 * angle.cos(),
                    y: pos.y + 1.0 * angle.sin(),
                },
                color: 0x22883200,
            };

            let mut did_intersect = false;
            let mut min_intersection = ray.clone();
            let mut min_dist = 0.0;
            for line in self.lines.iter() {
                let (rv, intersection_dist, intersection) = Line::find_intersection(&ray, line);
                // self.view.draw_line(&r1);
                if rv && (!did_intersect || intersection_dist < min_dist) {
                    did_intersect = true;
                    min_dist = intersection_dist;
                    min_intersection = intersection;
                }
            }
            if did_intersect {
                // println!("intersection {:?} with {:?}", ray, line);
                self.view.draw_line(&min_intersection);
            } else {
                self.view.draw_line(&ray.scale(len));
            }
        }
    }

    fn draw(&mut self) {
        self.view.draw_background();
        for line in self.lines.iter_mut() {
            self.view.draw_line(&line);
        }
        self.draw_player();
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
    let mut scene = Scene::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, Point::default());

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            scene.update();
            scene.draw();
            scene.view.render(pixels.get_frame(), frame_count);
            frame_count += 1;

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

        let mut do_move = false;
        let mut dxya = PosAngle::default();

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

            if input.key_held(VirtualKeyCode::Left) || input.key_held(VirtualKeyCode::A) {
                do_move = true;
                dxya.position.x -= 1.1;
            }
            if input.key_held(VirtualKeyCode::Right) || input.key_held(VirtualKeyCode::D) {
                do_move = true;
                dxya.position.x += 1.1;
            }

            if input.key_held(VirtualKeyCode::Up) || input.key_held(VirtualKeyCode::W) {
                do_move = true;
                dxya.position.y -= 2.0;
            }
            if input.key_held(VirtualKeyCode::Down) || input.key_held(VirtualKeyCode::S) {
                do_move = true;
                dxya.position.y += 1.1;
            }

            if input.key_held(VirtualKeyCode::Q) {
                do_move = true;
                dxya.angle += 0.04;
            }
            if input.key_held(VirtualKeyCode::E) {
                do_move = true;
                dxya.angle -= 0.04;
            }

            if input.key_held(VirtualKeyCode::J) {
                scene.view.angle -= 0.04;
            }
            if input.key_held(VirtualKeyCode::K) {
                scene.view.angle += 0.04;
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

        if do_move {
            scene.player_move(&dxya);
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

