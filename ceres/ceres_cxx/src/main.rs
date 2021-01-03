use cxx::CxxVector;

#[cxx::bridge(namespace = "org::ceres_example")]
mod ffi {

    extern "Rust" {
        fn evaluate(val: f64) -> f64;
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("ceres_cxx/include/ceres_example.h");

        type CeresExample;

        fn new_ceres_example() -> UniquePtr<CeresExample>;

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
