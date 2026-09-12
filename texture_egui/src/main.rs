use eframe::egui;
// use egui_plot::{Plot, PlotImage};

struct Particle {
    x: i32,
    y: i32,
}

/*
impl Particle {
}
*/

struct App {
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
    // texture_handle: egui::TextureHandle,
    counter: usize,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.update();

        egui::CentralPanel::default().show(ui, |ui| {
            let size = self.color_image.size;
            ui.label(format!(
                "vec size {size:?} {} -> {}",
                size[0] * size[1],
                self.color_image.pixels.len()
            ));
            ui.label(format!("max0 {:.6}, max1 {:.6}", self.max0, self.max1));
            // TODO(lucasw) use egui_plot to scale the image to the window
            let texture_handle = ui.ctx().load_texture(
                "image",
                self.color_image.clone(),
                egui::TextureOptions::NEAREST,
            );

            ui.image(&texture_handle);

            // TODO(lucasw) will this sometimes exceed 30 Hz if the mouse is moving?
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_secs_f32(1.0 / 30.0));
        });
    }
}

fn get_value(values: &[f32], width: usize, height: usize, pos: [i32; 2]) -> Option<f32> {
    let [x, y] = pos;
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
    particles.iter_mut().for_each(|particle| {
        let px = particle.x;
        let py = particle.y;

        {
            // move the particle to more attractive neighbor position
            let pos = [px, py];
            let moves = vec![
                [px - 1, py - 1],
                [px - 1, py + 1],
                [px + 1, py + 1],
                [px + 1, py - 1],
                [px, py - 1],
                [px, py + 1],
                [px - 1, py],
                [px + 1, py],
            ];

            let mut best_move = pos;
            if let Some(mut best_move_value) = get_value(field0, width, height, pos) {
                for test_pos in moves {
                    if let Some(test_value) = get_value(field0, width, height, test_pos)
                        && test_value > best_move_value
                    {
                        best_move = test_pos;
                        best_move_value = test_value;
                    }
                }

                particle.x = best_move[0];
                particle.y = best_move[1];
                particle.x %= width as i32;
                particle.y %= height as i32;
            }
        }

        let ind = particle.y as usize * width + particle.x as usize;
        field1[ind] += 0.2;
        // repel other particles (though this also self repels)
        field0[ind] -= 0.05;
    });
}

impl App {
    fn update(&mut self) {
        let [width, height] = self.color_image.size;

        let color = egui::Color32::from_rgb(0, 0, 0);
        for yi in 0..height {
            for xi in 0..width {
                let ind = yi * width + xi;
                self.color_image.pixels[ind] = color;
            }
        }

        let retain = 0.995;
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
                let pos_u = [px, py - 1];
                let pos_d = [px, py + 1];
                let pos_l = [px - 1, py];
                let pos_r = [px + 1, py];

                let mut count = 0;
                let mut aggregate_value = 0.0;
                for test_pos in [pos_u, pos_d, pos_l, pos_r] {
                    if let Some(test_value) = get_value(ga, width, height, test_pos) {
                        count += 1;
                        aggregate_value += test_value;
                    }
                }

                let fr = 0.25;
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
                let b = 0;

                if self.max0 > 0.0 {
                    let v = self.move_gradient0a[ind];
                    let v = 2.0 * (sigmoid(30.0 * v) - 0.5);
                    r += (255.0 * v).clamp(0.0, 255.0) as u8;
                }

                if self.max1 > 0.0 {
                    let v = self.move_gradient1a[ind];
                    let v = 2.0 * (sigmoid(30.0 * v) - 0.5);
                    g += (255.0 * v).clamp(0.0, 255.0) as u8;
                }

                let color = egui::Color32::from_rgb(r, g, b);
                self.color_image.pixels[ind] = color;
            }
        }

        // draw the particles
        self.particles1.iter_mut().for_each(|particle| {
            let ind = particle.y * width as i32 + particle.x;
            let color = egui::Color32::from_rgb(255, 100, 35);
            self.color_image.pixels[ind as usize] = color;
        });

        self.particles0.iter_mut().for_each(|particle| {
            let ind = particle.y * width as i32 + particle.x;
            let color = egui::Color32::from_rgb(100, 255, 30);
            self.color_image.pixels[ind as usize] = color;
        });

        self.counter += 1;
    }
}

fn main() {
    let width = 640;
    let height = 480;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([(width + 50) as f32, (height + 50) as f32]),
        ..Default::default()
    };

    let fill_color = egui::Color32::DARK_BLUE;

    let mut move_gradient0 = vec![0.0; width * height];
    let mut move_gradient1 = move_gradient0.clone();
    for yi in 0..height {
        for xi in 0..width {
            let ind = yi * width + xi;
            let fr = 0.00000003;
            let sc = 0.1;
            move_gradient0[ind] = 0.01 + rand::random::<f32>() * sc + yi as f32 * fr;
            move_gradient1[ind] = 0.01 + rand::random::<f32>() * sc + (height - yi) as f32 * fr;
        }
    }

    let mut app = App {
        particles0: Vec::new(),
        move_gradient0a: move_gradient0.clone(),
        move_gradient0b: move_gradient0,
        max0: 0.0,
        particles1: Vec::new(),
        move_gradient1a: move_gradient1.clone(),
        move_gradient1b: move_gradient1,
        max1: 0.0,
        color_image: egui::ColorImage::filled([width, height], fill_color),
        counter: 0,
    };

    for i in 0..150 {
        let x = 5 + i * 10;
        app.particles0.push(Particle {
            x: (x % width) as i32,
            y: (30 + (x / width) * 10) as i32 + (rand::random::<f32>() * 20.0) as i32,
        });
    }

    for i in 0..150 {
        let x = 5 + i * 10;
        app.particles1.push(Particle {
            x: (x % width) as i32,
            y: (height - 20 - (x / width) * 10) as i32 - (rand::random::<f32>() * 20.0) as i32,
        });
    }

    let rv = eframe::run_native("texture_egui", options, Box::new(|_cc| Ok(Box::new(app))));

    println!("{rv:?}");
}
