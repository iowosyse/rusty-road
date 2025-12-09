# Hello, world!
Este es un simple y sencillo `print`, se utiliza la sintaxis
```rust 
println!();
```
Imprimirá en consola todo argumento que se pase dentro de los paréntesis.   
Existen diferentes maneras de pasar argumentos al print:
```rust
let string: &str = "nombre";
let numero: i32 = 2;

//Similar a un printf en C
println!("Hola {}", nombre);

//Similar a un System.out.println("Hola" + nombre); de java
print!("Hola {nombre}");

//Print con argumentos posicionales
println!("{0} * {0} = {1}", x, x * x);
```
También existen sintaxis que permiten usar herramientas de debug para tipos de datos personalizados, sean Structs o ENUM:  
```rust
print!("{:?}", argumento);
print!("{:#?}", argumento);
```