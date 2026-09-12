use eframe::egui;
// use egui_plot::{Plot, PlotImage};

struct App {
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

impl App {
    fn update(&mut self) {
        let [wd, ht] = self.color_image.size;
        for yi in 0..ht {
            let r = (yi + self.counter % 256) as u8;
            let g = (self.counter % 256) as u8;
            let b = 128;
            let color = egui::Color32::from_rgb(r, g, b);
            for xi in 0..wd {
                let ind = yi * wd + xi;
                self.color_image.pixels[ind] = color;
            }
        }

        self.counter += 1;
    }
}

fn main() {
    let width = 640;
    let height = 480;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([width as f32, height as f32]),
        ..Default::default()
    };

    let fill_color = egui::Color32::DARK_BLUE;

    let rv = eframe::run_native(
        "texture_egui",
        options,
        Box::new(|_cc| {
            Ok(Box::new(App {
                color_image: egui::ColorImage::filled([width, height], fill_color),
                counter: 0,
            }))
        }),
    );

    println!("{rv:?}");
}
