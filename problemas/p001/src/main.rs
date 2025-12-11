use std::io::{self, Write};

fn main() {
    let mut nums: [i32; 3] = [0; 3];
    let mut mayor: i32 = i32::MIN; //inicializar con el valor mínimo posible
    
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

    //buscar el mayor
    for n in nums.iter() {
        if *n > mayor { //elimina lar referencia y se queda con el valor almacenado en la variable n
            mayor = *n;
        }
    }

    println!("El número mayor es: {}", mayor);
}