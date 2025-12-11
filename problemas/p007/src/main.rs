use util::*;

fn main() {
    print("Ingresa el año actual: ");
    let a1 = read_u32();

    print("Ingresa otro año: ");
    let a2 = read_u32();

    if a1 < a2 {
        println!("Faltan {} años para el año {}", a2 - a1, a2);
    } else if a1 > a2 {
        println!("Han pasado {} años desde el año {}", a1 - a2, a2);
    } else {
        println!("Estamos en el año {}", a1);
    }
    
}
