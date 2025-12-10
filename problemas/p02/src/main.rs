use std::io::{self, Write};

fn main() {
    let mut nums: [i32; 3] = [0; 3];
    
    let mut input: String = String::new();

    //leer los números
    nums[0] = loop {
        print!("Ingrese la primera cifra: ");
        io::stdout().flush().unwrap(); // Asegura que el prompt se muestre antes de leer la entrada
        
        input.clear(); 
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim().parse() {
            Ok(num) => break num, 
            Err(_) => {
                println!("Por favor, ingrese un número válido.");
                continue;
            }
        };
    };

    nums[1] = loop {
        print!("Ingrese la segunda cifra: ");
        io::stdout().flush().unwrap(); // Asegura que el prompt se muestre antes de leer la entrada
        
        input.clear(); 
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim().parse() {
            Ok(num) => break num, 
            Err(_) => {
                println!("Por favor, ingrese un número válido.");
                continue;
            }
        };
    };

    nums[2] = loop {
        print!("Ingrese la tercer cifra: ");
        io::stdout().flush().unwrap(); // Asegura que el prompt se muestre antes de leer la entrada
        
        input.clear(); 
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim().parse() {
            Ok(num) => break num, 
            Err(_) => {
                println!("Por favor, ingrese un número válido.");
                continue;
            }
        };
    };

    nums.sort();
    for n in nums.iter() {
        println!("{}", n);
    }
}
