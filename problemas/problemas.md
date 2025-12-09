# Ejercicios de Programación en Rust  
*Ordenados por tema y complejidad*

## 1. Fundamentos y Entrada/Salida

### 1.1. Números y Operaciones Básicas
1. **Leer tres números e indicar el mayor**  
   Solicita tres números al usuario y muestra cuál es el mayor.

2. **Leer tres números e imprimirlos en orden ascendente**  
   Solicita tres números y los muestra ordenados de menor a mayor.

3. **Positivo, negativo o cero**  
   Solicita un número e indica si es positivo, negativo o cero.

4. **Mayor y menor de dos números**  
   Solicita dos números e indica cuál es mayor, cuál es menor, o si son iguales.

5. **Múltiplo de otro**  
   Solicita dos números e indica si el primero es múltiplo del segundo.

6. **División exacta o no**  
   Solicita dos enteros, calcula el cociente e indica si la división es exacta.

7. **Años pasados o futuros**  
   Solicita el año actual y otro año, e indica cuántos años han pasado o faltan.

8. **Día de la semana**  
   Solicita un número del 1 al 7 y muestra el nombre del día correspondiente (1 = domingo, etc.). Si está fuera de rango, muestra error.

9. **Categoría del signo zodiacal**  
   Muestra una lista de los 12 signos del zodíaco con números. Luego solicita un número y muestra la categoría (Fuego, Tierra, Aire, Agua). Si el número no es válido, muestra error.

10. **Mensaje según calificación**  
    Solicita una calificación (0-100) y muestra un mensaje según el rango:  
    - 96-100: "Excelente"  
    - 91-95: "Muy bien"  
    - 86-90: "Bien"  
    - 75-85: "Suficiente"  
    - 70-74: "Apenas suficiente"  
    - 50-69: "Esfuérzate más"  
    - 0-49: "Toma clases extras"

### 1.2. Cálculos Simples
11. **Elevar a potencia según elección**  
    Solicita un número y la potencia a la que se elevará (2, 3 o 4). Muestra el resultado.

12. **Cuadrado o cubo según paridad**  
    Solicita un número. Si es par, muestra su cuadrado; si es impar, muestra su cubo.

13. **Factorial de un número**  
    Solicita un número entero no negativo (N) y calcula su factorial. Recuerda que 0! = 1.

14. **Potencia mediante multiplicaciones sucesivas**  
    Solicita dos números enteros positivos B y P. Calcula B^P mediante multiplicaciones sucesivas (sin usar el operador `^` o `pow`). Valida que B y P sean positivos.

15. **Multiplicación mediante sumas sucesivas**  
    Solicita dos enteros no negativos N y M. Calcula N × M mediante sumas sucesivas, usando el menor número posible de sumas (no usar `*`).

16. **División mediante restas sucesivas**  
    Solicita dos enteros no negativos N y M. Calcula el cociente y residuo de N / M mediante restas sucesivas (no usar `/` ni `%`). Si M es cero, muestra error.

17. **Fibonacci**  
    Solicita la cantidad de términos N y muestra los primeros N términos de la sucesión de Fibonacci (comenzando con 0 y 1).

18. **Suma de términos consecutivos**  
    Solicita un número inicial y un número N. Muestra la suma de los N términos consecutivos a partir del inicial, y la expresión completa (ej: 20+21+22=63).

19. **Interés compuesto hasta duplicar**  
    Solicita una cantidad inicial y una tasa de interés anual. Muestra una tabla con el año, el interés ganado y la cantidad acumulada hasta que la cantidad sea al menos el doble de la inicial.

20. **Reproducción de bacterias**  
    Solicita un tiempo en minutos. Muestra una tabla con la cantidad de bacterias en cada minuto (se duplican cada minuto, comenzando con 1).

21. **Serie de Ulam**  
    Solicita un número entero positivo y genera la serie de Ulam (si es par, divídelo entre 2; si es impar, multiplícalo por 3 y súmale 1) hasta llegar a 1.

22. **Conversión de unidades**  
    Solicita un valor inicial, final y un incremento (en metros). Genera una tabla de conversión de metros a kilómetros y millas (1 km = 1000 m, 1 mi = 1609 m).

