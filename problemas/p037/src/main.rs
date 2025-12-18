use util::*;

fn main() {
    print("Ingresa un numero entero positivo: ");
    let num1 = read_u32();

    print("Ingresa otro numero entero positivo: ");
    let num2 = read_u32();

    let d1 = sumar_divisores(num1);
    let d2 = sumar_divisores(num2);

    if d1 == d2 {
        println!("Los numeros son amigos");
    } else {
        println!("Los numeros no son amigos");
    }
}

fn sumar_divisores(n: u32) -> u32 {
    let mut suma = 0;

    for i in n - 1..0 {
        if i % n == 0 {
            suma += i;
        }
    }

    suma
}
