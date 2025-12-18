use util::*;

fn main() {
    let mut num: u32 = 1000;

    while num > 999 {
        print("Ingresa un numero de 3 cifras: ");
        num = read_u32();
    }

    if num % 10 == (num / 100) % 10 {
        println!("El numero es palindromo");
    } else {
        println!("El numero no es palindromo");
    }
}
