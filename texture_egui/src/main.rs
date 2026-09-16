use eframe::egui;
use egui_plot::{Plot, PlotImage, PlotPoint};
use std::time::{Duration, Instant};

/*
// TODO(lucasw) make these methods on float array struct
fn get_value_i32(values: &[f32], width: usize, height: usize, pos: [i32; 2]) -> Option<f32> {
    let [x, y] = pos;
    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
        let ind = y as usize * width + x as usize;
        Some(values[ind])
    } else {
        None
    }
}

fn get_value(values: &[f32], width: usize, height: usize, pos: [f32; 2]) -> Option<f32> {
    let [x, y] = pos;
    let x = x.round() as i32;
    let y = y.round() as i32;
    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
        let ind = y as usize * width + x as usize;
        Some(values[ind])
    } else {
        None
    }
}

struct Particle {
    x: f32,
    y: f32,
}
*/

/*
impl Particle {
}
*/

fn position_to_ind(x: f32, y: f32, width: usize, height: usize) -> Option<usize> {
    let x = x.round();
    let y = y.round();
    if x >= 0.0 && x <= (width as f32 - 1.0) && y >= 0.0 && y <= (height as f32 - 1.0) {
        Some(y as usize * width + x as usize)
    } else {
        None
    }
}

#[derive(Clone, Debug)]
struct Angle {
    /// radians
    // angle: f32,
    /// one of these will be +/- 1.0, together they will point
    /// in the direction of angle but define a vector with a length >= 1.0
    dx: f32,
    dy: f32,
    scale: f32,
}