23. **Tabla de números triangulares**  
    Solicita un número máximo M. Imprime los números del 1 al M en forma triangular: el primer renglón solo el 1, el segundo dos números, el tercero tres, etc.

## 2. Estructuras Condicionales y de Control

### 2.1. Descuentos y Cálculos de Costos
24. **Descuento por edad en inscripción**  
    Solicita la edad del alumno, el semestre (2-9) y el monto de la inscripción. Aplica un descuento del 1.5% por cada año que exceda la mayoría de edad (18 años). Muestra el monto a pagar.

25. **Cálculo de agua**  
    Solicita los litros de agua consumidos. Calcula el monto a pagar: cuota fija de $300 (incluye primeros 50 litros), $1.5 por litro entre 51 y 200, y $2.08 por litro a partir del 201.

26. **Alquiler de automóviles**  
    Solicita los kilómetros recorridos. Calcula el monto: $800 hasta 300 km; por encima hasta 1000 km, $15 por km extra; más de 1000 km, $12.50 por km extra (siempre a partir del km 300).

27. **Cálculo de impuestos sobre salario**  
    Solicita el salario. Calcula el impuesto:  
    - Hasta $2000: 0%  
    - $2001 a $5000: 2% sobre lo que excede $2000  
    - Más de $5000: $500 + 5% sobre lo que excede $5000  
    Muestra salario bruto, impuesto y neto.

28. **Descuento por bolitas de colores**  
    Solicita el importe total y el color de la bolita ('a'=azul 20%, 'r'=roja 30%, 'b'=blanca 0%). Muestra el monto a pagar.

29. **Descuento por antigüedad en compra de mallas**  
    Solicita los metros de malla plana y de rollo, y los años que el cliente ha comprado cada tipo. Aplica descuento del 6% en malla plana si tiene más de 6 años, y 8% en malla de rollo si tiene más de 5 años. Muestra el descuento total y el monto a pagar.

30. **Estacionamiento con descuento**  
    Solicita los minutos estacionados y el día de la semana. Cobra: hasta 10 minutos gratis; a partir de 11 minutos, cada media hora $15.50, cada hora $20.00 (solo medias horas y horas completas). Los martes y sábados aplica 12.56% de descuento.

31. **Peaje por tipo de vehículo**  
    Solicita el tipo de vehículo ('b'=bicicleta, 'm'=moto, 'a'=auto, 'c'=camión), los kilómetros recorridos y, para camiones, las toneladas de carga. Calcula el importe:  
    - Bicicleta: $0.8/km  
    - Moto/Auto: $3.5/km  
    - Camión: $12.80/km + $12/tonelada

32. **Compra de sillas con descuento**  
    Solicita la cantidad de sillas tipo A, B y C. Aplica descuento por cada 10 sillas: 3% para A, 5% para B, 7% para C. Muestra el total por tipo y el total general.

33. **Compra de madera con descuento**  
    Solicita los metros cúbicos de madera tipo A, B y C. Si el total conjunto supera 30 m³, aplica descuentos individuales: A 4%, B 8%, C 10%. Muestra el total a pagar.

34. **Comisiones de vendedores**  
    Almacena los datos de dos vendedores (nombre y ventas). Calcula la comisión:  
    - Menos de $500: 0.7%  
    - $500 a $999: 1.2%  
    - $1000 a $1500: 2.5%  
    - Más de $1500: 4%  
    Muestra la comisión de cada uno y el nombre del que tuvo mayores ventas.

35. **Cine con obsequios**  
    Solicita la edad, el sexo ('M' o 'F') y la cantidad de visitas previas al cine. Asigna:  
    - Hasta 12 años: boleto para "Bambi", y combo palomitas (niñas) o doritos (niños).  
    - 13 años o más: boleto para "Drácula", y vale de descuento del 30% (si tiene al menos 5 visitas) o 35% (más de 5 visitas).  
    Muestra los obsequios.

