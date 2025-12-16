use util::*;

fn main() {
    const MONTO_FIJO: f32 = 800.0;
    print("Cuantos kilometros recorriste? ");
    let km: f32 = read_f32();

    if km <= 300.0 {
        println!("Tendras que pagar {MONTO_FIJO}");
        return;
    }

    if km <= 1000.0 {
        let diferencia = km - 300.0;
        let precio = 15.0 * diferencia;
        println!("Tendras que pagar ${}", MONTO_FIJO + precio);
        return;
    }

    let diferencia = km - 300.0;
    let precio = 12.50 * diferencia;
    println!("Tendras que pagar ${}", MONTO_FIJO + precio);
}
