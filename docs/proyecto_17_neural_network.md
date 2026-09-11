# 📚 Proyecto 17: Neural Network Pequeña — Documentación Educativa

**Campo:** 🤖 AI / Machine Learning | **Nivel:** 🟡 Intermedio | **Estado:** ✅ COMPLETADO
**Carpeta:** `ia-ml/neural_net/`

> **Nota de alcance:** la guía sugiere ReLU + Softmax; usamos **sigmoid** en ambas capas por ser más simple de derivar a mano para backpropagation — es la variante clásica para el primer contacto con el concepto. Al final se explica cómo serían ReLU/Softmax en una red real.

---

## 🎯 Resumen: ¿Qué Construimos?

Una red neuronal de 2 capas, **desde cero, sin frameworks**, que aprende a resolver XOR — el ejemplo clásico porque NO es resoluble con una sola línea recta (no es "linealmente separable"), así que demuestra que la red realmente aprendió una función no-trivial.

```
Input (2 valores) → Layer oculta (4 neuronas, sigmoid) → Layer salida (1 neurona, sigmoid) → Output
                              ↑                                                                 ↓
                              └──────────────── Backpropagation (ajusta pesos) ←── Error ───────┘
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. Generador pseudo-aleatorio propio (LCG) sin dependencias

```rust
struct SimpleRng { state: u64 }

impl SimpleRng {
    fn next_f64(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.state >> 33) as f64 / u32::MAX as f64) - 0.5
    }
}
```

Un **Linear Congruential Generator** es la forma más simple de "aleatoriedad" (en realidad determinista, pero parece azarosa): cada nuevo número se calcula a partir del anterior con una fórmula fija. `wrapping_mul`/`wrapping_add` permiten que el número se desborde (overflow) intencionalmente sin que Rust entre en pánico — es exactamente el comportamiento que un LCG necesita.

### 2. `iter().zip().map().sum()` para el producto punto de una neurona

```rust
let sum: f64 = neuron_weights.iter().zip(inputs.iter())
    .map(|(w, x)| w * x)
    .sum();
```

Mismo patrón que en el Proyecto 16 (`zip` + `map`), aquí con `.sum()` en vez de `.collect()` — colapsa el iterador a un solo número en vez de construir otra colección. Es literalmente la fórmula de una neurona: `Σ(peso_i × entrada_i)`.

### 3. Estructuras anidadas: `NeuralNetwork` contiene `Layer`s

```rust
struct NeuralNetwork {
    hidden: Layer,
    output: Layer,
}
```

Patrón de composición: una red es "una colección de capas conectadas", cada capa resuelve su propia responsabilidad (`forward`), y la red solo encadena sus salidas/entradas. Mismo principio de modularidad de proyectos anteriores.

### 4. Tuplas para retornar múltiples valores

```rust
fn forward(&self, inputs: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let hidden_output = self.hidden.forward(inputs);
    let final_output = self.output.forward(&hidden_output);
    (hidden_output, final_output)
}
```

Necesitamos AMBAS salidas (la de la capa oculta y la final) porque backpropagation las usa a las dos — Rust permite retornar múltiples valores empaquetados en una tupla, sin necesitar una struct dedicada para algo tan simple.

---

## 🖥️ Conceptos de Redes Neuronales

### 1. Por qué XOR necesita una capa oculta

```
AND, OR: se pueden separar con UNA línea recta
XOR:     NO se puede — necesitas al menos 2 líneas (o una curva)

  1 |  1   0        Una sola neurona (sin capa oculta) solo puede
    |               dibujar UNA línea recta divisoria — no alcanza
  0 |  0   1         para separar los "1" de los "0" en este patrón.
    +--------
      0   1