### 2.2. Validación y Clasificación
36. **Número caprichoso**  
    Solicita un número entero X y una potencia P. Determina si X es caprichoso en la potencia P, es decir, si la suma de sus dígitos elevada a P es igual a X.

37. **Números amigos**  
    Solicita dos números enteros positivos. Determina si son amigos: la suma de los divisores propios de uno es igual al otro, y viceversa (excluyendo al número mismo).

38. **Palíndromo de 3 cifras**  
    Solicita un número de 3 cifras (validar). Determina si es palíndromo (se lee igual al derecho y al revés).

39. **Validación de número binario**  
    Solicita un número entero. Determina si contiene solo dígitos 0 y 1. Si es así, muestra "Valor tiene apariencia de binario"; sino, muestra cuántos 0, cuántos 1 y cuántos otros dígitos tiene.

40. **Paridad y múltiplo de 7**  
    Solicita un número entero. Si es cero, muestra "No trabajo con valor cero". Si es par, indica si es positivo o negativo. Si es impar, indica si es múltiplo de 7 o no.

## 3. Estructuras de Repetición (Ciclos)

### 3.1. Generación y Cálculo con Números Aleatorios
41. **Números aleatorios impares mayores que N**  
    Solicita un número N. Genera 8 números aleatorios impares mayores que N, muéstralos y su suma.

42. **Números aleatorios en rango**  
    Genera 7 números aleatorios entre 5 y 100. Muestra la suma, el mayor y el menor.

43. **Generar 10 números aleatorios entre 60 y 500**  
    Genera 10 números aleatorios en ese rango. Encuentra el mayor y el menor. Luego divide el mayor entre el menor mediante restas sucesivas (sin usar `/` ni `%`). Muestra cociente y residuo.

### 3.2. Procesamiento de Secuencias de Números
44. **Mayor diferencia entre consecutivos**  
    Solicita números enteros hasta que se ingrese un 0. Encuentra la mayor diferencia absoluta entre dos números consecutivos y muestra esa diferencia y el par de números.

45. **Mayor múltiplo de 7 y menor múltiplo de 3**  
    Solicita números enteros positivos hasta que se ingrese 99 (sin incluirlo). Encuentra el mayor múltiplo de 7 y el menor múltiplo de 3. Si no hay alguno, muestra el mensaje correspondiente.

46. **Números con suma de dígitos pares e impares igual**  
    Solicita dos números N y M (rango). Imprime todos los números en ese rango cuya suma de dígitos pares sea igual a la suma de dígitos impares.

47. **Invertir un número**  
    Solicita un número entero. Inviértelo y almacénalo en otra variable. Muestra el original y el invertido.

48. **Imprimir dígitos de atrás hacia adelante**  
    Solicita un número entero e imprime sus dígitos comenzando por el último.

49. **Imprimir dígitos de adelante hacia atrás**  
    Solicita un número entero e imprime sus dígitos comenzando por el primero.

50. **Suma de dígitos pares e impares**  
    Solicita un número entero. Si la suma de sus dígitos pares es igual a la suma de sus dígitos impares, imprime "YES!".

### 3.3. Cálculos Repetitivos
51. **Productividad anual**  
    Solicita la cantidad de artículos producidos en cada mes (12 meses). Calcula la productividad anual multiplicando cada mes por su factor (Ene-Mar: 15, Abr-Jun: 17, Jul-Ago: 19, Sep-Nov: 20, Dic: 21) y sumando.

52. **Mínimo y máximo común divisor**  
    Fuerza al usuario a ingresar dos valores mayores que cero (validar). Encuentra el mínimo común divisor (que no sea 1) y el máximo común divisor. Si no existen, indícalo.

53. **Promedio ignorando ceros**  
    Solicita un número N, luego N números enteros. Ignora los ceros (no los incluyas en el promedio). Muestra el mayor, el menor y el promedio de los restantes.

54. **Primeros 10 números menores que N**  
    Solicita un número N. Imprime los primeros 10 números enteros menores que N y su suma.

