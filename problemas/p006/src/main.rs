use util::*;

fn main() {
    print("Ingresa el primer numero: ");
    let num1 = read_i32();

    print("Ingresa el segundo numero: ");
    let num2 = read_i32();

    if num1 % num2 == 0 {
        println!("Es división exacta")
    } else {
        println!("No es división exacta")
    }
}
