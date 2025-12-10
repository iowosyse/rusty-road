# Entrada de datos por terminal

Para poder leer datos desde terminal es necesario importar la libreria `std::io`, que es la que se encarga de las entradas y salidas. El método para leer el teclado es `read_line()`, el cual devuelve un `Result<T, E>`, por lo que se agrega el método `expect()` que se encarga del manejo del error (termina el proceso)

```rust
use std::io; //libreria estandar para entrada y salida

fn main() {
    let mut nombre: String = String::new(); //crea una variable mutable de tipo String
    println!("Por favor, ingresa tu nombre:"); //imprime en pantalla un mensaje
    io::stdin() //obtiene la entrada estandar (teclado)
        .read_line(&mut nombre) //lee una linea y la guarda en la variable nombre
        .expect("Error al leer la linea"); //manejo de errores

    println!("Hola, {}!", nombre.trim()); //imprime un saludo con el nombre ingresado, trim() elimina espacios en blanco
}
```