55. **Promedio de calificaciones de alumnos**  
    Solicita el nombre y tres calificaciones de cada alumno hasta que el nombre sea "Adios". Para cada alumno, si el promedio es >=70 y no tiene materias reprobadas (<70), muestra "Felicidades [nombre] aprobaste con un promedio de: [prom]". Si promedio >=70 pero tiene materias reprobadas, muestra "[nombre], tienes promedio de: [prom] pero tienes materias reprobadas". Calcula el promedio del grupo, el porcentaje de aprobados y reprobados.

### 3.4. Menús y Operaciones
56. **Menú de operaciones básicas**  
    Muestra un menú con opciones: suma, resta, multiplicación, división, módulo y salir. Solicita el código de operación y dos números. Muestra el resultado. Si el código no existe, pide otro. Termina cuando el código sea el de salida.

57. **Menú de figuras con caracteres**  
    Crea un menú con nombres de figuras (ej: cuadrado, triángulo). El usuario elige una figura y el carácter con el que se dibujará. Después de imprimir la figura, vuelve al menú. Incluye opción para cambiar el carácter y terminar.

## 4. Arreglos Unidimensionales (Vectores)

### 4.1. Generación y Manipulación Básica
58. **Convertir binario a decimal**  
    Genera un arreglo de 8 elementos con valores 0 y 1 aleatorios. Interpreta el arreglo como un número binario y muestra su equivalente decimal.

59. **Convertir decimal a binario con arreglo**  
    Genera un número aleatorio entre 1 y 500,000. Mediante divisiones sucesivas entre 2, almacena los residuos en un arreglo de 10 celdas. Muestra el arreglo. Si las celdas no son suficientes, muestra "celdas del arreglo se agotaron".

60. **Mostrar a partir del primer 1**  
    Genera un arreglo de longitud aleatoria (5 a 20) con valores 0 y 1. Muestra solo los valores a partir del primer 1 encontrado (ignorando ceros a la izquierda).

61. **Arreglo sin repetidos**  
    Genera un arreglo de 20 elementos con valores aleatorios del 1 al 20, sin repetir. Muestra el arreglo.

62. **Formar un número a partir de dígitos**  
    Genera 6 números aleatorios del 0 al 9 y los almacena en un arreglo. Luego forma un solo número concatenando los dígitos (ej: [8,3,1,7,2,1] → 831721). Muestra el arreglo y el número formado.

63. **Cifrado de 4 dígitos**  
    Genera un número aleatorio de 4 cifras (1000 a 9999). Separa sus dígitos en un arreglo. Aplica el cifrado: suma 7 a cada dígito, obtén el residuo módulo 10, luego intercambia posiciones (primero con tercero, segundo con cuarto). Muestra el arreglo original y el cifrado.

64. **Arreglo ordenado ascendente**  
    Solicita un número N (entre 2 y 50). Genera un arreglo de N elementos con números aleatorios entre 3 y 900, asegurando que estén en orden ascendente (cada elemento >= anterior). Muestra el arreglo.

65. **Media (mediana) de un arreglo ordenado**  
    Dado un arreglo ordenado (ejercicio anterior), calcula la media: si N es impar, el valor central; si es par, el promedio de los dos centrales. Muestra el arreglo y la media.

66. **Registro de caracteres con corrimiento**  
    Crea un arreglo de 12 caracteres inicializado con espacios. Solicita caracteres al usuario; cada nuevo carácter se inserta en la posición 0, desplazando los anteriores a la derecha. Termina cuando se ingresa '$' o se llena el arreglo. Muestra solo las celdas con caracteres ingresados.

67. **Serie de Ulam en arreglo**  
    Solicita un número entero positivo. Genera la serie de Ulam y almacénala en un arreglo de tamaño exacto (sin celdas sobrantes). Muestra el arreglo.

### 4.2. Búsquedas y Cálculos
68. **Mínimo, máximo y promedio de un arreglo**  
    Genera un arreglo de enteros aleatorios. Encuentra el valor mínimo, el máximo y calcula el promedio. Muestra los resultados.

69. **Conteo de valores**  
    Genera un arreglo de enteros aleatorios en un rango dado. Cuenta cuántas veces aparece cada valor y muestra los conteos.

