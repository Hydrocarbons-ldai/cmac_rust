mod cmac;            // говорим компилятору "подключи модуль из файла cmac.rs"

use crate::cmac::CMAC;  // импортируем сам тип CMAC

fn main() {
    let arr = vec![1.0, f64::NAN, 3.0, f64::NAN, 5.0];
    let genp = 2;

    let mut cmac = CMAC::new(arr, genp);
    let result = cmac.full_row(0.5, 4);

    println!("Заполненный ряд: {:?}", result);
}
