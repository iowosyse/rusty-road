use util::*;

fn main() {
    print("Ingresa un valor inicial: ");
    let inicio: u32 = read_u32();

    print("Ingresa un valor final: ");
    let fin: u32 = read_u32();

    print("Ingresa un incremento: ");
    let incremento = read_u32();
    
    println!("| Metros | Kilómetros | Millas |");
    println!("|--------|------------|--------|");
    let mut metros = inicio;
    while metros <= fin {
        let kilometros = metros as f64 / 1000.0;
        let millas = kilometros / 1.60934;
        println!("| {:^6} | {:^10.3} | {:^6.3} |", metros, kilometros, millas);
        println!("|--------|------------|--------|");
        metros += incremento;
    }
}
