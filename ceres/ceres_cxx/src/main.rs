#[cxx::bridge(namespace = "org::ceres_example")]
mod ffi {

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("ceres_cxx/include/ceres_example.h");

        type CeresExample;

        fn new_ceres_example() -> UniquePtr<CeresExample>;
        fn run(&self);
    }
}

fn main() {
    let ceres_example = ffi::new_ceres_example();
    ceres_example.run();
}
