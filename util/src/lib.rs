use std::io::{self, Write};

pub fn str_in() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string() //la linea que devuelve el input sin espacios
}

pub fn i8_in() -> i8 {
    loop {
        let input = str_in();
        match input.parse::<i8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn i16_in() -> i16 {
    loop {
        let input = str_in();
        match input.parse::<i16>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn i32_in() -> i32 {
    loop {
        let input = str_in();
        match input.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn i64_in() -> i64 {
    loop {
        let input = str_in();
        match input.parse::<i64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn i128_in() -> i128 {
    loop {
        let input = str_in();
        match input.parse::<i128>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn isize_in() -> isize {
    loop {
        let input = str_in();
        match input.parse::<isize>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn u8_in() -> u8 {
    loop {
        let input = str_in();
        match input.parse::<u8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn u16_in() -> u16 {
    loop {
        let input = str_in();
        match input.parse::<u16>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn u32_in() -> u32 {
    loop {
        let input = str_in();
        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn u64_in() -> u64 {
    loop {
        let input = str_in();
        match input.parse::<u64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn u128_in() -> u128 {
    loop {
        let input = str_in();
        match input.parse::<u128>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn usize_in() -> usize {
    loop {
        let input = str_in();
        match input.parse::<usize>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn f8_in() -> f32 {
    loop {
        let input = str_in();
        match input.parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn f16_in() -> f64 {
    loop {
        let input = str_in();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn f32_in() -> f32 {
    loop {
        let input = str_in();
        match input.parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn f64_in() -> f64 {
    loop {
        let input = str_in();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
            }
        }
    }
}

pub fn f128_in() -> f64 {
    loop {
        let input = str_in();
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
