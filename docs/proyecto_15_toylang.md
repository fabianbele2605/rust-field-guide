# 📚 Proyecto 15: Mini Compiler / Interpreter — Documentación Educativa

**Campo:** 📦 CLI / Developer Tools | **Nivel:** 🔴 Avanzado | **Estado:** ✅ COMPLETADO
**Carpeta:** `cli-tools/toylang/`

---

## 🎯 Resumen: ¿Qué Construimos?

Un intérprete completo para un lenguaje "toy": variables (`let`), suma (`+`), y `print()`.

```
let a = 5;
let b = 10;
let c = a + b;
print(c);          → 15
let d = c + a + b;
print(d);          → 30
```

**Pipeline completa:**
```
Código fuente (texto)
    ↓ Lexer
Tokens (lista de palabras/símbolos reconocidos)
    ↓ Parser
AST (árbol que representa la estructura del programa)
    ↓ Interpreter
Resultado (ejecución real, con estado)
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. El Lexer: consumir texto carácter por carácter

```rust
while i < chars.len() {
    let c = chars[i];
    if c.is_alphabetic() {
        let start = i;
        while i < chars.len() && chars[i].is_alphanumeric() { i += 1; }
        let word: String = chars[start..i].iter().collect();
        // decidir si es palabra clave o identificador
    }
    ...
}
```

Patrón clásico de tokenización: mientras el carácter actual pertenezca a una categoría (letra, dígito), seguir avanzando y acumulando — al cortar, se tiene la "palabra" completa lista para clasificar.

### 2. El AST con recursión: `Box<Expr>`

```rust
pub enum Expr {
    Number(i64),
    Ident(String),
    Add(Box<Expr>, Box<Expr>),
}
```

**¿Por qué `Box`?** `Expr` se contiene a sí mismo dentro de `Add` (una expresión de suma contiene DOS expresiones más, que a su vez podrían ser sumas). Sin `Box`, Rust no puede calcular el tamaño de `Expr` en tiempo de compilación (sería infinito: una suma contiene sumas que contienen sumas...). `Box<Expr>` pone esos datos en el heap con un puntero de tamaño fijo, rompiendo la recursión infinita de tamaño.

### 3. El Parser: descenso recursivo

```rust
fn parse_expr(&mut self) -> Expr {
    let left = /* parsear número o identificador */;
    if *self.current() == Token::Plus {
        self.advance();
        let right = self.parse_expr();  // ← se llama a sí misma
        return Expr::Add(Box::new(left), Box::new(right));
    }
    left
}
```

Para `c + a + b`, la función se llama a sí misma para el lado derecho de cada `+`, construyendo un árbol anidado: `Add(c, Add(a, b))`. Esta técnica ("recursive descent parsing") es la forma más común de escribir parsers a mano, sin librerías.

### 4. El Interpreter: evaluación recursiva del AST

```rust
fn eval_expr(&self, expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(name) => *self.env.get(name).unwrap(),
        Expr::Add(left, right) => self.eval_expr(left) + self.eval_expr(right),
    }
}
```

Evaluar `Add(c, Add(a, b))` evalúa primero cada lado (recursivamente) y luego suma los resultados — el árbol se "colapsa" desde las hojas hacia la raíz.

### 5. `Option<&T>::cloned()` vs `Option<&T>::clone()`

```rust
self.tokens.get(self.pos).clone()    // ❌ Option<&Token> (clona la referencia)
self.tokens.get(self.pos).cloned()   // ✅ Option<Token> (clona el valor de adentro)
```

**Bug real de esta sesión:** `.clone()` sobre un `Option<&T>` produce OTRO `Option<&T>` (clonar un puntero es barato y no cambia su tipo). `.cloned()` es el método específico que, cuando `T: Clone`, transforma `Option<&T>` en `Option<T>` clonando lo que hay adentro. Necesitábamos `Token` (con dueño propio), no `&Token` (prestado) — de ahí el error de tipos.

---

## 🐛 Otros bugs de esta sesión

### 1. Typo en el código fuente de prueba

```rust
let source = "...\nnprint(y);";  // ❌ "nprint" en vez de "print"
```

El lexer no reconoció `"nprint"` como palabra clave, lo trató como un identificador nuevo. Recordatorio de que el lexer solo puede trabajar con lo que realmente recibe — el error estaba en el texto de entrada, no en la lógica.

### 2. Falta un `;` al final del archivo de prueba

```
print(d)     // ❌ sin punto y coma
```

El parser de `print(...)` siempre espera consumir exactamente `)` y luego `;`. Sin el `;`, el parser intenta leer un token que no existe → panic. Este bug motivó el Paso 6 (mensajes de error más claros).

---

## 🖥️ Conceptos de Compiladores / Intérpretes

### 1. Por qué se separan Lexer, Parser e Interpreter

Cada etapa resuelve UN problema:
- **Lexer:** ¿cuáles son las "palabras" válidas? (no le importa el orden ni la estructura)
- **Parser:** ¿en qué ORDEN y ESTRUCTURA deben aparecer esas palabras? (no ejecuta nada, solo valida forma)
- **Interpreter:** dado un árbol ya válido, ¿qué SIGNIFICA, qué debe hacer?

Separar estas responsabilidades hace que cada una sea más simple de escribir, testear y entender por separado — el mismo principio de modularidad que ya viste en proyectos anteriores (kernel: paging/syscalls/filesystem como módulos independientes).

### 2. Manejo de errores en un parser: de panic a mensaje útil

```rust
// Antes: index out of bounds: the len is 35 but the index is 35
// Después: "Fin de archivo inesperado (¿falta un ';' o ')'?)"
```

Un compilador/intérprete real (rustc, por ejemplo) invierte MUCHO esfuerzo en mensajes de error claros — es una de las razones por las que Rust es conocido por sus errores "amigables". Aquí dimos el primer paso: convertir un fallo críptico de índice en un mensaje que sugiere la causa probable.

---

## 📝 Código Final

Ver `cli-tools/toylang/src/` — 3 módulos:
- `lexer.rs` — `tokenize(source: &str) -> Vec<Token>`
- `parser.rs` — `Parser::parse_program() -> Vec<Statement>`
- `interpreter.rs` — `Interpreter::run(&program)`

```rust
// main.rs — la pipeline completa en 4 líneas
let source = fs::read_to_string(&args[1]).expect("No se pudo leer el archivo");
let tokens = tokenize(&source);
let mut parser = Parser::new(tokens);
let program = parser.parse_program();
let mut interpreter = Interpreter::new();
interpreter.run(&program);
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| Lexer | Texto → tokens | Reconocer las "palabras" válidas del lenguaje |
| `Box<Expr>` | Puntero al heap dentro de un enum recursivo | Tamaño finito en tiempo de compilación |
| Recursive descent parsing | El parser se llama a sí mismo | Construir árboles anidados desde una lista plana |
| Evaluación recursiva | El interpreter se llama a sí mismo | "Colapsar" el árbol desde las hojas hacia la raíz |
| `.cloned()` vs `.clone()` | Clonar el valor vs clonar la referencia | Necesario para obtener un `Option<T>` desde un `Option<&T>` |

---

## 🚀 Cierre del Campo CLI / Developer Tools

Con este proyecto se cierra **CLI / Developer Tools** (Proyectos 13, 14, 15).

**Si se retoma en el futuro:**
- Soporte para más operadores (`-`, `*`, `/`) y precedencia
- Condicionales (`if`) y loops (`while`)
- Funciones definidas por el usuario
- Tipos además de enteros (strings, booleanos)

---

**🎉 Proyecto 15: Completado. Campo CLI / Developer Tools (13, 14, 15) cerrado.**
