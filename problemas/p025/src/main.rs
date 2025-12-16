use util::*;

fn main() {
    const CUOTA_FIJA: f32 = 300.0;

    print("Cuantos litros de agua fueron? ");
    let litros: u32 = read_u32();

    if litros <= 50 {
        print!("Deberás pagar ${CUOTA_FIJA}");
        return;
    }

    let diferencia: u32 = litros - 50;

    if litros <= 200 {
        let precio: f32 = 1.5 * diferencia as f32;
        println!("Deberas pagar {}", (CUOTA_FIJA + precio));
        return;
    }

    let precio: f32 = 2.08 * diferencia as f32;
    println!("Deberas pagar {}", (CUOTA_FIJA + precio));
}
