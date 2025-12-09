# Condicionales y ciclos

## Condicionales 1
En Rust exsite el `if` de toda la vida. A diferencia de java, en Rust la condición no necesita estar dentro de paréntesis.  

## Expresiones
No se muy bien como explicarlo, pero en Rust exsite cierto tipo de sintaxis que cuenta como "expresión". Las expresiones son el equivalente a un `return` en cualquier otro lenguaje, pero este queda implícito y la línea se escribe sin `;`.  
Los condicionales pueden tener expresiones dentro de ellos, y por lo tanto, se puede asignar el valor de una variable al resultado de un condicional, como si fuese un operador ternario:
```rust
let x: u32 = if true { 10 };
```

## Condicionales 2
Un condicional normal en todos los lenguajes se abre y cierra con `{}`, que es el caso de Rust también, pero cuando se usa el condicional para definir el valor de una variable, se debe terminar con `;` esta vez. 
Y claro, también existe el bloque `else if` y el `else`, que se pueden aplicar igualmente a la asignación de valor de una variable.
### match
Match es el equivalente a un switch case() de otros lengajes, y su sintaxis es la siguiente:
```rust
let x = 3;
match x {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
}
```
El caso por default se puede utilizar con un bloque `else` o con un `_`.
```rust
let x = 3;
match x {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("default"),
}
```
Se puede hacer match para multiples casos:
```rust
match x {
    1 | 2 | 3 => println!("numero"),
    _ => println!("default"),
}
```
Si se quiere matchear en un rango más grande se utiliza la sintaxis con `..`. Si se quiere añadir al extremo superior se cambia a  `..=`. En caso de que se quiera saber exactamente que opción se matcheó se puede usar `@`:
```rust
match x {
        i @ 1..=10 => println!("@ {i}"),
        _ => println!("other"),
    }
```
Al igual que los ciclos, se puede retornar un valor a partir de un `match`:
```rust
enum Animal {
    Cat,
    Dog,
    Duck,
    Mouse,
}

let animal = Animal::Cat;
    let animal_sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        Animal::Duck => "quack",
        _ => "?",
    };
    println!("{}", animal_sound);
```

## Ciclos
### loop
El bloque `loop` equivale a poner un `while (true)` en cualquier otro lenguaje: es un ciclo sin fin. La forma de salir del ciclo es poner condicionales dentro del bloque y utilizar `break;` cuando se cumpla la condicion.

### While
```rust
while i <= 5 {
    println!("loop {}", i);
    i ++;
}
```

### for y foreach
```rust
for i in 0..5 { //ciclo for normal
    println!("{}", arr[i]);
}

let arr = [1, 2, 3]
for i in arr { //ciclo con array, for each
    println!("{}", i); // equivalente a imprimir arr[i]
}

let v = vec! [1, 2, 3];
for x in v {//for each 
    println!("{}", x);
}

n = arr.len();
for i in 0..n { //ciclo obteniendo un valor límite. Se usa la sintaxis del for normal.
    println!("{}", arr[i]);
}
```
Se incrementa de 1 en 1. Para Los límites se utiliza la sintaxis con dos puntos seguidos: `a..b`. Si en lugar de poner un intervalo se utiliza un arreglo o un vector, equivale a un foreach en java.   
Como el manejo de memoria de Rust es distinto, el concepto de pertenencia causa problemas al querer iterar un vector más de una vez, por lo que se necesita aplicar el método `.iter()` al vector en cada declaración de ciclo.

#### Retornar valores con ciclos
Esto es posible utilizando el ciclo `loop{}` únicamente 
```rust
let mut i: u32 = 0;
    let v = loop {
        i += 1;
        if i > 3 {
            break "i > 3";
        }
    };
```
Se retorna el valor posterior a `break`.

### Etiquetas
```rust
let mut i = 0;
    'outer: for i in 0..3 {
        'inner: for j in 0..3 {
            println!("{i}, {j}");
            if i == 1 && j == 1 {
                break 'outer;
            }
        }
    }
```
Las etiquetas permiten realizar operaciones en ciclos ajenos en el que nos encontramos, permitiendo salir de bloques de los cuales ya satisfacimos una condición arbitraria. Lo podría comparar a utilizar un return antes de tiempo en un método cuando no se cumple una condición crítica.