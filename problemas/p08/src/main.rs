use util::*;

fn main() {
    print("Ingresa un día de la semana (1-7): ");
    let dia = read_u8();

    let nombre_dia = match dia {
        1 => "Lunes",
        2 => "Martes",
        3 => "Miércoles",
        4 => "Jueves",
        5 => "Viernes",
        6 => "Sábado",
        7 => "Domingo",
        _ => {
            print("Día inválido. Por favor ingresa un número entre 1 y 7.");
            return;
        }
    };

    println!("El día correspondiente es: {nombre_dia}");
}
