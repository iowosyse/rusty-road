use util::*;

fn main() {
    print("Cual es tu calificacion? ");
    let calificacion: i32 = read_i32();

    if calificacion > 95 {
        println!("Excelente");
    } else if calificacion > 90 {
        println!("Muy bien");
    } else if calificacion > 85 {
        println!("Bien");
    } else if calificacion > 75 {
        println!("Suficiente");
    } else if calificacion > 69{
        println!("Apenas suficiente");
    } else if calificacion > 49 {
        println!("Esfuérzate más")
    } else {
        println!("Toma clases extras");
    }
}
