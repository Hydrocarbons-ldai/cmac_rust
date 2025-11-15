use cmac_rust::Cmac;
use std::f64;

fn main() {
    let arr = vec![1.0, f64::NAN, 3.0, f64::NAN, 5.0];
    let genp = 2;

    let mut cmac = Cmac::new(arr, genp);
    let imputed = cmac.impute(0.5, 4);

    println!("Заполненный ряд: {:?}", imputed);
}
