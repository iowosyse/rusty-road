use util::*;

fn main() {
    print("Ingresa un número: ");
    let numero: i32 = read_i32();
    
    if numero % 2 == 0 {
        println!("El número elevado al cuadrado es: {}", numero * numero);
    } else {
        println!("El número elevado al cubo es: {}", numero * numero * numero);
    }
}