70. **Separar pares e impares**  
    Genera un arreglo de enteros aleatorios. Separa los números pares e impares en dos arreglos diferentes y muéstralos.

71. **Rotación de arreglo**  
    Genera un arreglo de enteros. Solicita un número de posiciones a rotar a la derecha. Realiza la rotación y muestra el arreglo original y el rotado.

72. **Invertir un arreglo**  
    Genera un arreglo de enteros. Invierte el orden de sus elementos (sin usar un arreglo auxiliar) y muestra el original y el invertido.

## 5. Cadenas (Strings)

### 5.1. Procesamiento de Texto
73. **Contar vocales**  
    Solicita una cadena al usuario. Cuenta cuántas vocales (mayúsculas y minúsculas) contiene y muestra el resultado.

74. **Validar cadena numérica y elevarla al cuadrado**  
    Solicita una cadena. Valida que contenga solo dígitos del 0 al 9. Si es válida, conviértela a entero, elévala al cuadrado y muestra el resultado. Si no, muestra "Error la cadena no tiene formato de número entero".

75. **Invertir cadena y convertir a mayúsculas**  
    Solicita una cadena (puede incluir espacios). Inviértela y conviértela a mayúsculas. Muestra el resultado.

76. **Capitalizar palabras**  
    Solicita una frase (puede tener espacios múltiples). Conviértela para que cada palabra comience con mayúscula y el resto en minúscula. Además, normaliza los espacios (solo uno entre palabras). Muestra la frase resultante.

77. **Evaluar expresión aritmética**  
    Solicita una cadena con una expresión aritmética simple (ej: "5+3", "10/2"). Separa los operandos y el operador (+, -, *, /, %). Realiza la operación y muestra el resultado.

78. **Buscar subcadena exacta**  
    Solicita dos cadenas. Determina si la segunda cadena está contenida de manera exacta en la primera. Muestra un mensaje indicando si está o no.

79. **Contar palabras**  
    Solicita una cadena. Cuenta cuántas palabras contiene (separadas por espacios). Muestra el resultado.

80. **Eliminar caracteres duplicados consecutivos**  
    Solicita una cadena. Elimina los caracteres duplicados consecutivos (ej: "aaabbbccc" → "abc"). Muestra el resultado.

81. **Verificar palíndromo en cadena**  
    Solicita una cadena (puede contener espacios y signos). Ignora los no alfabéticos y determina si es un palíndromo (se lee igual al derecho y al revés). Muestra el resultado.

### 5.2. Transformaciones
82. **Reemplazar vocales por números**  
    Solicita una cadena. Reemplaza cada vocal por un número: a=1, e=2, i=3, o=4, u=5 (sin distinguir mayúsculas/minúsculas). Muestra la cadena resultante.

83. **Codificar cadena (Cifrado César)**  
    Solicita una cadena y un número de desplazamiento. Aplica un cifrado César (desplazar cada letra por el desplazamiento dado en el alfabeto). Muestra la cadena codificada.

84. **Contar ocurrencias de una letra**  
    Solicita una cadena y una letra. Cuenta cuántas veces aparece esa letra (sin distinguir mayúsculas/minúsculas). Muestra el resultado.

85. **Extraer números de una cadena**  
    Solicita una cadena que puede contener números y letras. Extrae todos los números y muéstralos como un solo número o como una lista.

## 6. Matrices (Arreglos Bidimensionales)

### 6.1. Generación y Recorrido
86. **Matriz con pares en filas pares e impares en filas impares**  
    Genera una matriz de R filas (aleatorio 2-9) y C columnas (aleatorio 2-6). Llena las filas pares solo con números pares (0-99) y las impares solo con impares. Muestra la matriz.

87. **Llenar matriz desde un arreglo unidimensional**  
    Crea un arreglo unidimensional de caracteres (de tu elección). Luego crea una matriz de R x C (aleatorio 2-10). Llena la matriz columna por columna, de abajo hacia arriba, con los caracteres del arreglo. Muestra el arreglo y la matriz.

88. **Mínimo, máximo y promedio de la periferia**  
    Genera una matriz de enteros (dimensiones aleatorias). Encuentra el valor mínimo y máximo, sus posiciones, y calcula el promedio de los valores en la periferia (primera y última fila, primera y última columna). Muestra todo.

