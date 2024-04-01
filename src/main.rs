use std::io;

fn main() {
    println!("Temp Conv");

    loop {
        println!("1. C to F");
        println!("2. F to C");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        let choice: u32 = choice.trim().parse().expect("Enter Number:");

        if choice == 1 {
            println!("Enter temp in C:");
            let mut celsius = String::new();
            io::stdin().read_line(&mut celsius).expect("Failed");
            let celsius: f64 = celsius.trim().parse().expect("Enter Number:");
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            println!("Temp in F: {:.2}", fahrenheit);
        } else if choice == 2 {
            println!("Enter temp in F:");
            let mut fahrenheit = String::new();
            io::stdin().read_line(&mut fahrenheit).expect("Failed");
            let fahrenheit: f64 = fahrenheit.trim().parse().expect("Enter Number:");
            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
            println!("Temp in C: {:.2}", celsius);
        } else if choice == 3 {
            break;
        } else {
            println!("Invalid");
        }
    }
}
