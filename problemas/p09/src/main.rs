use util::*;

fn main() {
    println!("Zignos zodiacales: \n
    1. Acuario: 20 de enero - 18 de febrero \n
    2. Piscis: 19 de febrero - 20 de marzo \n
    3. Aries: 21 de marzo - 19 de abril \n
    4. Tauro: 20 de abril - 20 de mayo \n
    5. Géminis: 21 de mayo - 20 de junio \n
    6. Cáncer: 21 de junio - 22 de julio \n
    7. Leo: 23 de julio - 22 de agosto \n
    8. Virgo: 23 de agosto - 22 de septiembre \n
    9. Libra: 23 de septiembre - 22 de octubre \n
    10. Escorpio: 23 de octubre - 21 de noviembre \n
    11. Sagitario: 22 de noviembre - 21 de diciembre \n
    12. Capricornio: 22 de diciembre - 19 de enero");
    
    print("Ingresa un número del 1 al 12 para conocer tu signo zodiacal: ");
    let signo: u32 = read_u32();

    match signo {
        1 => println!("Tu signo zodiacal es de elemento Aire: Acuario."),
        2 => println!("Tu signo zodiacal es de elemento Agua: Piscis."),
        3 => println!("Tu signo zodiacal es de elemento Fuego: Aries."),
        4 => println!("Tu signo zodiacal es de elemento Tierra: Tauro."),
        5 => println!("Tu signo zodiacal es de elemento Aire: Géminis."),
        6 => println!("Tu signo zodiacal es de elemento Agua: Cáncer."),
        7 => println!("Tu signo zodiacal es de elemento Fuego: Leo."),
        8 => println!("Tu signo zodiacal es de elemento Tierra: Virgo."),
        9 => println!("Tu signo zodiacal es de elemento Aire: Libra."),
        10 => println!("Tu signo zodiacal es de elemento Agua: Escorpio."),
        11 => println!("Tu signo zodiacal es de elemento Fuego: Sagitario."),
        12 => println!("Tu signo zodiacal es de elemento Tierra: Capricornio."),
        _ => println!("Número inválido. Por favor ingresa un número del 1 al 12."),
    }
}
