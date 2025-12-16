use util::*;

fn main() {
    print("Cual fue el importe total? ");
    let importe: f32 = read_f32();

    print("Que bolita sacaste? ");
    let bolita: char = read_char();

    let descuento = match bolita {
        'a' => 0.2,
        'r' => 0.3,
        'b' => 0.0,
        _ => 0.0,
    };

    let costo = (1.0 - descuento) * importe;

    println!("El costo total es: ${costo}");
}
