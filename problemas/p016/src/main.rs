use util::*;

fn main() {
    print("Ingresa un numero entero positivo: ");
    let n1 = read_u32();
    print("Ingresa otro numero entero positivo: ");
    let n2 = read_u32();
    let mut result = n1;

    for _ in 1..n2 {
        result += n1;
    }

    println!("El resultado de sumar {} + {} veces es: {}", n1, n2, result);
}
