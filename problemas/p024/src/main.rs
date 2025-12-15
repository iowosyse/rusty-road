use util::*;

fn main() {
    print("Cuál es tu edad? ");
    let edad: u32 = read_u32();

    print("Cuál es tu semestre? ");
    let _: u32 = read_u32();

    let costo_semestre: u32 = 3200;
    println!("El monto del semestre es: ${}", costo_semestre);
    if edad <= 18 {
        println!("No tienes decuento");
        return;
    }

    let descueto = 1.5 * (edad as f32 - 18.0);
    let monto_descuento = (descueto / 100.0) * (costo_semestre as f32);
    let monto_a_pagar = (costo_semestre as f32) - monto_descuento;
    
    println!("Tienes un descuento de: ${:.2}", monto_descuento);
    println!("El monto a pagar es: ${:.2}", monto_a_pagar);
}