89. **Promedio por fila**  
    Genera una matriz de enteros (dimensiones aleatorias). Calcula el promedio de cada fila y almacénalo en un arreglo de flotantes. Muestra la matriz y los promedios.

90. **Intercambiar dos filas**  
    Crea una matriz de 3x8 con valores iniciales predefinidos. Solicita dos números de fila (validar que existan). Intercambia los valores de esas dos filas. Muestra la matriz antes y después.

91. **Matriz sin repetidos**  
    Crea una matriz de 3x5. Llénala con números aleatorios entre 20 y 36, sin repetir. Muestra la matriz.

92. **Moda de una matriz**  
    Crea una matriz de R x C (aleatorio 3-7) con valores entre 0 y 5. Calcula la moda (el valor que más se repite). Usa un arreglo unidimensional para los conteos. Muestra la matriz y la moda.

93. **Convertir filas binarias a decimal**  
    Genera una matriz de dimensiones aleatorias (3-6 filas y columnas) con valores 0 y 1. Interpreta cada fila como un número binario y convierte a decimal. Almacena los decimales en un arreglo. Muestra la matriz y el arreglo.

### 6.2. Operaciones Avanzadas
94. **Clasificar cadenas en matriz**  
    Crea una matriz de cadenas de 3 filas y N columnas (N aleatorio 4-9). Solicita cadenas al usuario hasta que ingrese "alto". Clasifica: si la cadena tiene un solo carácter, va a la fila 0; si tiene dos caracteres que se alternan uniformemente (ej: "abab"), a la fila 1; sino, a la fila 2. Muestra la matriz al final.

95. **Cifrado múltiple de números de 4 cifras**  
    Crea una matriz de N filas (aleatorio 3-9) y 4 columnas. Solicita N números de 4 cifras (validar). Separa cada número en sus dígitos y colócalos en una fila de la matriz. Luego aplica el cifrado (sumar 7, módulo 10, intercambiar columnas 0-2 y 1-3) y guarda el resultado en otra matriz. Muestra ambas matrices.

96. **Transponer matriz**  
    Genera una matriz de dimensiones aleatorias. Crea su transpuesta (intercambiar filas por columnas). Muestra la original y la transpuesta.

97. **Suma de dos matrices**  
    Genera dos matrices de las mismas dimensiones (aleatorias). Súmalas y muestra las matrices originales y la resultante.

98. **Multiplicación de matrices**  
    Genera dos matrices (dimensiones compatibles para multiplicación). Realiza la multiplicación y muestra las matrices y el resultado.

99. **Recorrido en espiral**  
    Genera una matriz cuadrada (aleatoria). Recorre la matriz en espiral (desde la esquina superior izquierda hacia el centro) y muestra los elementos en ese orden.

## 7. Estructuras de Datos y Programación Orientada a Objetos (Structs y Traits)

### 7.1. Cola (Queue)
100. **Implementar una cola de cadenas**  
     Crea una estructura `Cola` con:  
     - Un vector para almacenar N cadenas (N se pasa al constructor).  
     - Un contador de elementos.  
     Métodos:  
     - `agregar`: agrega un elemento al final, devuelve `bool` (éxito o fallo).  
     - `eliminar`: elimina y devuelve el primer elemento (cadena vacía si está vacía).  
     - `recorrer`: desplaza los elementos una celda a la izquierda (apoyo para eliminar).  
     - `mostrar`: muestra los elementos activos.  
     - `esta_vacia`, `esta_llena`, `vaciar`.  
     Incluye un programa principal con menú para probar todas las operaciones.

101. **Implementar una pila de cadenas**  
     Similar a la cola, pero con comportamiento LIFO. Método `eliminar` quita el último elemento ingresado.

