use std::io;

pub fn test_temperatures() {
    let mut temp: f64;
    loop {
        let mut input = String::new();
        println!("Enter temperature: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        temp = match input.trim().parse::<f64>(){
            Ok(temp) => temp,
            Err(_) => continue
        };
        match check_temperature_type() {
            true => println!("The temperature in Fahrenheit is: {} F\n", to_fahrenheit(temp)),
            false => println!("The temperature in Celsius is: {} C\n", to_celsius(temp))
        };
    }
}

fn check_temperature_type() -> bool {
    println!("Is this Celsius?");
    loop {
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read line");
        match a.trim() {
            "yes" => break true,
            "no" => break false,
            _ => {
                println!("Enter yes/no: ");
                continue
            }
        }
    }
}

fn to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}

fn to_fahrenheit(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}