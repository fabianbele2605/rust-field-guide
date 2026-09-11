// Generador pseudo-aleatorio simple (LCG), sin dependencias externas
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    fn next_f64(&mut self) -> f64 {
        // Constantes tipicas de un LCG (linear congruential generator)
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Convertir a un valor entre -0.5 y 0.5
        ((self.state >> 33) as f64 / u32::MAX as f64) - 0.5
    }
}

struct Layer {
    weights: Vec<Vec<f64>>,     // weights[neurona][entrada]
    biases: Vec<f64>,
}

struct NeuralNetwork {
    hidden: Layer,
    output: Layer,
}

impl NeuralNetwork {
    fn new(n_inputs: usize, n_hidden: usize, n_output: usize, rng: &mut SimpleRng) -> Self {
        NeuralNetwork {
            hidden: Layer::new(n_inputs, n_hidden, rng),
            output: Layer::new(n_hidden, n_output, rng),
        }
    }

    fn forward(&self, inputs: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let hidden_output = self.hidden.forward(inputs);
        let final_output = self.output.forward(&hidden_output);
        (hidden_output, final_output)
    }

    fn train_step(&mut self, inputs: &[f64], target: f64, learning_rate: f64) {
        let (hidden_output, final_output) = self.forward(inputs);
        let output_value = final_output[0];

        // --- Capa de salida ---
        let error_output = target - output_value;
        let delta_output = error_output * output_value * (1.0 - output_value);

        // --- Capa oculta ---
        let mut delta_hidden = vec![0.0; hidden_output.len()];
        for j in 0..hidden_output.len() {
            let error_hidden = delta_output * self.output.weights[0][j];
            delta_hidden[j] = error_hidden * hidden_output[j] * (1.0 - hidden_output[j]);
        }

        // --- Actualizar pesos de la capa de salida ---
        for j in 0..hidden_output.len() {
            self.output.weights[0][j] += learning_rate * delta_output * hidden_output[j];
        }
        self.output.biases[0] += learning_rate * delta_output;

        // --- Actualizar pesos de la capa oculta ---
        for j in 0..self.hidden.weights.len() {
            for k in 0..inputs.len() {
                self.hidden.weights[j][k] += learning_rate * delta_hidden[j] * inputs[k];
            }
            self.hidden.biases[j] += learning_rate * delta_hidden[j];
        }
    }
}

impl Layer {
    fn new(n_inputs: usize, n_neurons: usize, rng: &mut SimpleRng) -> Self {
        let weights = (0..n_neurons)
            .map(|_| (0..n_inputs).map(|_| rng.next_f64()).collect())
            .collect();
        let biases = (0..n_neurons).map(|_| rng.next_f64()).collect();

        Layer { weights, biases }
    }

    fn forward(&self, inputs: &[f64]) -> Vec<f64> {
        self.weights.iter().zip(self.biases.iter())
            .map(|(neuron_weights, bias)| {
                let sum: f64 = neuron_weights.iter().zip(inputs.iter())
                    .map(|(w, x)| w * x)
                    .sum();
                sigmoid(sum + bias)
            })
            .collect()
    }
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn main() {
    let mut rng = SimpleRng::new(42);
    let mut net = NeuralNetwork::new(2, 4, 1, &mut rng);    // 2 entradas, 4 neuronas ocultas, 1 salida

    let dataset = [
        ([0.0, 0.0], 0.0),
        ([0.0, 1.0], 1.0),
        ([1.0, 0.0], 1.0),
        ([1.0, 1.0], 0.0),
    ];

    let epochs = 10000;
    let learning_rate = 0.5;

    for _ in 0..epochs {
        for (inputs, target) in dataset.iter() {
            net.train_step(inputs,  *target, learning_rate);
        }
    }

    println!("Resultados despues de {} épocas:", epochs);
    for (inputs, target) in dataset.iter() {
        let (_h, output) = net.forward(inputs);
        println!("{:?} -> {:.4} (esperado: {})", inputs, output[0], target);
    }
}