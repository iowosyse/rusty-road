use util::*;

fn main() {
    print("Cual es la base? ");
    let base = read_u32();

    print("Cual es el exponente? ");
    let exponente = read_u32();

    let mut i: i32 = base as i32;
    let mut sumatoria : i32 = 0;
    
    while i > 0 {
        sumatoria += i % 10;
        i /= 10;
    }

    if sumatoria.pow(exponente) == base as i32{
        println!("El numero es caprichoso en la potencia {exponente}")
    } else {
        println!("El numero no es caprichoso en la potencia {exponente}");
    }
}
