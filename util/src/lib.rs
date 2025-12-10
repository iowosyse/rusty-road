use std::io::{self, Write};

pub fn read_string() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string() //la linea que devuelve el input sin espacios
}

pub fn read_i8() -> i8 {
    loop {
        let input = read_string();
        match input.parse::<i8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_i16() -> i16 {
    loop {
        let input = read_string();
        match input.parse::<i16>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_i32() -> i32 {
    loop {
        let input = read_string();
        match input.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_i64() -> i64 {
    loop {
        let input = read_string();
        match input.parse::<i64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_i128() -> i128 {
    loop {
        let input = read_string();
        match input.parse::<i128>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_isize() -> isize {
    loop {
        let input = read_string();
        match input.parse::<isize>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_u8() -> u8 {
    loop {
        let input = read_string();
        match input.parse::<u8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_u16() -> u16 {
    loop {
        let input = read_string();
        match input.parse::<u16>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_u32() -> u32 {
    loop {
        let input = read_string();
        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_u64() -> u64 {
    loop {
        let input = read_string();
        match input.parse::<u64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_u128() -> u128 {
    loop {
        let input = read_string();
        match input.parse::<u128>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_usize() -> usize {
    loop {
        let input = read_string();
        match input.parse::<usize>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_f8() -> f32 {
    loop {
        let input = read_string();
        match input.parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_f16() -> f64 {
    loop {
        let input = read_string();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_f32() -> f32 {
    loop {
        let input = read_string();
        match input.parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_f64() -> f64 {
    loop {
        let input = read_string();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn read_128() -> f64 {
    loop {
        let input = read_string();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn print(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().expect("Failed to flush stdout");
}
