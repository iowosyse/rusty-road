use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Ingresa un número: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let n1: i32 = input.trim().parse().expect("Por favor ingresa un número válido");

    print!("Ingresa otro número: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let n2: i32 = input.trim().parse().expect("Por favor ingresa un número válido");

    if n1 > n2 {
        println!("El número mayor es el primero que ingresó: {}", n1);
    } else if n2 > n1 {
        println!("El número mayor es el segundo que ingresó: {}", n2);
    } else {
        println!("Ambos números son iguales: {}", n1);  
    }
}
