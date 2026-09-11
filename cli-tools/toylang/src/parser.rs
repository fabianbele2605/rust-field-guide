use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expr),
    Print(Expr),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned()
            .unwrap_or_else(|| panic!("Fin de archivo inesperado (¿falta un ';' o ')'?)"));
        self.pos += 1;
        tok
    }

    // Parsea una expresión simple: número, identicador, o "algo + algo"
    fn parse_expr(&mut self) -> Expr {
        let left = match self.advance() {
            Token::Number(n) => Expr::Number(n),
            Token::Ident(name) => Expr::Ident(name),
            other => panic!("Token inesperado en expresion: {:?}", other),
        };

        if self.current() == Some(&Token::Plus) {
            self.advance();  // consume el '+'
            let right = self.parse_expr();
            return Expr::Add(Box::new(left), Box::new(right));
        }

        left
    }

    fn parse_statement(&mut self) -> Statement {
        match self.advance() {
            Token::Let => {
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    other => panic!("Se esperaba un identificador, llegó: {:?}", other),
                };
                self.advance();  // consume '='
                let expr = self.parse_expr();
                self.advance();  // consume ';'
                Statement::Let(name, expr)
            }
            Token::Print => {
                self.advance();  // consume '('
                let expr = self.parse_expr();
                self.advance();  // consume ')'
                self.advance();  // consume ';'
                Statement::Print(expr)
            }
            other => panic!("Statement inesperado: {:?}", other),
        }
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.pos < self.tokens.len() {
            statements.push(self.parse_statement());
        }
        statements
    }
}