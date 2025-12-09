# Tipos de dato, variables y métodos
Las variables en Rust son inmutables naturalmente, lo que quiere decir que su valor no cambiará por nada en el mundo. Para poder modificar el valor de una variable durante la ejecución se debe añadir una palabra restringida extra:
```rust
let numero = 3;
```
Ahora mismo, la variable `numero` no puede cambiar su valor, por lo que si se intentase hacer algo como
```rust
numero += 1;
```
Se obtendría un error de compilación, por lo que se modifica la declaración de la variable de la siguiente manera:
```rust
let mut numero = 3;
```
Con eso es suficiente para modificar su valor con lineas como la anterior.

## Tipos de variables:
### Tipos de dato primitivo
#### Integers
##### Enteros con signo
Estos tienen un rango de `-(2^(n-1))` hasta `2^(n-1) - 1`. El mínimo valor para `n` es `8`, se declara de la siguiente manera:
```rust
let i0: i8 = 3;
```
A partir de ahí, se multiplica `n` por 2, hasta llegar a 128. Adicionalmente, hay un tipo de entero que depende de la arquitectura de la computadora, siendo este `isize`.

##### Enteros sin signo
Funcionan muy similar a los enteros con signo, pero su rango va desde 0 hasta `2^n - 1`, por lo tanto, una variable del tipo `u8` tiene un rango de 0 hasta 255.
La forma de continuar con la numeración de `n` es igual a la de los enteros con signo, incluyendo también el `usize`.
Ejemplo:
```rust
let enteroPositivo: u32 = 3;
```

#### Flotantes
Los números de punto flotante siguen una sintaxis similar a los enteros, siendo esta la siguiente:
```rust
let f0: f32 = 0.01;
let f1: f64 = 0.01;
```
#### Booleanos
Los booleanos de toda la vida:
```rust
let bandera: bool = true;
```
#### Caracteres
Pueden ser cualquier valor UNICODE:
```rust
let caracter: char = 'c';
```
#### Conversiones
Similar al concepto llamado "casteo" en java, permite convertir una variable a un tipo compatible, por ejemplo:
```rust
let n1: i32 = 100;
let n2: u32 = 1;
let resultado: u32 = n2 + (n1 as u32);
// resultado = 101
```
### Tipos de dato compuesto
#### Tuplas:
Las tuplas son conjuntos de datos que pueden almacenar diferentes tipos de datos, por ejemplo:
```rust
let t: (bool, u32, char) = (true, 1, 'c');
```
Se puede tener tuplas con variables, a las cuales se les asignará el valor correspondiente si se asigna a una tupla ya existente:
```rust
let (a, b, c) = t;
```
En caso de que no se necesiten valores de una tupla, se pueden omitir con `_`:
```rust
let (_, b, _) = t;
```
Las tuplas vacías se pueden comparar con el tipo ```void``` de otros lenguajes de programación, en donde existen métodos donde no se requiere devolver algo, se devolvería una tupla vacía en Rust:
```rust
let t = ();
```
Las tuplas también pueden ser anidadas, y para obtener un valor en específico de las tuplas basta con indicar el índice del valor que se requiere.

#### Arrays
##### Arrays
Tienen una longitud física que se conoce durante el tiempo de compilado, por ejemplo:
```rust
let arr: [u32; 3] = [0, 1, 2];
```
Para retornar un valor requerido la sintaxis es igual a la de java.

