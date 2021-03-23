use std::io;
//Accepts user input

fn to_farenheit (temp: u32) -> u32 {

    (temp * 9/5) + 32

}

fn to_celcius (temp: u32) -> u32 {

    (temp - 32) * 5/9

}

fn main() {

    let mut temp = String::new();
    let mut conversion = String::new();

    println!("Welcome to the temperature conversion program!");

    println!("Please enter a temperature to convert.");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");

    println!("Do you wish to convert {} to celcius or farenheit?", temp);

    io::stdin()
        .read_line(&mut conversion)
        .expect("Failed to read line.");

    let conversion = conversion
    .trim()
    .to_lowercase();

    let temp: u32 = temp
    .trim()
    .parse()
    .expect("Failed to read line.");

    if conversion == "farenheit" {
        let val = to_farenheit(temp).to_string();
        println!("{} degrees in {} is {} degrees", temp, conversion, val)
    } else if conversion == "celcius" {
        let val = to_celcius(temp).to_string();
        println!("{} degrees in {} is {} degrees", temp, conversion, val)
    } else {
        println!("Please enter either farenheit or celcius.");
    }
}
