extern crate image;
extern crate libloading;

// use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use image::{GenericImageView};
use std::convert::TryFrom;
// use std::env;
use libloading::{Library, Symbol};
// use std::ffi::CString;
use std::ffi::CStr;
use std::mem;
use std::ptr;

pub type F0rInstance = *mut ::std::os::raw::c_void;

#[doc = " The f0r_plugin_info_t structure is filled in by the plugin"]
#[doc = " to tell the application about its name, type, number of parameters,"]
#[doc = " and version."]
#[doc = ""]
#[doc = " An application should ignore (i.e. not use) frei0r effects that"]
#[doc = " have unknown values in the plugin_type or color_model field."]
#[doc = " It should also ignore effects with a too high frei0r_version."]
#[doc = ""]
#[doc = " This is necessary to be able to extend the frei0r spec (e.g."]
#[doc = " by adding new color models or plugin types) in a way that does not"]
#[doc = " result in crashes when loading effects that make use of these"]
#[doc = " extensions into an older application."]
#[doc = ""]
#[doc = " All strings are unicode, 0-terminated, and the encoding is utf-8."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct F0rPluginInfo {
    #[doc = "< The (short) name of the plugin"]
    pub name: *const ::std::os::raw::c_char,
    #[doc = "< The plugin author"]
    pub author: *const ::std::os::raw::c_char,
    #[doc = " The plugin type"]
    #[doc = " \\see PLUGIN_TYPE"]
    pub plugin_type: ::std::os::raw::c_int,
    #[doc = "< The color model used"]
    pub color_model: ::std::os::raw::c_int,
    #[doc = "< The frei0r major version this plugin is built for"]
    pub frei0r_version: ::std::os::raw::c_int,
    #[doc = "< The major version of the plugin"]
    pub major_version: ::std::os::raw::c_int,
    #[doc = "< The minor version of the plugin"]
    pub minor_version: ::std::os::raw::c_int,
    #[doc = "< The number of parameters of the plugin"]
    pub num_params: ::std::os::raw::c_int,
    #[doc = "< An optional explanation string"]
    pub explanation: *const ::std::os::raw::c_char,
}

type F0rUpdate = fn(F0rInstance, f64, *const u32, *mut u32);
pub type F0rParamT = *mut ::std::os::raw::c_void;
pub type F0rSetParamValue = fn(F0rInstance, F0rParamT, ::std::os::raw::c_int);

pub struct F0rInstanceWrapper {
    f0r_instance: F0rInstance,
    f0r_update: F0rUpdate,
    f0r_set_param_value: F0rSetParamValue,
    width: usize,
    height: usize,
}

fn vec8to32(vec8: &Vec<u8>) -> Vec<u32> {
    let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();
    let length = vec8.len() / ratio;
    let capacity = vec8.capacity() / ratio;
    let vec32 = unsafe {
        // let ptr = vec8.as_mut_ptr() as *mut u32;
        let ptr = vec8.as_ptr() as *mut u32;

        // Don't run the destructor for vec32
        // mem::forget(vec8);

        // Construct new Vec
        Vec::from_raw_parts(ptr, length, capacity)
    };
    let vec32b = vec32.clone();
    mem::forget(vec32);
    vec32b
}

fn vec32to8(vec32: &Vec<u32>) -> Vec<u8> {
    let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();
    let length = vec32.len() * ratio;
    let capacity = vec32.capacity() * ratio;
    let vec8 = unsafe {

        // let ptr = vec32.as_mut_ptr() as *mut u8;
        let ptr = vec32.as_ptr() as *mut u8;

        // Construct new Vec
        Vec::from_raw_parts(ptr, length, capacity)
    };
    let vec8b = vec8.clone();
    // vec8 destructor would also destroy the passed in vec32- use a box or arc to handle that
    // instead?
    mem::forget(vec8);
    vec8b
}

impl F0rInstanceWrapper {
    fn update(&self, time: f64, vec8: Vec<u8>) -> Vec<u8> {
        let vec32 = vec8to32(&vec8);
        println!("converted {} {}", vec8.len(), vec32.len());
        // TODO(lucasw) create this once on init an re-use it?
        let mut vec32_out: Vec<u32> = vec![0; vec32.len()];
        println!("vec32 out 1 {}", vec32_out.len());
        // TODO(lucasw) this is segfaulting
        (self.f0r_update)(self.f0r_instance, time, vec32.as_ptr(), vec32_out.as_mut_ptr());

        println!("vec32 out 2 {}", vec32_out.len());
        let vec8_out = vec32to8(&vec32_out);
        println!("converted {} {}", vec32_out.len(), vec8_out.len());
        vec8_out
    }

    fn update_test(&self) {
        let mut vec32: Vec<u32> = vec![0; self.width * self.height];
        let mut count = 0;
        println!("vec32 {} {:p}", vec32.len(), &vec32);
        for pix in vec32.iter_mut() {
            *pix = count << 8;
            print!("{:08X?} ", pix);
            count += 1;

            if count % 4 == 0 {
                println!("");
            }
        }
        println!("");

        // TODO(lucasw) these are leaking memory
        let vec8 = vec32to8(&vec32);
        println!("vec32to8 {} {:p}", vec8.len(), &vec8);
        count = 0;
        for pix in vec8.iter() {
            print!("{:02X?} ", pix);
            count += 1;
            if count % 4 == 0 {
                print!(" ");
            }
            if count % 16 == 0 {
                println!("");
            }
        }
        println!("");
/*
        let vec32b = vec8to32(&vec8);
        println!("vec8to32b {}", vec32b.len());
        count = 0;
        for pix in vec32.iter() {
            print!("{:08X?} ", pix);
            count += 1;
        }
        println!("");
*/
        self.set_param_value(0.1, 0);

        let mut vec32_out: Vec<u32> = vec![0; vec32.len()];
        (self.f0r_update)(self.f0r_instance, 0.0, vec32.as_ptr(), vec32_out.as_mut_ptr());

        println!("f0r update output vec32 {}", vec32_out.len());
        count = 0;
        for pix in vec32_out.iter() {
            print!("{:08X?} ", pix);
            count += 1;
            if count % 4 == 0 {
                println!("");
            }
        }
        println!("");
    }

