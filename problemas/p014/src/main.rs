use util::*;

fn main() {
    print("Ingresa la base: ");
    let base: f64 = loop {
        break read_f64()
    };

    print("Ingresa el exponente: ");
    let exponente: f64 = loop {
        break read_f64()
    };

    let mut resultado: f64 = base;
    for _ in 1..exponente as i32 {
        resultado *= base;
    }

    println!("{} elevado a {} es {}", base, exponente, resultado);

}
