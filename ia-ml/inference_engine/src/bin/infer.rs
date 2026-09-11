use std::fs::File;
use std::io::Read;
use std::time::Instant;

struct Layer {
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

struct NeuralNetwork {
    hidden: Layer,
    output: Layer,
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

impl NeuralNetwork {
    fn forward(&self, inputs: &[f64]) -> Vec<f64> {
        let hidden_output: Vec<f64> = self.hidden.weights.iter().zip(self.hidden.biases.iter())
            .map(|(w, b)| {
                let sum: f64 = w.iter().zip(inputs.iter()).map(|(wi, xi)| wi * xi).sum();
                sigmoid(sum + b)
            })
            .collect();

        self.output.weights.iter().zip(self.output.biases.iter())
            .map(|(w, b)| {
                let sum: f64 = w.iter().zip(hidden_output.iter()).map(|(wi, xi)| wi * xi).sum();
                sigmoid(sum + b)
            })
            .collect()
    }

    fn load(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let mut pos = 0;
        let read_u32 = |buf: &[u8], pos: &mut usize| -> u32 {
            let v = u32::from_le_bytes(buf[*pos..*pos + 4].try_into().unwrap());
            *pos += 4;
            v
        };
        let read_f64 = |buf: &[u8], pos: &mut usize| -> f64 {
            let v = f64::from_le_bytes(buf[*pos..*pos + 8].try_into().unwrap());
            *pos += 8;
            v
        };

        let n_inputs = read_u32(&buf, &mut pos) as usize;
        let n_hidden = read_u32(&buf, &mut pos) as usize;
        let n_outputs = read_u32(&buf, &mut pos) as usize;

        let mut hidden_weights = Vec::new();
        for _ in 0..n_hidden {
            let mut row = Vec::new();
            for _ in 0..n_inputs {
                row.push(read_f64(&buf, &mut pos));
            }
            hidden_weights.push(row);
        }
        let hidden_biases: Vec<f64> = (0..n_hidden).map(|_| read_f64(&buf, &mut pos)).collect();

        let mut output_weights = Vec::new();
        for _ in 0..n_outputs {
            let mut row = Vec::new();
            for _ in 0..n_hidden {
                row.push(read_f64(&buf, &mut pos));
            }
            output_weights.push(row);
        }
        let output_biases: Vec<f64> = (0..n_outputs).map(|_| read_f64(&buf, &mut pos)).collect();

        Ok(NeuralNetwork {
            hidden: Layer { weights: hidden_weights, biases: hidden_biases },
            output: Layer { weights: output_weights, biases: output_biases },
        })
    }

    fn forward_barch(&self, bath: &[[f64; 2]]) -> Vec<f64> {
        bath.iter().map(|inputs| self.forward(inputs)[0]).collect()
    }
}

fn main() {
    let net = NeuralNetwork::load("model.bin").expect("No se pudo cargar model.bin");
    println!("Modelo cargado. Haciendo inferencia (sin entrenar):");

    let batch = [[0.0, 0.0],[0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

    let start = Instant::now();
    let results = net.forward_barch(&batch);
    let elapsed = start.elapsed();

    for (inputs, output) in batch.iter().zip(results.iter()) {
        println!("{:?} -> {:.4}", inputs, output);
    }
    println!("Tiempo de inferencia del batch: {:?}", elapsed);
}
