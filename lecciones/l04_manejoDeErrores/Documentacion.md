# Manejo de errores

## Macro panic
El macro de panic hace que el programa termine por completo y muestra un mensaje en terminal. Se utiliza cuando no se puede manejar un error.

## Option y Result explicados por Google Gemini
Aquí tienes el resumen técnico definitivo.

En Rust, `Option` y `Result` son los mecanismos estándar para gestionar la **seguridad de tipos** en situaciones donde un valor no está garantizado. Ambos son `enums` (enumeraciones) que encapsulan datos, obligando al programador a gestionar todos los posibles estados del flujo de ejecución en tiempo de compilación.

-----

### 1\. `Option<T>`: Gestión de Ausencia (Null Safety)

**Definición Técnica:**
Es un enum utilizado para representar un valor que **puede existir o no**. Rust no admite punteros nulos (`null`). Si un dato es opcional, debe envolverse explícitamente en un `Option`.

**Variantes:**

  * `Some(T)`: Contiene el valor de tipo `T`.
  * `None`: Representa la ausencia total de valor (estado vacío).

**Caso de Uso Real:**
Búsqueda en bases de datos o estructuras de datos. Si buscas un registro por ID, es un estado válido que el registro no exista. No es un error del sistema, es simplemente una ausencia de información.

```rust
// Ejemplo: Buscar un usuario en una "base de datos" (vector)
fn buscar_usuario(id: i32) -> Option<String> {
    let db = vec![(1, "Alice"), (2, "Bob")];
    
    for (user_id, name) in db {
        if user_id == id {
            return Some(name.to_string()); // Éxito: Retorna el valor envuelto
        }
    }
    None // Ausencia: No se encontró, pero no es un fallo crítico
}

// Consumo
let usuario = buscar_usuario(99); // Tipo: Option<String>
```

-----

### 2\. `Result<T, E>`: Gestión de Fallos (Error Handling)

**Definición Técnica:**
Es un enum utilizado para operaciones que **pueden fallar** debido a factores externos o lógicos. Reemplaza al mecanismo de Excepciones (`try-catch`) de otros lenguajes, tratando los errores como valores de primera clase que deben ser propagados o manejados.

**Variantes:**

  * `Ok(T)`: La operación fue exitosa y contiene el resultado `T`.
  * `Err(E)`: La operación falló y contiene el detalle del error de tipo `E`.

**Caso de Uso Real:**
Operaciones de Entrada/Salida (I/O), parseo de datos o peticiones de red. Aquí necesitamos saber *por qué* falló para tomar una decisión (reintentar, loguear el error, cerrar el programa).

```rust
use std::fs::File;
use std::io::Error;

// Ejemplo: Intentar abrir un archivo de configuración
// Retorna: O el archivo (File) O un error de sistema (Error)
fn abrir_config() -> Result<File, Error> {
    let ruta = "config.ini";
    File::open(ruta) // Esta función nativa retorna un Result
}

// Consumo
let resultado = abrir_config(); // Tipo: Result<File, Error>
```

-----

### Tabla Comparativa Técnica

| Característica | `Option<T>` | `Result<T, E>` |
| :--- | :--- | :--- |
| **Semántica** | **Existencia**: ¿Hay dato o no? | **Éxito**: ¿Funcionó o falló? |
| **Estado Negativo** | `None` (Vacío, sin información extra). | `Err(E)` (Fallo, con información del error). |
| **Equivalente Java** | `null` (pero seguro). | `throw Exception` (pero explícito). |
| **Escenario Típico** | Parámetros opcionales, búsquedas. | Archivos, Red, Conversiones de tipo. |

-----

### Mecanismo de Extracción: `match`

Independientemente de si usas `Option` o `Result`, el dato real (`T`) está "atrapado" dentro del enum. Para usarlo, debes extraerlo. La forma idiomática y segura es el **Pattern Matching**.

```rust
// Supongamos que tenemos una variable 'resultado' que es Result<i32, String>

match resultado {
    // Rama de éxito: Extraemos el valor a la variable 'valor'
    Ok(valor) => {
        println!("Operación exitosa: {}", valor);
        // Aquí 'valor' es un i32 puro y usable.
    },
    
    // Rama de fallo: Extraemos el error a la variable 'e'
    Err(e) => {
        eprintln!("Error crítico: {}", e);
        // Aquí manejamos el error (log, exit, etc.)
    }
}
```

**Resumen en una frase:**
Usa `Option` cuando la falta de un valor es una posibilidad normal del negocio (ej. "sin segundo nombre"); usa `Result` cuando la falta de un valor se debe a que algo se rompió o salió mal en el proceso (ej. "disco lleno").

## Sentencias `if let` y `let else`
Fueron creadas para facilitar el uso de match en casos que se necesita un valor específico. `if let` se usa cuando interesa el caso positivo, es decir, existe un `Some(v)` o un `Ok(v)`, y el caso negativo no se hace nada al respecto. `let else` es para el caso contrario, cuando se requiere de tomar acción en caso de algun error o un valor `None`.    
```rust
enum Accion {
    Caminar,
    Saltar,
    Disparar(u8), // Trae cuántas balas disparó
}

fn main() {
    let movimiento = Accion::Disparar(5);

    // CASO GENÉRICO: "Si la acción es Disparar, dame las balas y ejecuta".
    // Cualquier otra acción (Caminar, Saltar) se ignora automáticamente.
    if let Accion::Disparar(balas) = movimiento {
        println!("¡Pum! Disparaste {} veces.", balas);
    }
}
```

```rust
use std::io;

// Esta función es una "máquina genérica" de pedir números.
// Recibe un mensaje personalizado y devuelve un número validado.
fn pedir_entero(mensaje: &str) -> i32 {
    loop {
        // 1. Mostrar el mensaje (ej: "Dame el inicio", "Dame el fin")
        println!("{}", mensaje);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer");

        // 2. EL GUARDIÁN (let else)
        // Intenta convertir (parse). 
        // Si falla (el 'else'), ejecuta el bloque de rechazo.
        let Ok(numero) = input.trim().parse::<i32>() else {
            println!("❌ Error: '{}' no es un número entero. Intenta de nuevo.\n", input.trim());
            
            // 3. EL REINICIO (continue)
            // Ignora todo lo que haya abajo y salta al inicio del loop.
            continue; 
        };

        // 4. EL ÉXITO (break / return)
        // Si llegamos aquí, el 'let else' no se activó, así que 'numero' es válido.
        // Rompemos el ciclo y entregamos el valor.
        return numero; // 'return' rompe el loop y la función al mismo tiempo.
    }
}

fn main() {
    // Mira qué limpio queda el código principal ahora:
    
    println!("--- CONFIGURACIÓN ---");
    
    // Reutilizamos la lógica genérica 3 veces
    let inicio = pedir_entero("Ingresa el valor inicial:");
    let fin    = pedir_entero("Ingresa el valor final:");
    let inc    = pedir_entero("Ingresa el incremento:");

    println!("\n✅ Configuración exitosa: De {} a {} saltando de {} en {}.", inicio, fin, inc, inc);
}
```
> *Codigo generado por gemini*
