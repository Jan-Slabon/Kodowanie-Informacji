mod golomb;
mod Fibbonaci;
use crate::Fibbonaci::{fibbonaci_code, fibbonaci_decode};

fn main() {
    let code = fibbonaci_code(34);
    code.iter().for_each(|x| {print!("{} ", x)});
    let original = fibbonaci_decode(code);
    println!("\n{}", original);
}
