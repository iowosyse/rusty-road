use util::*;

fn main() {
    print("Ingrese un número entero positivo: ");
    let mut n: u64 = read_u64();

    print("Serie de Ulam: ");
    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }

    println!("1");
}
