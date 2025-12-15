use util::*;

fn main() {
    print("Dime cuantos minutos quieres cultivar bacterias: ");
    let minutos = read_u32();

    let mut poblacion = 1;

    println!("|{:^8}|{:^11}|", "Minuto", "Poblacion");
    println!("|{:-^8}|{:-^11}|", "-", "-");

    for i in 0..minutos {
        println!("|{:^8}|{:^11}|", i, poblacion);
        println!("|{:-^8}|{:-^11}|", "-", "-");
        poblacion *= 2;
    }
}
