// using modules
// https://levelup.gitconnected.com/easiest-way-to-understand-rust-modules-across-multiple-files-234b5018cbfd
mod util;

fn main() {
    println!("Hello, day01");

    let mut total_fuel_value: f32 = 0.0;

    if let Some(lines) = util::get_lines("./01.data") {
        for line in lines {
            match line.parse::<f32>() {
                Ok(val) => {
                    let fuel_value = ((val / 3.0) - 0.5).round() - 2.0;
                    total_fuel_value = total_fuel_value + fuel_value;
                    println!("{} / {}", val, fuel_value);
                }
                Err(_) => {}
            }
        }
    }
    println!("total fuel value {}", total_fuel_value);
}
