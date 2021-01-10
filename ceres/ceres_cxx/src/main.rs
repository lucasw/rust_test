use cxx::CxxVector;

#[cxx::bridge(namespace = "org::ceres_example")]
mod ffi {

    // TODO(lucasw) Convert to and from the ceres::Jet, provide all Jet operations in native rust
    // also?
    // See ceres-solver/include/ceres/jet.h
    // struct RustJet {
    //   a: f64,
    //   TODO(lucasw) don't use a Vec, instead a fixed array based on ceres::Jet v size.
    //   v: Vec<f64>,
    // }

    extern "Rust" {
        // "extern function with generic parameters is not supported yet"
        // fn evaluate<T>(val: T) -> T;
        fn evaluate(val: f64) -> f64;
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("ceres_cxx/include/ceres_example.h");

        type CeresExample;

        fn new_ceres_example() -> UniquePtr<CeresExample>;

        // fn run<T>(&self, vals: &Vec<T>);
        fn run(&self, vals: &Vec<f64>);
    }
}

pub fn evaluate(val: f64) -> f64 {
    let residual = 12.3 - val;
    residual
}

fn main() {
    let ceres_example = ffi::new_ceres_example();
    // is it possible to create a CxxVector on the rust side?  There isn't a CxxVector::new()
    // let vals = CxxVector;  // ::<f64>();
    let mut vals = Vec::new();
    vals.push(5.342);
    vals.push(8.0);
    ceres_example.run(&vals);
}