```

Este es el motivo histórico por el que XOR es EL ejemplo clásico: demuestra por qué las redes necesitan capas ocultas (fue, de hecho, un problema real que casi mata la investigación en redes neuronales en los años 60, hasta que se popularizó backpropagation con capas ocultas en los 80).

### 2. Forward pass vs Backward pass

- **Forward pass:** los datos fluyen de entrada → salida, calculando la predicción actual
- **Backward pass (backpropagation):** el ERROR fluye de salida → entrada, calculando "cuánto debería cambiar cada peso" para reducir ese error la próxima vez

### 3. La regla de la cadena, aplicada sin decirlo

```rust
let delta_output = error_output * output_value * (1.0 - output_value);
```

`output_value * (1 - output_value)` es la derivada de sigmoid evaluada en su propia salida (`sigmoid'(x) = sigmoid(x)(1-sigmoid(x))`). Multiplicar el error por esta derivada es aplicar la **regla de la cadena** del cálculo diferencial — backpropagation es, en esencia, la regla de la cadena aplicada capa por capa, desde la salida hacia la entrada.

### 4. Learning rate: qué tan grande es cada "paso" de ajuste

```rust
self.output.weights[0][j] += learning_rate * delta_output * hidden_output[j];
```

`learning_rate` (aquí 0.5) controla cuánto se mueve cada peso en cada actualización. Muy alto: la red "salta" de más y puede no converger. Muy bajo: aprende correctamente pero muy lento (necesitaría muchas más épocas).

### 5. Épocas: repetir el dataset completo muchas veces

Una época = una pasada completa por los 4 ejemplos de XOR. Con solo 1 época, la red apenas ajusta un poco sus pesos. Con 10,000 épocas, los ajustes pequeños se acumulan hasta que la red converge al patrón correcto — verificado en esta sesión: de valores cercanos a 0.4-0.5 (aleatorios) a valores cercanos a 0.02/0.98 (correctos).

---

## 📝 Código Final

Ver `ia-ml/neural_net/src/main.rs` completo. Estructura:

```rust
struct Layer { weights: Vec<Vec<f64>>, biases: Vec<f64> }
struct NeuralNetwork { hidden: Layer, output: Layer }

impl NeuralNetwork {
    fn forward(&self, inputs: &[f64]) -> (Vec<f64>, Vec<f64>) { ... }
    fn train_step(&mut self, inputs: &[f64], target: f64, learning_rate: f64) {
        // 1. Forward pass
        // 2. Calcular delta de la capa de salida (error × derivada de sigmoid)
        // 3. Propagar el error hacia la capa oculta
        // 4. Actualizar todos los pesos y biases con gradient descent
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| LCG (pseudo-random) | Generador determinista que parece azaroso | Inicializar pesos sin que la red arranque simétrica |
| Sigmoid | Aplasta cualquier número a (0,1) | Interpretar salida como activación/probabilidad |
| Forward pass | Datos → predicción | Calcular qué "cree" la red actualmente |
| Backpropagation | Error → ajuste de pesos | Aprender de los errores, capa por capa |
| Learning rate | Tamaño del paso de ajuste | Balance entre velocidad y estabilidad de aprendizaje |
| Época | Una pasada completa por el dataset | Repetir hasta que los pesos converjan |

---

## 🔄 Variante real: ReLU + Softmax (mencionado en la guía original)

En redes más grandes (clasificación multi-clase, capas profundas), es más común usar:
- **ReLU** (`max(0, x)`) en capas ocultas — más simple y rápida que sigmoid, evita que el gradiente se "desvanezca" en redes profundas
- **Softmax** en la capa de salida — convierte varios números en una distribución de probabilidad que suma 1 (útil cuando hay más de 2 clases posibles, no solo "sí/no")

No las implementamos aquí porque sigmoid + MSE (implícito en `target - output`) es más simple de derivar a mano para un primer contacto — pero el mecanismo de backpropagation es EXACTAMENTE el mismo, solo cambia la derivada de la función de activación.

---

## 🚀 Próximos Pasos

- **Proyecto 18:** Inference Engine (cargar un modelo preentrenado real)
- Reemplazar sigmoid por ReLU + Softmax
- Reutilizar la `Matrix` del Proyecto 16 en vez de `Vec<Vec<f64>>`
- Mini-batches (entrenar con varios ejemplos a la vez, no uno por uno)

---

**🎉 Proyecto 17: Completado. Red neuronal desde cero que aprendió XOR.**