impl Angle {
    fn new(angle: f32) -> Self {
        let mut dx = angle.cos();
        let mut dy = angle.sin();
        let scale = {
            if dx.abs() > dy.abs() {
                1.0 / dx.abs()
            } else {
                1.0 / dy.abs()
            }
        };
        dx *= scale;
        dy *= scale;

        Self {
            // TODO(lucasw) normalize angle to be -π to π
            // angle: angle % (2.0 * std::f32::consts::PI),
            dx,
            dy,
            scale,
        }
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[derive(Debug)]
struct Turret {
    x: f32,
    y: f32,
    angle: Angle,
    reload: usize,
    last_shot: usize,
    /// which enemy soldier to target
    target: usize,
}

/// Turrets fire shells
struct Shell {
    x: f32,
    y: f32,
    angle: Angle,
    speed: f32,
}

impl Shell {
    // TODO(lucasw) this is a sprite
    const DAMAGE_PATTERN: [[u16; 5]; 5] = [
        [0, 120, 140, 120, 0],
        [120, 150, 800, 150, 120],
        [140, 800, 1500, 800, 140],
        [120, 150, 800, 150, 120],
        [0, 120, 140, 120, 0],
    ];
}

/// TODO(lucasw) consider a separate array of positions and of hit points
struct Soldier {
    x: f32,
    y: f32,
    health: u16,
}

#[derive(Default)]
struct Game {
    static_obstacles: Vec<u16>,

    turrets0: Vec<Turret>,

    shells: Vec<Shell>,

    soldiers: Vec<Soldier>,

    /*
    particles0: Vec<Particle>,
    /// particles will move to higher scoring neighbor cells here
    move_gradient0a: Vec<f32>,
    move_gradient0b: Vec<f32>,
    max0: f32,

    particles1: Vec<Particle>,
    move_gradient1a: Vec<f32>,
    move_gradient1b: Vec<f32>,
    max1: f32,
    */
    counter: usize,
}

impl Game {
    // update and draw
    fn update(&mut self, color_image: &mut egui::ColorImage) {
        let [width, height] = color_image.size;

        self.static_obstacles
            .iter()
            .enumerate()
            .for_each(|(ind, obstacle)| {
                // if *obstacle > 0 {
                let color = egui::Color32::from_rgb(
                    (obstacle >> 5).clamp(0, 255) as u8,
                    (obstacle >> 9) as u8,
                    (obstacle >> 10) as u8,
                );
                color_image.pixels[ind] = color;
                // } else {
                //     color_image.pixels[ind] = color_image.pixels[ind].gamma_multiply_u8(200);
                // }
            });

        // how much damage is being done within a grid location
        let mut damage: Vec<u16> = vec![0; width * height];

        self.turrets0
            .iter_mut()
            .enumerate()
            .for_each(|(ind, turret)| {
                if !self.soldiers.is_empty() {
                    // switch to a random target if it is closer
                    {
                        let target0 = turret.target % self.soldiers.len();
                        // TODO(lucasw) fastrand another index
                        let target1 = fastrand::usize(0..self.soldiers.len());
                        // target the one closer to the bottom
                        let dx0 = self.soldiers[target0].x - turret.x;
                        let dy0 = self.soldiers[target0].y - turret.y;
                        let dist0_sq = dx0 * dx0 + dy0 * dy0;
                        let dx1 = self.soldiers[target1].x - turret.x;
                        let dy1 = self.soldiers[target1].y - turret.y;
                        let dist1_sq = dx1 * dx1 + dy1 * dy1;
                        if dist1_sq < dist0_sq {
                            turret.target = target1;
                        } else {
                            turret.target = target0;
                        }
                    }

                    let target = &self.soldiers[turret.target];
                    let dx = target.x - turret.x;
                    let dy = target.y - turret.y;
                    // TODO(lucasw) rotate slowly instead of instantaneous
                    turret.angle = Angle::new(dy.atan2(dx) + (rand::random::<f32>() - 0.5) * 0.005);

                    let elapsed = self.counter - turret.last_shot;
                    if elapsed > turret.reload {
                        // println!("[{}] {ind} shoot", self.counter);
                        let speed = 20.0 + rand::random::<f32>() * 2.0;
                        let offset = speed * rand::random::<f32>() * 0.4;
                        let shell = Shell {
                            x: turret.x + turret.angle.dx * offset,
                            y: turret.y + turret.angle.dy * offset,
                            angle: turret.angle.clone(),
                            speed,
                        };
                        self.shells.push(shell);
                        // turret.angle =
                        //     Angle::new(turret.angle.angle + (rand::random::<f32>() - 0.5) * 0.02);
                        turret.last_shot = self.counter;
                    }
                }
                // TODO(lucasw) else the turrets won, maybe reset or write a message,
                // or spawn more enemies

                let color = egui::Color32::from_rgb(225, ind as u8 * 10, 255);
                for oy in [-1.0, 0.0, 1.0] {
                    for ox in [-1.0, 0.0, 1.0] {
                        if let Some(pixel_ind) =
                            position_to_ind(turret.x + ox, turret.y + oy, width, height)
                        {
                            /*
                            if turret.last_shot == self.counter {
                            println!("[{}] turret {ind} pixel {pixel_ind}", self.counter);
                            }
                            */
                            color_image.pixels[pixel_ind] = color;
                        }
                    }
                }
            });

        let mut obstacles = self.static_obstacles.clone();
        self.soldiers.iter().for_each(|soldier| {
            if let Some(pixel_ind) = position_to_ind(soldier.x, soldier.y, width, height) {
                obstacles[pixel_ind] = soldier.health;
            }
        });

        // faster than retain() since we don't care about preserving shell order
        // self.shells.iter_mut().rev().enumerate().for_each(|(shell_index, shell)| {
        for shell_index in (0..self.shells.len()).rev() {
            let shell = &mut self.shells[shell_index];
            // need to scale the speed to account for dx or dy being normalized
            let num = (shell.speed / shell.angle.scale).floor() as usize;
            // TODO(lucasw) move the fractional part at the end as well
            for i in 0..num {
                let fr = i as f32 / shell.speed;
                let color = egui::Color32::from_rgb((fr * 255.0) as u8, (fr * 255.0) as u8, 0);
                shell.x += shell.angle.dx;
                shell.y += shell.angle.dy;
                if let Some(pixel_ind) = position_to_ind(shell.x, shell.y, width, height) {
                    color_image.pixels[pixel_ind] = color;
                    // see if the shell has hit an obstacle
                    if obstacles[pixel_ind] > 0 {
                        // cause damage to the obstacle and surrounding grid cells
                        // via the damage map
                        // TODO(lucasw) sprite blitting
                        for (iy, row) in Shell::DAMAGE_PATTERN.iter().enumerate() {
                            let oy = iy as f32 - 2.0;
                            for (ix, shell_pattern_value) in row.iter().enumerate() {
                                let ox = ix as f32 - 2.0;
                                if let Some(damage_ind) =
                                    position_to_ind(shell.x + ox, shell.y + oy, width, height)
                                {
                                    damage[damage_ind] += shell_pattern_value;
                                }
                            }
                        }
                        self.shells.swap_remove(shell_index);
                        break;
                    }
                } else {
                    // remove since the shell went off map
                    self.shells.swap_remove(shell_index);
                    break;
                }
            }
        }

        for soldier_index in (0..self.soldiers.len()).rev() {
            if let Some(pixel_ind) = position_to_ind(
                self.soldiers[soldier_index].x,
                self.soldiers[soldier_index].y,
                width,
                height,
            ) {
                let mut health = self.soldiers[soldier_index].health;
                health = health.saturating_sub(damage[pixel_ind]);
                self.soldiers[soldier_index].health = health;

                if health == 0 {
                    self.soldiers.swap_remove(soldier_index);
                }

                // TODO(lucasw) should this be in a separate loop?
                let color = egui::Color32::from_rgb(20, 210, health.clamp(0, 255) as u8);
                color_image.pixels[pixel_ind] = color;
            } else {
                // off map soldiers are removed
                self.soldiers.swap_remove(soldier_index);
            }
        }

        // move the (surviving) soldiers if the path is clear
        // TODO(lucasw) the lowest indexed ones get the initiative
        self.soldiers.iter_mut().for_each(|soldier| {
            let x0 = soldier.x;
            let y0 = soldier.y;
            if let Some(pixel_ind0) = position_to_ind(x0, y0, width, height) {
                let x_left = x0 - 1.0;
                let x_right = x0 + 1.0;
                let y_down = y0 + 1.0;
                // TODO(lucasw) semi randomize choosing left or right
                // don't do anything if can't move left or right or down for now
                let positions = {
                    let mut positions = vec![
                        (x0, y_down),
                        (x_left, y_down),
                        (x_right, y_down),
                        (x_left, y0),
                        (x_right, y0),
                    ];
                    fastrand::shuffle(&mut positions);
                    positions
                };
                for (x, y) in positions.into_iter() {
                    if let Some(pixel_ind1) = position_to_ind(x, y, width, height)
                        && obstacles[pixel_ind1] == 0
                    {
                        soldier.x = x;
                        soldier.y = y;
                        // another solider can move into the empty space this one leaves
                        // in this same update loop
                        obstacles[pixel_ind0] = 0;
                        break;
                    }
                }
            }
        });

        for (pixel_ind, hits) in damage.into_iter().enumerate() {
            self.static_obstacles[pixel_ind] =
                self.static_obstacles[pixel_ind].saturating_sub(hits);

            if hits > 0 {
                let color = egui::Color32::from_rgb(255, hits.clamp(0, 255) as u8, 0);
                color_image.pixels[pixel_ind] = color;
            }
        }

        self.counter += 1;
    }

    fn reset(&mut self, width: usize, height: usize) {
        self.static_obstacles = vec![0; width * height];

        // make barriers in the static obstacles
        for (x0, y0) in [
            (0, 50),
            (50, 140),
            (100, 80),
            (120, 140),
            (140, 50),
            (200, 75),
            (250, 200),
            (400, 90),
            (500, 80),
            (300, 130),
        ] {
            for yi in y0..(y0 + 24) {
                for xi in x0..(x0 + 60) {
                    let ind = yi * width + xi;
                    self.static_obstacles[ind] = 2000;
                }
            }
        }

        self.turrets0.clear();
        let num = 32;
        for i in 0..32 {
            let turret = Turret {
                x: i as f32 * width as f32 / num as f32,
                y: height as f32 - 20.0,
                angle: Angle::new(-std::f32::consts::FRAC_PI_2 + i as f32 * 0.005),
                reload: 4 + (rand::random::<f32>() * 3.0) as usize,
                last_shot: 0,
                target: i * 100,
            };
            println!("{i} {turret:?}");
            self.turrets0.push(turret);
        }

        self.soldiers.clear();
        for i in 0..28000 {
            let soldier = Soldier {
                x: (i % width) as f32,
                y: (i / width) as f32,
                health: 20,
            };
            self.soldiers.push(soldier);
        }
    }

    /*
        fn update(&mut self) {
            let [width, height] = self.color_image.size;

            let color = egui::Color32::from_rgb(0, 0, 0);
            for yi in 0..height {
                for xi in 0..width {
                    let ind = yi * width + xi;
                    self.color_image.pixels[ind] = color;
                }
            }

            // These take almost no time
            move_particles(
                width,
                height,
                &mut self.particles0,
                &mut self.move_gradient0a,
                &mut self.move_gradient1a,
            );
            move_particles(
                width,
                height,
                &mut self.particles1,
                &mut self.move_gradient1a,
                &mut self.move_gradient0a,
            );

            fn smooth_gradient(width: usize, height: usize, ga: &[f32], gb: &mut [f32]) {
                let retain = 0.99;
                for ind in 0..ga.len() {
                    let px = ind as i32 % width as i32;
                    let py = ind as i32 / width as i32;
                    // let pos = [px, py];
                    let positions = vec![
                        [px - 1, py - 1],
                        [px - 1, py + 1],
                        [px + 1, py + 1],
                        [px + 1, py - 1],
                        [px, py - 1],
                        [px, py + 1],
                        [px - 1, py],
                        [px + 1, py],
                    ];

                    let mut count = 0;
                    let mut aggregate_value = 0.0;
                    for test_pos in positions {
                        if let Some(test_value) = get_value_i32(ga, width, height, test_pos) {
                            count += 1;
                            aggregate_value += test_value;
                        }
                    }

                    let fr = 0.1;
                    gb[ind] = ga[ind] * (1.0 - fr) * retain;
                    gb[ind] += (aggregate_value / count as f32) * fr;
                }
            }

            if self.counter % 8 == 0 {
                // these take 5 ms each at 640x480
                smooth_gradient(
                    width,
                    height,
                    &self.move_gradient0a,
                    &mut self.move_gradient0b,
                );
                // these swaps take no time
                std::mem::swap(&mut self.move_gradient0a, &mut self.move_gradient0b);
            } else if self.counter % 8 == 4 {
                smooth_gradient(
                    width,
                    height,
                    &self.move_gradient1a,
                    &mut self.move_gradient1b,
                );
                std::mem::swap(&mut self.move_gradient1a, &mut self.move_gradient1b);
            }

            fn sigmoid(x: f32) -> f32 {
                1.0 / (1.0 + (-x).exp())
            }

            // draw the move gradients
            for yi in 0..height {
                for xi in 0..width {
                    let ind = yi * width + xi;
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;

                    // if self.max0 > 0.0
                    {
                        let v = self.move_gradient0a[ind];
                        let v = 2.0 * (sigmoid(30.0 * v) - 0.5);
                        r += (175.0 * v).clamp(0.0, 255.0) as u16;
                        g += (58.0 * v).clamp(0.0, 255.0) as u16;
                        b += (188.0 * v).clamp(0.0, 255.0) as u16;
                    }

                    // if self.max1 > 0.0
                    {
                        let v = self.move_gradient1a[ind];
                        let v = 2.0 * (sigmoid(30.0 * v) - 0.5);
                        r += (15.0 * v).clamp(0.0, 255.0) as u16;
                        g += (195.0 * v).clamp(0.0, 255.0) as u16;
                        b += (58.0 * v).clamp(0.0, 255.0) as u16;
                    }

                    let color = egui::Color32::from_rgb(r as u8, g as u8, b as u8);
                    self.color_image.pixels[ind] = color;
                }
            }

            // draw the particles
            self.particles1.iter_mut().for_each(|particle| {
                let ind = (particle.y.round() * width as f32 + particle.x.round()) as usize;
                let color = egui::Color32::from_rgb(255, 130, 225);
                if ind < (width * height) {
                    self.color_image.pixels[ind] = color;
                }
            });

            self.particles0.iter_mut().for_each(|particle| {
                let ind = (particle.y.round() * width as f32 + particle.x.round()) as usize;
                let color = egui::Color32::from_rgb(190, 255, 130);
                if ind < (width * height) {
                    self.color_image.pixels[ind] = color;
                }
            });

            self.counter += 1;
        }

        fn reset(&mut self) {
            self.move_gradient0a = vec![0.0; self.width * self.height];
            self.move_gradient1a = self.move_gradient0a.clone();
            for yi in 0..self.height {
                for xi in 0..self.width {
                    let ind = yi * self.width + xi;
                    let fr = 0.000004;
                    let sc = 0.001;
                    let base = 0.0001;
                    self.move_gradient0a[ind] = base + rand::random::<f32>() * sc + yi as f32 * fr;
                    self.move_gradient1a[ind] =
                        base + rand::random::<f32>() * sc + (self.height - yi) as f32 * fr;
                }
            }
            self.move_gradient0b = self.move_gradient0a.clone();
            self.move_gradient1b = self.move_gradient1a.clone();

            self.particles0.clear();
            for _i in 0..250 {
                let x = rand::random::<f32>() * (self.width - 1) as f32;
                let y = 0.25 * (rand::random::<f32>() * (self.height - 1) as f32);
                self.particles0.push(Particle { x, y });
            }

            self.particles1.clear();
            for _i in 0..250 {
                let x = rand::random::<f32>() * (self.width - 1) as f32;
                let y = 0.25 * (rand::random::<f32>() * (self.height - 1) as f32);
                self.particles1.push(Particle {
                    x,
                    y: self.height as f32 - y,
                });
            }
        }

    fn move_particles(
        width: usize,
        height: usize,
        particles: &mut [Particle],
        field0: &mut [f32],
        field1: &mut [f32],
    ) {
        let speed: f32 = 0.55;
        let num = 16;
        let mut samples = Vec::new();
        for dist in [1.2, 2.5, 4.0] {
            let new_samples: Vec<_> = (0..num)
            .map(|ind| {
                let angle = (ind as f32 / num as f32) * std::f32::consts::PI * 2.0;
                let cosa = angle.cos();
                let sina = angle.sin();
                ([cosa, sina], [dist * cosa, dist * sina], speed)
            })
            .collect();
            samples.extend(new_samples);
        }

        particles.iter_mut().for_each(|particle| {
            let px = particle.x;
            let py = particle.y;

            if rand::random::<f32>() < 0.0004 {
                // this particle dies and is respawned
                let ind = particle.y.round() as usize * width + particle.x.round() as usize;
                if ind < field1.len() {
                    field1[ind] += 5.0;
                    field0[ind] -= 0.5;
                }
                particle.x = rand::random::<f32>() * (width - 1) as f32;
                particle.y = rand::random::<f32>() * (height - 1) as f32;
            } else {
                // move the particle to more attractive neighbor position
                let pos = [px, py];
                let mut best_move = pos;
                if let Some(mut best_move_value) = get_value(field0, width, height, pos) {
                    for ([cosa, sina], [sx, sy], scale) in &samples {
                        let tpx = px + sx;
                        let tpy = py + sy;
                        let test_pos: [f32; 2] = [tpx, tpy];
                        if let Some(test_value) = get_value(field0, width, height, test_pos)
                            && test_value > best_move_value
                        {
                            best_move = [px + scale * cosa, py + scale * sina];
                            best_move_value = test_value;
                        }
                    }

                    particle.x = best_move[0];
                    particle.y = best_move[1];
                }
            }

            particle.x %= (width - 1) as f32;
            particle.y %= (height - 1) as f32;

            let ind = particle.y.round() as usize * width + particle.x.round() as usize;
            // if ind < field1.len() {
            // attract the opposite particles
            field1[ind] += 0.1;
            // repel other particles (though this also self repels)
            field0[ind] -= 0.05;
            // }
        });
    }
    */
}

#[derive(Default)]
struct App {
    width: usize,
    height: usize,

    game: Game,

    color_image: egui::ColorImage,
    texture_handle: Option<egui::TextureHandle>,

    // use_plot_ui: bool,
    allow_zoom: bool,
    pause: bool,

    instant: Option<Instant>,
    update_elapsed: Duration,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            if !self.pause
                && (self.instant.is_none()
                    || self.instant.unwrap().elapsed()
                        > Duration::from_millis((1000.0 * 1.0 / 35.0) as u64))
            {
                self.instant = Some(Instant::now());
                self.game.update(&mut self.color_image);
                self.update_elapsed = self.instant.unwrap().elapsed();
            }
            ui.label(format!(
                "update elapsed {:.2}ms",
                self.update_elapsed.as_micros() as f32 / 1000.0,
            ));

            let size = self.color_image.size;
            ui.label(format!(
                "vec size {size:?} {} -> {}",
                size[0] * size[1],
                self.color_image.pixels.len()
            ));
            // ui.label(format!("max0 {:.3}, max1 {:.3}", self.max0, self.max1));
            //
            ui.label(format!("{} soldiers left", self.game.soldiers.len()));
        });

