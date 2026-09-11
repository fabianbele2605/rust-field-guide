use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
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

    pub fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{:.1} ", self.get(r, c));
            }
            println!();
        }
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
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows, "Filas no coinciden");
        assert_eq!(self.cols, other.cols, "Columnas no coinciden");

        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Matrix::from_vec(self.rows, self.cols, data)
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows, "Filas no coinciden");
        assert_eq!(self.cols, other.cols, "Columnas no coinciden");

        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        Matrix::from_vec(self.rows, self.cols, data)
    }
}

fn main() {
    let m1 = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::from_vec(2, 2, vec![10.0, 20.0, 30.0, 40.0]);

    println!("m1:");
    m1.print();
    println!("m2:");
    m2.print();

    let suma = m1 + m2;
    println!("m1 + m2:");
    suma.print();

    let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    let producto = a.multiply(&b);
    println!("a x b:");
    producto.print();

    let c = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    println!("c:");
    c.print();
    println!("c transpuesta:");
    c.transpose().print();

    let d1 = Matrix::from_vec(2, 2, vec![10.0, 20.0, 30.0, 40.0]);
    let d2 = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    println!("d1 - d2:");
    (d1 - d2).print();
}