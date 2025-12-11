use util::*;
use libm::pow; //libm es la liberia para matematicas, importo unicamente el metodo para calcular una potencia

fn main() {
    print("Ingresa la base: ");
    let base = read_f64();

    print("Ingresa el exponente: ");
    let exponente = read_f64();

    let resultado = pow(base, exponente);
    println!("{} elevado a la {} es {}", base, exponente, resultado);
}