        let mut reset_plot = false;
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.pause, "pause");

            // ui.checkbox(&mut self.use_plot_ui, "use egui_plot").changed();
            reset_plot = ui
                .checkbox(&mut self.allow_zoom, "allow zoom and drag")
                .changed()
                && !self.allow_zoom;

            if ui.button("reset").clicked() {
                self.reset();
            }
        });

        let texture_handle = self.texture_handle.get_or_insert(ui.ctx().load_texture(
            "image",
            self.color_image.clone(),
            egui::TextureOptions::NEAREST,
        ));

        // TODO(lucasw) use egui_plot to scale the image to the window
        texture_handle.set(self.color_image.clone(), egui::TextureOptions::NEAREST);

        // if self.use_plot_ui {
        {
            let mut plot = Plot::new("image")
                // .grid_spacing(egui::Rangef::new(10000.0, 10000.0))
                .show_grid([false, false])
                .set_margin_fraction(egui::vec2(0.0, 0.0))
                .auto_bounds(true)
                .allow_double_click_reset(true)
                .allow_zoom(self.allow_zoom)
                .allow_drag(self.allow_zoom)
                .allow_scroll(false)
                .show_axes([false, false])
                .data_aspect(1.0);
            if reset_plot {
                plot = plot.reset();
            }

            // egui::CentralPanel::default().show(ui, |ui| {
            let _plot_response = plot.show(ui, |plot_ui| {
                let size = texture_handle.size_vec2();
                let plot_image = PlotImage::new(
                    "image",
                    texture_handle.id(),
                    PlotPoint::new(size[0] / 2.0, -size[1] / 2.0),
                    size,
                );
                plot_ui.image(plot_image);
            });
        }
        /* else {
            ui.image(texture_handle);
        }
        */

        // TODO(lucasw) will this sometimes exceed 30 Hz if the mouse is moving?
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_secs_f32(1.0 / 30.0));
    }
}

impl App {
    fn reset(&mut self) {
        self.game.reset(self.width, self.height);
    }
}

fn main() {
    let (width, height) = (640, 480);
    // let (width, height) = (1280, 720);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([(width + 50) as f32, (height + 50) as f32]),
        ..Default::default()
    };

    let fill_color = egui::Color32::BLACK;
    let color_image = egui::ColorImage::filled([width, height], fill_color);

    let mut app = App {
        width,
        height,
        color_image,
        ..Default::default()
    };

    app.reset();

    let rv = eframe::run_native("texture_egui", options, Box::new(|_cc| Ok(Box::new(app))));

    println!("{rv:?}");
}
