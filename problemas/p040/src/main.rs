use util::*;

fn main() {
    print("Dame un numero: ");
    let msg = read_string();

    let numero: i32 = match get_num(msg) {
        Ok(n) => n,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    if numero % 2 == 0 {
        println!("El numero es positivo: {}", numero > 0);
        println!("El numero es negativo: {}", numero < 0);
    } else {
        println!("El numero es multiplo de 7: {}", (numero % 7) == 0);
    }
}

//funcion custom para evitar que se ingrese un 0
fn get_num(msg: String) -> Result<i32, String> {
    let num;
    num = match msg.to_string().parse::<i32>() {
        Ok(a) => a,
        Err(_) => 0,
    };

    if num != 0 {
        Ok(num)
    } else {
        Err("La cadena no es un numero o es 0".to_string())
    }
}
