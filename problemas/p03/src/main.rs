use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    let numero: i32 = loop {
        print!("Introduce un número entero: ");
        io::stdout().flush().unwrap(); 
        input.clear();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Entrada inválida. Por favor, introduce un número entero válido.");
                continue;
            }
        }
    };

    if numero < 0 {
        println!("El número {} es negativo.", numero);
    } else if numero > 0 {
        println!("El número {} es positivo.", numero);
    } else {
        println!("El número es cero.");
    }
}
