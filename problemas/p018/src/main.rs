use util::*;

fn main() {
    print("Ingresa un numero inicial: ");
    let inicial = read_i32();

    print("Ingresa un numero final: ");
    let final_ = read_i32();
    let mut sumatoria = 0;

    let mut suma = String::new();

    for i in inicial..=final_ {
        sumatoria += i;
        let _ = if suma.is_empty() {
            suma += &i.to_string();
        } else {
            suma += &format!(" + {}", i);
        };
    }

    println!("{} = {}", suma, sumatoria);
}
