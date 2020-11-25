extern crate libloading;

// use std::env;
use libloading::{Library, Symbol};
use std::ffi::CString;

// type AddFunc = fn(isize, isize) -> isize;

pub type F0rInstance = *mut ::std::os::raw::c_void;

/*
extern "C" {
    #[doc = " f0r_init() is called once when the plugin is loaded by the application."]
    #[doc = " \\see f0r_deinit"]
    pub fn f0r_init() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " f0r_deinit is called once when the plugin is unloaded by the application."]
    #[doc = " \\see f0r_init"]
    pub fn f0r_deinit();
}
*/

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

/*
extern "C" {
    #[doc = " Constructor for effect instances. The plugin returns a pointer to"]
    #[doc = " its internal instance structure."]
    #[doc = ""]
    #[doc = " The resolution must be an integer multiple of 8,"]
    #[doc = " must be greater than 0 and be at most 2048 in both dimensions."]
    #[doc = " The plugin must set default values for all parameters in this function."]
    #[doc = ""]
    #[doc = " \\param width The x-resolution of the processed video frames"]
    #[doc = " \\param height The y-resolution of the processed video frames"]
    #[doc = " \\returns 0 on failure or a pointer != 0 on success"]
    #[doc = ""]
    #[doc = " \\see f0r_destruct"]
    pub fn f0r_construct(
        width: ::std::os::raw::c_uint,
        height: ::std::os::raw::c_uint,
    ) -> F0rInstance;

    pub fn f0r_update(instance: F0rInstance, time: f64, inframe: *const u32, outframe: *mut u32);
}
*/
type F0rConstruct = fn(::std::os::raw::c_uint, ::std::os::raw::c_uint) -> F0rInstance;
type F0rInit = fn();

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
/*
extern "C" {
    #[doc = " Is called once after init. The plugin has to fill in the values in info."]
    #[doc = ""]
    #[doc = " \\param info Pointer to an info struct allocated by the application."]
    pub fn f0r_get_plugin_info(info: *mut F0rPluginInfo);
}
*/
pub type F0rGetPluginInfo = fn(*mut F0rPluginInfo);


fn main() {
    let library_path = "/usr/lib/frei0r-1/saturat0r.so";  // env::args().nth(1).expect("USAGE: loading <LIB>");
    println!("loading {}", library_path);

    let lib = Library::new(library_path).unwrap();

    let width = 8;
    let height = 8;

    unsafe {
        println!("f0r_init");
        let f0r_initor: Symbol<F0rInit> = lib.get(b"f0r_init").unwrap();
        f0r_initor();

        println!("f0r_get_plugin_info");
        let f0r_get_plugin_infor: Symbol<F0rGetPluginInfo> = lib.get(b"f0r_get_plugin_info").unwrap();
        // Need to write a Default for this to work?
        let mut info = F0rPluginInfo {
            name: CString::new("abc").unwrap().as_ptr(),
            author: CString::new("def").unwrap().as_ptr(),
            plugin_type: 0,
            color_model: 0,
            frei0r_version: 0,
            major_version: 0,
            minor_version: 0,
            num_params: 0,
            explanation: CString::new("none").unwrap().as_ptr(),
        };
        f0r_get_plugin_infor(&mut info);
        // Not sure about the strings yet
        // println!("num_params {} {}", CString::from_raw(info.name).to_str().unwrap(), info.num_params);
        println!("plugin type {}, color model {}",
                 info.plugin_type, info.color_model);
        println!("frei0r version {} {} {}",
                 info.frei0r_version, info.major_version, info.minor_version);
        println!("num params {}", info.num_params);

        let f0r_constructor: Symbol<F0rConstruct> = lib.get(b"f0r_construct").unwrap();
        let _instance = f0r_constructor(width, height);

        println!("done with frei0r");
    }
}