Para inicializar un arreglo con los mismos valores en todas sus celdas existe una sintaxis especial para eso:
```rust
let arr: [u32; 10] = [0; 10];
```
Adicionalmente, no es necesario utilizar un bucle para imprimir todos los elementos del arreglo, con usar uno de los [prints especiales](/lecciones/l01_helloWorld/Documentacion.md) basta:
```rust
println!("{:?}", arr);
```
##### Slices
Su longitud es desconocida durante el tiempo de compilado.
Más específicamente son segmentos de un arreglo, o en otras palabras, un subarreglo.
```rust
let nums: [u32; 5] = [0, 1, 2, 3, 4];
//Sub arreglo:
let s = &nums[0..3];
```
La última línea es el `Slice` requerido, obtiene una referencia del arreglo que necesita, y obtiene los índices del 0 al 2, no incluye el límite superior.
Si se quiere obtener desde el inicio hasta un índice específico se puede omitir el límite inferior, y ocurre de la misma manera si se quiere obtener desde un índice específico hasta el final:
```rust
let s = &nums[..3]; //obtiene del 0 al 2
let s = &nums[3..}]; //obtiene del 3 hasta el final
```
Algo que me parece chistoso es que se puede obtener una slice de todos los elementos:
```rust
let s = &nums[..];
```
#### Cadenas
##### Strings
Las cadenas son vectores del tipo u8: `Vec<u8>` y muestran cualquier caracter válido en UTF-8. Un vector es comparable con un `ArrayList` de java.
Las cadenas pueden ser usadas cuando se hagan cambios en la información
Para declarar una cadena se hace con la siguiente linea:
```rust
let msg: String = String::from("");
```
El valor de la longitud de una cadena es del tipo `usize` y se obtiene con el método `len()`.
##### String slices
Los string slices son para uso exclusivo de lectura, son referencias a otra cadena y son inmutables.

Se pueden declarar String slices dentro del código, los cuales utilizan literales de cadena. Se almacenan en los biarios y su apuntador apunta a una parte específica del binario, y siguen siendo inmutables, pues fueron hard-codeadas a los binarios.

Igualmente, se pueden tener literales de cadena que ocupen varias líneas:
```rust
let s: &str = r#"
    Hola, esto es
    una literal de cadena
    multilinea
"#
```

#### ENUMs
Los enums son en principio como los de java, pero se pueden agregar valores que reciben parámetros, y para usarlo es de manera similar al String:
```rust
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl {h: u8, s: u8, l: u8} //se puede declarar como Struct también

    let color0: Color = Color::Red;
    let color1: Color = Color::Rgba(167, 47, 86);
}
```

#### Sructs
Se pueden comparar vagamente con los objetos de Java, pero les encuentro más similitud con los mismos `Structs` de `C`. La sintaxis es la siguiente:

```rust
struct Point {
    x: f32,
    y: f32,
}

//Un struct puede tener como "atributo" a otro
struct Circle {
    center: Point,
    radius: u32,
}
```

Para crear un struct compuesto, símplemente se repite la sintaxis al crear un struct normal:
```rust
let p = {X: 0.0, y: 0.0}; //Struct normal
let circle = Circle {
    center = Point {x: 0.0, y: 0.0},
    radius = 5,
}
```
Es necesario especificar el valor que se está agregando y no se lee en automático por la posición, si se quisiera leer por posición se crea un struct con sintaxis medio de tupla, y con eso sale como si fuese un objeto de java:
```rust
struct tupleStruct(u8, u8, u8);
```
En ese caso, al introducir los valores, basta con ponerlos en el orden correcto y no se necesita especificar cuál es.   
Debería ser evidente, pero se pueden crear variables mutables de tipo struct.   
Algo que **no** es evidente es que si una variable tiene el mismo nombre que un atributo de struct, no hace falta indicar a qué campo se asigna, ahí es una versión mejorada de leer los valores por posición.

##### Funciones de structs
Ahora sí, a diferencia de los `Structs` de C, los de rust si pueden tener métodos propios, similar a un objeto de java.
```rust
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn zero() -> Self {
        Self {x: 0.0, y: 0.0}
    }

    fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

    fn distance(&self) -> f32 {
        //pitágoras
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
```

Aquí hay varias cosas curiosas por notar. Aquellos métodos que no reciben una referencia de `self` son los que se pueden llamar "constructores". Al principio me pareció confuso, pero la manera en que un método define qué tipo de dato va a retornar  es similar a como en python, se utiliza `->` seguido del tipo de dato que se retorna. Se puede aprecar la diferencia en los métodos `new()` y `distance()`.    

Y, similar a python nuevamente, cuando es un método que se aplica sobre una instancia y no para crear una nueva, se debe declarar con un argumento que haga referencia a sí mismo. De hecho, ese argumento es una referencia al mismo objeto (como un puntero).