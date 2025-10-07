#![allow(dead_code)]
#![allow(unused_variables)]
use std::{char, process::exit, vec};

use regex::Regex;

#[derive(Clone, Copy)]
enum Data {
    Int(i32),
    Char(char),
    Float(f32),
}

#[derive(Debug)]
enum Func {
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Input,
    DPrint,
    Dup,
    Swap,
}

#[derive(Debug)]
enum Token {
    Int(i32),
    Char(char),
    Float(f32),
    Func(Func),
    Keyword(String),
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Int(x) => write!(f, "Int({x})"),
            Data::Char(x) => write!(f, "Char({x})"),
            Data::Float(x) => write!(f, "Float({x})"),
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Int(x) => write!(f, "{x}"),
            Data::Char(x) => write!(f, "{x}"),
            Data::Float(x) => write!(f, "{x}"),
        }
    }
}

// type Stack = Vec<Data>;

pub enum StackError {
    EmptyStack,
}

pub trait Stack<T> {
    fn dup(&mut self) -> Result<usize, StackError>;

    fn pop(&mut self) -> Option<T>;

    fn swap(&mut self) -> Result<(), StackError>;
    fn push(&mut self, val: T) -> usize;
}

impl<T> Stack<T> for Vec<T>
where
    T: Clone,
{
    fn dup(&mut self) -> Result<usize, StackError> {
        let data = self.pop();
        match data {
            Some(x) => {
                self.push(x.clone());
                self.push(x);
            }
            None => {
                return Err(StackError::EmptyStack);
            }
        }
        Ok(self.len())
    }

    fn swap(&mut self) -> Result<(), StackError> {
        let a_opt = self.pop();
        match a_opt {
            Some(a) => {
                let b_opt = self.pop();
                if let Some(b) = b_opt {
                    self.push(a);
                    self.push(b);
                    return Ok(());
                } else {
                    return Err(StackError::EmptyStack);
                }
            }
            None => {
                return Err(StackError::EmptyStack);
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn push(&mut self, val: T) -> usize {
        self.push(val);
        self.len()
    }
}

fn print_stack(stack: &Vec<Data>) {
    for data in stack {
        print!("{data} ");
    }
    println!("<- Top");
}

#[derive(Debug)]
enum ParseError {
    InvalidToken(String),
}

enum ExecutionError {
    EmptyStack,
    ImproperArgument,
    DivideByZero,
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::EmptyStack => write!(f, "Empty stack error"),
            ExecutionError::DivideByZero => write!(f, "Attempt to divide by zero"),
            ExecutionError::ImproperArgument => write!(f, "Improper arguments provided"),
        }
    }
}

fn parse_token(token: &str) -> Result<Token, ParseError> {
    // large regex :#
    let re = Regex::new(r"(?<float>\d+\.\d+)|(?<int>\d+)|(?<char>'\\?.)|(?<ops>\+|\-|\/|\*|\||\$|\.|\#|\:)|(?<keyword>[A-Z]\w+)").unwrap();

    if let Some(capture) = re.captures(token) {
        // println!("{:?}", capture);
        if let Some(float) = capture.name("float") {
            // Read float
            todo!();
        }
        if let Some(int) = capture.name("int") {
            // read int
            return Ok(Token::Int(int.as_str().parse().unwrap()));
        }
        if let Some(character) = capture.name("char") {
            // read char
            let (_, character) = character.as_str().split_at(1);
            let character = character.replace("\\n", "\n");
            let character = character.replace("\\t", "\t");
            let character = character.parse::<char>();
            if let Ok(character) = character {
                return Ok(
                    Token::Char(
                        character
                    )
                )
            }
            else {
                if let Err(e) = character {
                    panic!("{:?}", e);
                }
                return Err(ParseError::InvalidToken(token.to_owned()));
            }
            
        }
        if let Some(op) = capture.name("ops") {
            // read op
            return match op.as_str() {
                "+" => Ok(Token::Func(Func::Add)),
                "-" => Ok(Token::Func(Func::Sub)),
                "/" => Ok(Token::Func(Func::Div)),
                "*" => Ok(Token::Func(Func::Mul)),
                "|" => Ok(Token::Func(Func::Input)),
                "$" => Ok(Token::Func(Func::Print)),
                "." => Ok(Token::Func(Func::Dup)),
                ":" => Ok(Token::Func(Func::Swap)),
                "#" => Ok(Token::Func(Func::DPrint)),
                x => Err(ParseError::InvalidToken(x.to_owned())),
            };
        }
        if let Some(keyword) = capture.name("keyword") {
            // perform a keyword search in internal hashmap
            todo!();
        }
    }

    Err(ParseError::InvalidToken(token.to_owned()))
}

fn execute_token<T: Stack<Data>>(token: Token, stack: &mut T) -> Result<(), ExecutionError> {
    match token {
        Token::Int(x) => {
            stack.push(Data::Int(x));
        }
        Token::Char(x) => {
            stack.push(Data::Char(x));
        },
        Token::Float(_) => todo!(),
        Token::Func(func) => match func {
            Func::Add => {
                let x_opt = stack.pop();
                let y_opt = stack.pop();
                if let None = x_opt {
                    return Err(ExecutionError::EmptyStack);
                }
                if let None = y_opt {
                    return Err(ExecutionError::EmptyStack);
                }
                if let Some(Data::Int(x)) = x_opt {
                    if let Some(Data::Int(y)) = y_opt {
                        stack.push(Data::Int(x + y));
                        return Ok(());
                    } else {
                        return Err(ExecutionError::ImproperArgument);
                    }
                } else {
                    return Err(ExecutionError::ImproperArgument);
                }
            }
            Func::Sub => {
                let x_opt = stack.pop();
                let y_opt = stack.pop();
                if let None = x_opt {
                    return Err(ExecutionError::EmptyStack);
                }
                if let None = y_opt {
                    return Err(ExecutionError::EmptyStack);
                }
                if let Some(Data::Int(x)) = x_opt {
                    if let Some(Data::Int(y)) = y_opt {
                        stack.push(Data::Int(x - y));
                        return Ok(());
                    } else {
                        return Err(ExecutionError::ImproperArgument);
                    }
                } else {
                    return Err(ExecutionError::ImproperArgument);
                }
            }
            Func::Mul => {
                let x_opt = stack.pop();
                let y_opt = stack.pop();
                if let None = x_opt {
                    return Err(ExecutionError::EmptyStack);
                }
                if let None = y_opt {
                    return Err(ExecutionError::EmptyStack);
                }
                if let Some(Data::Int(x)) = x_opt {
                    if let Some(Data::Int(y)) = y_opt {
                        stack.push(Data::Int(x * y));
                        return Ok(());
                    } else {
                        return Err(ExecutionError::ImproperArgument);
                    }
                } else {
                    return Err(ExecutionError::ImproperArgument);
                }
            }
            Func::Div => todo!(),
            Func::Print => {
                let val_opt = stack.pop();
                if let Some(val) = val_opt {
                    print!("{}", val);
                    return Ok(());
                } else {
                    return Err(ExecutionError::EmptyStack);
                }
            }
            Func::Input => todo!(),
            Func::DPrint => {
                let val_opt = stack.pop();
                if let Some(val) = val_opt {
                    println!("{:?}", val);
                    return Ok(());
                } else {
                    return Err(ExecutionError::EmptyStack);
                }
            }
            Func::Dup => {
                if let Err(x) = stack.dup() {
                    return Err(ExecutionError::EmptyStack);
                }
            }
            Func::Swap => {
                if let Err(x) = stack.swap() {
                    return Err(ExecutionError::EmptyStack);
                }
            },
        },
        Token::Keyword(_) => todo!(),
    }
    Ok(())
}

fn main() {
    let mut stack: Vec<Data> = vec![];

    let file = std::fs::read_to_string("./test.forth").unwrap();

    // let test: &str = "1 2 3 + + . $";
    for str_token in file.split_ascii_whitespace() {
        // println!("{str_token}");
        let token = parse_token(str_token).expect("Parse Error");
        if let Err(error) = execute_token(token, &mut stack) {
            println!("Execution error: {error}");
            exit(1);
        }
    }
    println!();
    print_stack(&stack);
}