    fn set_param_value(&self, val_arg: f64, ind: i32) {
        let mut val: f64 = val_arg;
        let val_ptr: *mut ::std::os::raw::c_void = &mut val as *mut _ as *mut ::std::os::raw::c_void;
        (self.f0r_set_param_value)(self.f0r_instance, val_ptr, ind);
    }
    // TODO(lucasw) need a Drop to deconstruct this instance
}

type F0rConstruct = fn(::std::os::raw::c_uint, ::std::os::raw::c_uint) -> F0rInstance;
type F0rInit = fn();

pub type F0rGetPluginInfo = fn(*mut F0rPluginInfo);

pub struct F0rWrapper {
    library_path: String,
    lib: Library,
    name: String,
    author: String,
    explanation: String,
    raw_info: F0rPluginInfo,
}

impl F0rWrapper {
    fn print(&self) {
        // TODO(lucasw) only want to call from_raw once needs to be i
        println!("'{}' '{}'", self.name, self.author);
        println!("'{}'", self.explanation);
        println!("'{}'", self.library_path);
        println!("plugin type {}, color model {}",
                 self.raw_info.plugin_type, self.raw_info.color_model);
        println!("frei0r version {} {} {}",
                 self.raw_info.frei0r_version, self.raw_info.major_version, self.raw_info.minor_version);
        println!("num params {}", self.raw_info.num_params);
    }

    fn default(library_path: &str) -> F0rWrapper {
        let lib = Library::new(library_path).unwrap();

        println!("f0r_init {}", library_path);
        unsafe {
            let f0r_initor: Symbol<F0rInit> = lib.get(b"f0r_init").unwrap();
            f0r_initor();
        }

        println!("f0r_get_plugin_info");
        let mut raw_info = F0rPluginInfo {
            name: ptr::null(),
            author: ptr::null(),
            plugin_type: 0,
            color_model: 0,
            frei0r_version: 0,
            major_version: 0,
            minor_version: 0,
            num_params: 0,
            explanation: ptr::null(),
        };

        let name;
        let author;
        let explanation;
        unsafe {
            let f0r_get_plugin_infor: Symbol<F0rGetPluginInfo> = lib.get(b"f0r_get_plugin_info").unwrap();
            // Need to write a Default for this to work?
            f0r_get_plugin_infor(&mut raw_info);

            name = CStr::from_ptr(raw_info.name).to_string_lossy().into_owned();
            author = CStr::from_ptr(raw_info.author).to_string_lossy().into_owned();
            explanation = CStr::from_ptr(raw_info.explanation).to_string_lossy().into_owned();
        }

        F0rWrapper {
            library_path: library_path.to_string(),
            lib,
            name,
            author,
            explanation,
            raw_info,
        }
    }

    // TODO(lucasw) need a Drop to call the deinit

    fn instance(&self, width: u32, height: u32) -> F0rInstanceWrapper {
        let f0r_instance;
        unsafe {
            /*
            let t0 = CString::new("abc").unwrap();
            let t0_ptr = t0.into_raw();
            let t1 = CString::from_raw(t0_ptr);
            println!("cstring test {:?}", t1);
            */

            println!("w {} x h {}", width, height);
            let f0r_constructor: Symbol<F0rConstruct> = self.lib.get(b"f0r_construct").unwrap();
            f0r_instance = f0r_constructor(width, height);
        }
        let f0r_update : Symbol<F0rUpdate>;
        unsafe {
            f0r_update = self.lib.get(b"f0r_update").unwrap();
        }

        let f0r_set_param_value: Symbol<F0rSetParamValue>;
        unsafe {
            f0r_set_param_value = self.lib.get(b"f0r_set_param_value").unwrap();
        }

        F0rInstanceWrapper {
            f0r_instance: f0r_instance,
            f0r_update: *f0r_update,
            f0r_set_param_value: *f0r_set_param_value,
            width: usize::try_from(width).unwrap(),
            height: usize::try_from(height).unwrap(),
        }
    }
}

fn main() {
    let library_path = "/usr/lib/frei0r-1/saturat0r.so";  // env::args().nth(1).expect("USAGE: loading <LIB>");

    let f0r = F0rWrapper::default(&library_path);
    f0r.print();
    let width = 8;
    let height = 8;
    let f0r_inst = f0r.instance(width, height);
    f0r_inst.update_test();

    println!("now try real image");
    // TODO(lucasw) pass this in from a command line
    let img_name = "/home/lucasw/catkin_ws/src/lucasw/vimjay/data/slowmo/frame01294_sm.png";
    // TODO(lucasw) handle the case where this isn't found
    // TODO(lucasw) need to force alpha channel on this
    let img = image::open(img_name).unwrap();
    let (width, height) = img.dimensions();
    let rgba: Vec<u8> = img.into_rgba8().to_vec();
    image::save_buffer("test_in.png", &rgba, width, height, image::ColorType::Rgba8).unwrap();
    let f0r_inst = f0r.instance(width, height);
    f0r_inst.set_param_value(0.04, 0);
    let rgba_out = f0r_inst.update(0.0, rgba);
    image::save_buffer("test_out.png", &rgba_out, width, height, image::ColorType::Rgba8).unwrap();
}
