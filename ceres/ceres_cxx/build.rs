use std::path::PathBuf;

fn main() {
    /*
    let eigen = pkg_config::probe_library("eigen3").unwrap();
    let eigen_include_paths = eigen.include_paths.iter().map(PathBuf::as_path);
    let mut eigen_paths = String::new();
    // for path in eigen_include_paths {
    for path in eigen.include_paths {
        eigen_paths += &format!(" -I {:?}", path);
    }
    // println!("{:?}", CFG.exported_header_dirs);
    // panic!("foo");
    */

    cxx_build::bridge("src/main.rs")
        .file("src/ceres_example.cc")
        // TODO(lucasw) how to pass this in in a loop on every value in eigen.include_paths from
        // above?
        .flag("-I")
        .flag("/usr/include/eigen3")
        .flag_if_supported("-std=c++14")
        .compile("ceres_example");

    println!("cargo:rustc-link-lib=ceres");
    println!("cargo:rustc-link-lib=glog");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/ceres_example.cc");
    println!("cargo:rerun-if-changed=include/ceres_example.h");
}

