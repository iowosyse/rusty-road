use util::*;

fn main() {
    print("Ingresa un número entero positivo: ");
    let n: u32 = read_u32();
    let mut factorial: u64 = 1;

    if n <= 1 {
        println!("El factorial de {} es 1", n);
        return;
    }

    for i in 1..=n {
        factorial *= i as u64;
    }

    println!("El factorial de {} es {}", n, factorial);
}
