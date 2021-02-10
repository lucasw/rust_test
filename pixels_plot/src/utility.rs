// use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
// use std::process;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;

pub fn load_csv(csv_file: File) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    // println!("loading '{}'", csv_file);

    let mut data: Vec<Vec<f64>> = Vec::new();

    let mut reader = csv::Reader::from_reader(csv_file);
    for (i, result) in reader.records().enumerate() {
        // TODO(lucasw) Use a fixed size array inside the outer Vec
        let record = result?;
        if i == 0 {
            data.resize(record.len(), Vec::new());
            println!("processing {} columns", data.len());
        }
        if data.len() != record.len() {
            println!("unexpected record len {} {}", data.len(), record.len());
            continue;
        }
        // println!("{:?}", record);
        for (j, val_str) in record.iter().enumerate() {
            // print!(" '{}' -> ", val_str);
            let val = val_str.trim_start_matches(' ').parse::<f64>().unwrap();
            // print!(" {},", val);
            data[j].push(val);
        }
        // println!("");
    }
    Ok(data)
}

pub fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    // println!("args {:?}", args);
    // load a csv file
    let filename;
    if args.len() > 1 {
        filename = args[1].clone();
    } else {
        filename = "data.csv".to_string();
    }
    println!("file '{}'", filename);
    filename
}

pub struct Image {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

fn draw_point(image: &mut Image, x: f64, y: f64, color: u32) {
    let width = image.width;
    if x < 0.0 || x as usize >= width {
        return;
    }
    let height = image.height;
    if y < 0.0 || y as usize >= height {
        return;
    }
    let x = x as usize;
    let y = y as usize;
    let ind = y * width + x;
    image.buffer[ind] = color;
}

trait RGBExt {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl RGBExt for u32 {
    fn r(&self) -> u8 {
        ((self & (0xff << 24)) >> 24) as u8
    }
    fn g(&self) -> u8 {
        ((self & (0xff << 16)) >> 16) as u8
    }
    fn b(&self) -> u8 {
        ((self & (0xff << 8)) >> 8) as u8
    }
}

fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8
}

pub fn make_plot(mut image: &mut Image, filename: &str, x_scale: f64, y_scale: f64) {
    let width = image.width;
    let height = image.height;
    let sc = 0.95;
    for i in 0..(width * height) {
        let r = image.buffer[i].r();
        let g = image.buffer[i].g();
        let b = image.buffer[i].b();
        image.buffer[i] = from_rgb(
            (r as f64 * sc) as u8,
            (g as f64 * sc) as u8,
            (b as f64 * sc) as u8,
        );
    }

    let path = Path::new(filename);
    let csv_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(csv_file) => csv_file,
    };

    let columns = load_csv(csv_file).unwrap();

    for (col_ind, column) in columns.iter().enumerate() {
        // println!("{} {:?}", col_ind, column);
        let color = from_rgb(
            (col_ind * 30) as u8,
            (255 - (col_ind * 20)) as u8,
            (50 + col_ind * 10) as u8,
        );
        let tiles = 2;
        let x_offset = ((col_ind % tiles) * width / tiles + 10) as f64;
        let y_offset = (col_ind / tiles) as f64 * 180.0 + 120.0;

        for (i, val) in column.iter().enumerate() {
            let x = i as f64 * x_scale + 50.0 + x_offset;
            let y = val * y_scale + y_offset;
            draw_point(&mut image, x, height as f64 - y, color);
            draw_point(
                &mut image,
                x,
                height as f64 - y_offset,
                from_rgb(128, 128, 128),
            );
        }
    }
}

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
pub fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
    width: usize,
    height: usize,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = width as f64;
    let height = height as f64;
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