### 7.2. Fracciones
102. **Clase para fracciones mixtas**  
     Crea una estructura `Quebrado` con campos: entero, numerador, denominador.  
     Métodos:  
     - Constructor con tres parámetros.  
     - `sumar`, `restar`, `multiplicar`, `dividir`: cada uno recibe una cadena con formato "entero numerador/denominador" (ej: "2 1/8") y devuelve una cadena con la operación y resultado.  
     - `mostrar`: imprime la fracción.  
     - `resetear`: pone los valores a 1, 1, 2.  
     Escribe un programa principal que demuestre el funcionamiento.

### 7.3. Matrices como Clase
103. **Clase para matrices de enteros**  
     Crea una estructura `Matriz` con un campo que sea una matriz de enteros (dimensiones dadas en el constructor).  
     Métodos:  
     - Constructor que recibe filas y columnas, e inicializa con valores aleatorios (10-100).  
     - `obtener_min_max_fila`: recibe un índice de fila, devuelve una cadena con el mínimo y máximo de esa fila (formato "min-max"). Si la fila no existe, devuelve cadena vacía.  
     - `obtener_min_max_columna`: similar para columnas.  
     - `mostrar`: imprime la matriz.  
     Programa principal que muestre su uso.

### 7.4. Otros
104. **Lista enlazada simple**  
     Implementa una lista enlazada simple que almacene enteros. Debe tener métodos para insertar al inicio, al final, eliminar por valor, buscar, y mostrar la lista.

105. **Árbol binario de búsqueda**  
     Implementa un árbol binario de búsqueda para enteros. Incluye métodos para insertar, buscar, y recorridos (inorden, preorden, postorden).

## 8. Recursión

106. **Suma de cuadrados recursiva**  
     Solicita un número N. Calcula recursivamente la suma de los primeros N cuadrados: 1² + 2² + ... + N².

107. **Fibonacci recursivo**  
     Solicita un número N. Calcula el N-ésimo término de la sucesión de Fibonacci usando recursión.

108. **Factorial recursivo**  
     Calcula el factorial de N recursivamente.

109. **Potencia recursiva**  
     Calcula B^P recursivamente.

110. **Invertir número recursivamente**  
     Solicita un número entero. Inviértelo usando recursión.

111. **Suma de dígitos recursiva**  
     Calcula la suma de los dígitos de un número usando recursión.

112. **Máximo común divisor recursivo (Euclides)**  
     Calcula el MCD de dos números usando el algoritmo de Euclides recursivamente.

113. **Torres de Hanói**  
     Implementa el juego de las Torres de Hanói para N discos, mostrando los movimientos recursivamente.

## 9. Ejercicios Adicionales (para completar más de 100)

114. **Generador de contraseñas**  
     Solicita la longitud deseada. Genera una contraseña aleatoria que incluya letras mayúsculas, minúsculas, números y símbolos.

115. **Adivina el número**  
     Genera un número aleatorio entre 1 y 100. El usuario debe adivinarlo; el programa da pistas ("mayor" o "menor"). Cuenta los intentos.

116. **Calculadora de cambio**  
     Solicita el total a pagar y la cantidad con que paga el cliente. Calcula el cambio e indica cuántos billetes/monedas de cada denominación (500, 200, 100, 50, 20, 10, 5, 2, 1) se deben dar.

117. **Conversor de temperatura**  
     Solicita una temperatura y la unidad (Celsius, Fahrenheit, Kelvin). Convierte a las otras dos unidades y muestra los resultados.

118. **Calculadora de IMC**  
     Solicita peso (kg) y altura (m). Calcula el Índice de Masa Corporal y clasifica: bajo peso, normal, sobrepeso, obesidad.

119. **Simulador de dado**  
     Simula el lanzamiento de un dado (1-6). Permite al usuario lanzar múltiples veces y muestra la frecuencia de cada resultado.

120. **Ordenamiento de arreglo**  
     Genera un arreglo de números aleatorios. Implementa el ordenamiento por burbuja, selección o inserción y muestra el arreglo antes y después.

121. **Búsqueda binaria**  
     Genera un arreglo ordenado. Solicita un número y búscalo usando búsqueda binaria. Indica si está y en qué posición.

---

*Los ejercicios fueron obtenidos de mi material de clase de Fundamentos de Programación que cursé en 2023, ordenados y adecuados por deepsek*