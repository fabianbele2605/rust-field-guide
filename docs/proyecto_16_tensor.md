# 📚 Proyecto 16: Tensor Básico — Documentación Educativa

**Campo:** 🤖 AI / Machine Learning | **Nivel:** 🟢 Básico | **Estado:** ✅ COMPLETADO
**Carpeta:** `ia-ml/tensor/`

---

## 🎯 Resumen: ¿Qué Construimos?

Una estructura `Matrix` con las operaciones fundamentales que usa cualquier librería de tensores (NumPy, PyTorch, etc. por debajo): suma, resta, multiplicación (dot product) y transpose.

```
Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0])
  ↓
[1 2]
[3 4]

m1 + m2   → suma elemento a elemento
m1 - m2   → resta elemento a elemento
a.multiply(&b) → multiplicación matricial real
c.transpose()  → invierte filas y columnas
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. Memory layout plano: `Vec<f64>` en vez de `Vec<Vec<f64>>`

```rust
pub struct Matrix {
    pub data: Vec<f64>,   // todos los números en una sola fila continua
    pub rows: usize,
    pub cols: usize,
}

pub fn get(&self, row: usize, col: usize) -> f64 {
    self.data[row * self.cols + col]
}
```

**¿Por qué no `Vec<Vec<f64>>`?**
- `Vec<Vec<f64>>` significa: un Vec de punteros, cada uno apuntando a OTRO Vec en una posición distinta de memoria — los datos quedan dispersos (fragmentados)
- `Vec<f64>` plano guarda TODO en un bloque continuo de memoria — mucho más rápido de recorrer (mejor uso de caché del CPU) y es exactamente cómo lo hacen NumPy, PyTorch y toda librería de álgebra lineal seria
- El costo: hay que calcular manualmente el índice (`row * cols + col`) en vez de usar `data[row][col]` directamente

### 2. Operator Overloading: `std::ops::Add` y `std::ops::Sub`

```rust
impl Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix { ... }
}
```

**¿Qué hace esto?** Le enseña al compilador qué significa el símbolo `+` cuando AMBOS operandos son `Matrix` — así puedes escribir `m1 + m2` en vez de `m1.add(m2)`. Es el mismo mecanismo por el que `1 + 2` funciona para enteros: `i32` también implementa `Add`, solo que la implementación viene con el lenguaje.

`type Output = Matrix;` declara qué tipo produce la operación — no siempre tiene que ser el mismo tipo que los operandos (por ejemplo, `Vec<i32> + i32` podría producir otro `Vec<i32>` con cada elemento sumado, si alguien lo implementara así).

### 3. `iter().zip().map().collect()` — procesar dos colecciones en paralelo

```rust
let data: Vec<f64> = self.data.iter()
    .zip(other.data.iter())
    .map(|(a, b)| a + b)
    .collect();
```

- `.zip()` empareja elementos de dos iteradores por posición: `(self.data[0], other.data[0])`, `(self.data[1], other.data[1])`, etc.
- `.map(|(a, b)| a + b)` transforma cada par en su suma
- `.collect()` junta todos los resultados en un nuevo `Vec`

Es la forma idiomática de "recorrer dos listas a la vez" en Rust, sin usar índices manuales ni loops explícitos.

### 4. `assert_eq!` como validación de invariantes

```rust
assert_eq!(self.rows, other.rows, "Filas no coinciden");
```

Si las dimensiones no coinciden, el programa se detiene inmediatamente con un mensaje claro, en vez de producir un resultado incorrecto silenciosamente (o un panic críptico de índice fuera de rango más adelante). Es una forma simple de documentar y hacer cumplir las reglas matemáticas de la operación.

---

## 🖥️ Conceptos de Álgebra Lineal / ML

### 1. Suma/Resta vs Multiplicación de matrices — no son iguales

**Suma/Resta:** elemento a elemento, ambas matrices deben tener EXACTAMENTE las mismas dimensiones.
```
[1 2]   [5 6]   [6  8]
[3 4] + [7 8] = [10 12]
```

**Multiplicación (dot product):** cada elemento del resultado es la suma de productos entre una FILA completa de A y una COLUMNA completa de B. Las dimensiones deben "encajar": columnas de A = filas de B.
```
resultado[i][j] = suma_k( A[i][k] * B[k][j] )
```

### 2. Transpose — invertir filas y columnas

```
A (2x3):        Aᵀ (3x2):
[1 2 3]         [1 4]
[4 5 6]    →    [2 5]
                [3 6]
```

El elemento en `[i][j]` de A pasa a `[j][i]` en `Aᵀ` — las dimensiones se invierten. Es una operación fundamental en ML: por ejemplo, en redes neuronales, calcular gradientes durante backpropagation requiere transponer matrices de pesos constantemente.

### 3. Por qué esto importa para IA

Todo lo que hace una red neuronal — desde una capa simple hasta un transformer completo — se reduce, en su núcleo, a multiplicaciones y sumas de matrices como estas, ejecutadas millones de veces. Frameworks como PyTorch o TensorFlow son, en esencia, versiones MUY optimizadas (con GPU, paralelismo, etc.) de exactamente estas operaciones.

---

## 📝 Código Final

```rust
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix { data: vec![0.0; rows * cols], rows, cols }
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "El tamaño de data no coincide con rows*cols");
        Matrix { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows, "Dimensiones incompatibles para multiplicar");
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }

    pub fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{:.1} ", self.get(r, c));
            }
            println!();
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows, "Filas no coinciden");
        assert_eq!(self.cols, other.cols, "Columnas no coinciden");
        let data: Vec<f64> = self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect();
        Matrix::from_vec(self.rows, self.cols, data)
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows, "Filas no coinciden");
        assert_eq!(self.cols, other.cols, "Columnas no coinciden");
        let data: Vec<f64> = self.data.iter().zip(other.data.iter()).map(|(a, b)| a - b).collect();
        Matrix::from_vec(self.rows, self.cols, data)
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| Memory layout plano | `Vec<f64>` en vez de `Vec<Vec<f64>>` | Datos contiguos, más rápido, así lo hacen librerías reales |
| `impl Add`/`impl Sub` | Operator overloading | Escribir `m1 + m2` en vez de `m1.add(m2)` |
| `.zip().map().collect()` | Procesar dos colecciones en paralelo | Sin loops manuales ni índices |
| `assert_eq!` | Validar invariantes | Falla temprano y claro, no silenciosamente |
| Dot product | Multiplicación real de matrices | Base matemática de toda red neuronal |

---

## 🚀 Próximos Pasos

- **Proyecto 17:** Neural Network Pequeña (usará esta `Matrix` como base)
- Soporte para tensores de N dimensiones (no solo 2D)
- Operaciones element-wise adicionales (multiplicación por escalar, funciones de activación)

---

**🎉 Proyecto 16: Completado. Primer proyecto del campo AI / Machine Learning.**
