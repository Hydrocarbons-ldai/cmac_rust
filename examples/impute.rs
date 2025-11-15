use cmac_rust::Cmac;
use std::f64;

fn main() {
    let data = vec![1.0, f64::NAN, 3.0, f64::NAN, 5.0];
    let mut cmac = Cmac::new(data, 2);

    let filled = cmac.impute(0.5, 4);

    println!("Imputed series: {:?}", filled);
}
