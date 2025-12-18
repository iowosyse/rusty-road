use util::*;

fn main() {
    let mut unos: u32 = 0;
    let mut ceros: u32 = 0;
    let mut otros: u32 = 0;

    print("Ingresa un numero: ");
    let mut num = read_u32();

    while num > 0 {
        match num % 10 {
            0 => ceros += 1,
            1 => unos += 1,
            _ => otros += 1,
        }

        num /= 10;
    }

    if otros == 0 {
        println!("El numero tiene apariencia de binario");
    } else {
        println!(
            "El numero no tiene apariencia de binario\n
Tiene {} ceros\n
Tiene {} unos\n
Tiene {} numeros diferentes de cero o uno",
            ceros, unos, otros
        );
    }
}
