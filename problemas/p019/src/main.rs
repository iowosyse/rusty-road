use util::*;

fn main() {
    print("Ingrsa la cantidad inicial: ");
    let cantidad_inicial: f64 = read_f64();
    let mut pasos = cantidad_inicial;

    print("Ingresa la tasa de interes anual (en porcentaje): ");
    let tasa_interes_anual: f64 = read_f64();

    print("Ingresa el numero de años: ");
    let años: u32 = read_u32();
    let tasa_interes_decimal = tasa_interes_anual / 100.0;

    for i in 1 ..= años {
        let cantidad = pasos * (1.0 + tasa_interes_decimal);
        println!("Después del año {}: {:.2}", i, cantidad);
        pasos = cantidad
    }
}
