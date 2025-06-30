use std::{
  fmt,
  io::{self, Write},
};

fn main() {
  let mut stdout = io::stdout();
  let stdin = io::stdin();
  loop {
    print!(">> ");
    stdout.flush().unwrap();

    let line = {
      let mut buffer = String::new();
      stdin.read_line(&mut buffer).unwrap();
      buffer
    };

    if line.trim() == "exit" {
      break;
    }

    let expr = Expr::from_str(&line);

    println!("{}", expr.eval());
  }
}

#[derive(Debug, Clone, Copy)]
enum Token {
  Atom(char),
  Op(char),
  Eof,
}

#[derive(Debug)]
struct Lexer {
  tokens: Vec<Token>,
}

impl Lexer {
  fn new(input: &str) -> Self {
    let mut tokens: Vec<Token> = input
      .chars()
      .filter(|c| !c.is_ascii_whitespace())
      .map(|c| match c {
        '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
        _ => Token::Op(c),
      })
      .collect();

    tokens.reverse();

    Self { tokens }
  }

  fn next(&mut self) -> Token {
    self.tokens.pop().unwrap_or(Token::Eof)
  }

  fn peek(&self) -> Token {
    self.tokens.last().copied().unwrap_or(Token::Eof)
  }
}

#[derive(Debug)]
enum Expr {
  Atom(char),
  Op(char, Vec<Expr>),
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Atom(i) => write!(f, "{}", i),
      Expr::Op(head, rest) => {
        write!(f, "({}", head)?;
        for s in rest {
          write!(f, " {}", s)?
        }
        write!(f, ")")
      }
    }
  }
}

impl Expr {
  fn from_str(input: &str) -> Self {
    let mut lexer = Lexer::new(input);
    Expr::parse_expr(&mut lexer, 0.0)
  }

  fn parse_expr(lexer: &mut Lexer, min_bp: f32) -> Self {
    let mut lhs = match lexer.next() {
      Token::Atom(c) => Expr::Atom(c),
      t => panic!("bad token: {:?}", t),
    };

    loop {
      let op = match lexer.peek() {
        Token::Eof => break,
        Token::Op(op) => op,
        _ => panic!("Bad token"),
      };

      let (l_bp, r_bp) = Self::infix_binding_power(op);

      if l_bp < min_bp {
        break;
      }

      lexer.next();

      let rhs = Self::parse_expr(lexer, r_bp);
      lhs = Expr::Op(op, vec![lhs, rhs]);
    }

    lhs
  }

  fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
      '+' | '-' => (1.0, 1.1),
      '*' | '/' => (2.0, 2.1),
      _ => panic!("bad op: {:?}", op),
    }
  }

  fn eval(&self) -> f32 {
    match self {
      Expr::Atom(atom) => match atom {
        '0'..='9' => atom.to_digit(10).unwrap() as f32,
        _ => unreachable!(),
      },
      Expr::Op(op, operands) => {
        let lhs = operands.first().unwrap().eval();
        let rhs = operands.last().unwrap().eval();
        match op {
          '+' => lhs + rhs,
          '-' => lhs - rhs,
          '*' => lhs * rhs,
          '/' => lhs / rhs,
          _ => panic!("Unsupported operator"),
        }
      }
    }
  }
}
