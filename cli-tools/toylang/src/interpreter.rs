use std::collections::HashMap;
use crate::parser::{Statement, Expr};

pub struct Interpreter {
    env: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: HashMap::new() }
    }

    fn eval_expr(&self, expr: &Expr) -> i64 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Ident(name) => {
                *self.env.get(name).unwrap_or_else(|| panic!("Variable no definida: {}", name))
            }
            Expr::Add(left, right) => {
                self.eval_expr(left) + self.eval_expr(right)
            }
        }
    }

    pub fn run(&mut self, program: &[Statement]) {
        for statement in program {
            match statement {
                Statement::Let(name, expr) => {
                    let value = self.eval_expr(expr);
                    self.env.insert(name.clone(), value);
                }
                Statement::Print(expr) => {
                    let value = self.eval_expr(expr);
                    println!("{}", value);
                }
            }
        }
    }
}