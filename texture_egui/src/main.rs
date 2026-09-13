use eframe::egui;
use egui_plot::{Plot, PlotImage, PlotPoint};
use std::time::{Duration, Instant};

struct Particle {
    x: f32,
    y: f32,
}

/*
impl Particle {
}
*/

#[derive(Default)]
struct App {
    width: usize,
    height: usize,

    particles0: Vec<Particle>,
    /// particles will move to higher scoring neighbor cells here
    move_gradient0a: Vec<f32>,
    move_gradient0b: Vec<f32>,
    max0: f32,

    particles1: Vec<Particle>,
    move_gradient1a: Vec<f32>,
    move_gradient1b: Vec<f32>,
    max1: f32,

    color_image: egui::ColorImage,
    texture_handle: Option<egui::TextureHandle>,

    // use_plot_ui: bool,
    allow_zoom: bool,
    pause: bool,

    instant: Option<Instant>,
    update_elapsed: Duration,
    counter: usize,
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
                self.update();
                self.update_elapsed = self.instant.unwrap().elapsed();
            }
            ui.label(format!(
                "update elapsed {:.3}ms",
                self.update_elapsed.as_millis()
            ));

            let size = self.color_image.size;
            ui.label(format!(
                "vec size {size:?} {} -> {}",
                size[0] * size[1],
                self.color_image.pixels.len()
            ));
            ui.label(format!("max0 {:.3}, max1 {:.3}", self.max0, self.max1));
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

fn move_particles(
    width: usize,
    height: usize,
    particles: &mut [Particle],
    field0: &mut [f32],
    field1: &mut [f32],
) {
    let speed: f32 = 0.85;
    let dist: f32 = 2.5;
    let num = 16;
    let samples: Vec<([f32; 2], f32)> = (0..num)
        .map(|ind| {
            let angle = (ind as f32 / num as f32) * std::f32::consts::PI * 2.0;
            ([dist * angle.cos(), dist * angle.sin()], speed)
        })
        .collect();

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
                for ([ox, oy], scale) in &samples {
                    let tpx = px + ox;
                    let tpy = py + oy;
                    let test_pos: [f32; 2] = [tpx, tpy];
                    if let Some(test_value) = get_value(field0, width, height, test_pos)
                        && test_value > best_move_value
                    {
                        best_move = [px + scale * ox, py + scale * oy];
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

impl App {
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

    fn update(&mut self) {
        let [width, height] = self.color_image.size;

        let color = egui::Color32::from_rgb(0, 0, 0);
        for yi in 0..height {
            for xi in 0..width {
                let ind = yi * width + xi;
                self.color_image.pixels[ind] = color;
            }
        }

        let retain = 0.998;
        {
            // let mut min0: f32 = f32::MAX;
            let mut max0 = f32::MIN;
            self.move_gradient0a.iter_mut().for_each(|v| {
                *v *= retain;
                max0 = max0.max(*v);
                // min0 = min0.min(*v);
            });
            self.max0 = max0;
        }

        {
            let mut max1 = f32::MIN;
            self.move_gradient1a.iter_mut().for_each(|v| {
                *v *= retain;
                max1 = max1.max(*v);
            });
            self.max1 = max1;
        }

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
                gb[ind] = ga[ind] * (1.0 - fr);
                gb[ind] += (aggregate_value / count as f32) * fr; // * 0.999;
            }
        }

        smooth_gradient(
            width,
            height,
            &self.move_gradient0a,
            &mut self.move_gradient0b,
        );
        std::mem::swap(&mut self.move_gradient0a, &mut self.move_gradient0b);

        smooth_gradient(
            width,
            height,
            &self.move_gradient1a,
            &mut self.move_gradient1b,
        );
        std::mem::swap(&mut self.move_gradient1a, &mut self.move_gradient1b);

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

                if self.max0 > 0.0 {
                    let v = self.move_gradient0a[ind];
                    let v = 2.0 * (sigmoid(30.0 * v) - 0.5);
                    r += (175.0 * v).clamp(0.0, 255.0) as u16;
                    g += (58.0 * v).clamp(0.0, 255.0) as u16;
                    b += (188.0 * v).clamp(0.0, 255.0) as u16;
                }

                if self.max1 > 0.0 {
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
