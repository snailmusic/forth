#![allow(dead_code)]
#![allow(unused_variables)]
use std::{char, collections::HashMap, process::exit, vec};

use pest::Parser;
use pest_derive::Parser;

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
    Over,
    Rotate,
    Store,
    Retrieve,
}

#[derive(Debug)]
enum Reserved {
    Variable(String),
    Constant(String),
}

#[derive(Debug)]
enum Token {
    Int(i32),
    Char(char),
    Float(f32),
    Func(Func),
    Reserved(Reserved),
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

    fn over(&mut self) -> Result<usize, StackError>;
    fn rotate(&mut self) -> Result<(), StackError>;
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

    fn over(&mut self) -> Result<usize, StackError> {
        let x_opt = self.pop();
        let y_opt = self.pop();
        if let Some(x) = x_opt {
            if let Some(y) = y_opt {
                self.push(y.clone());
                self.push(x);
                self.push(y);
                return Ok(self.len());
            } else {
                return Err(StackError::EmptyStack);
            }
        } else {
            return Err(StackError::EmptyStack);
        }
    }

    fn rotate(&mut self) -> Result<(), StackError> {
        if let (Some(x), Some(y), Some(z)) = (self.pop(), self.pop(), self.pop()) {
            self.push(y);
            self.push(x);
            self.push(z);
            return Ok(());
        } else {
            return Err(StackError::EmptyStack);
        }
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
    InvalidWord(String),
    InvalidMemory,
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::EmptyStack => write!(f, "Empty stack error"),
            ExecutionError::DivideByZero => write!(f, "Attempt to divide by zero"),
            ExecutionError::ImproperArgument => write!(f, "Improper arguments provided"),
            ExecutionError::InvalidWord(word) => write!(f, "Invalid word: {word}"),
            ExecutionError::InvalidMemory => write!(f, "Memory location is invalid"),
        }
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ForthParser;

fn parse_token(token: pest::iterators::Pair<'_, Rule>) -> Result<Token, ParseError> {
    let token_str = token.as_str().to_owned();

    let token_parse = token.into_inner().next();

    if let Some(token_parse) = token_parse {
        // println!("{}", token_parse.as_str());
        match token_parse.as_rule() {
            Rule::float => todo!(),
            Rule::int => {
                return Ok(Token::Int(token_parse.as_str().parse().unwrap()));
            }
            Rule::char => {
                let character = token_parse.as_str();
                let (_, character) = character.split_at(1);
                let character = character.replace("\\n", "\n");
                let character = character.replace("\\t", "\t");
                let character = character.parse::<char>();
                if let Ok(character) = character {
                    return Ok(Token::Char(character));
                } else {
                    if let Err(e) = character {
                        panic!("{:?}", e);
                    }
                    return Err(ParseError::InvalidToken(token_str));
                }
            }
            Rule::operator => {
                let operator = token_parse.into_inner().next().unwrap();
                return match operator.as_rule() {
                    Rule::add => Ok(Token::Func(Func::Add)),
                    Rule::sub => Ok(Token::Func(Func::Sub)),
                    Rule::mul => Ok(Token::Func(Func::Mul)),
                    Rule::div => Ok(Token::Func(Func::Div)),
                    Rule::inp => Ok(Token::Func(Func::Input)),
                    Rule::print => Ok(Token::Func(Func::Print)),
                    Rule::debugprint => Ok(Token::Func(Func::DPrint)),
                    Rule::dup => Ok(Token::Func(Func::Dup)),
                    Rule::swp => Ok(Token::Func(Func::Swap)),
                    Rule::ovr => Ok(Token::Func(Func::Over)),
                    Rule::rot => Ok(Token::Func(Func::Rotate)),
                    Rule::store => Ok(Token::Func(Func::Store)),
                    Rule::retrieve => Ok(Token::Func(Func::Retrieve)),
                    _ => panic!("WHAT THE FUCK"),
                };
            }
            Rule::reservedKeyword => {
                let kw = token_parse.into_inner().next().unwrap();
                match kw.as_rule() {
                    Rule::variable => {
                        let name = kw.into_inner().next().unwrap();
                        let name = name.as_str();
                        return Ok(Token::Reserved(Reserved::Variable(name.to_owned())));
                    },
                    Rule::constant => {
                        let name = kw.into_inner().next().unwrap();
                        let name = name.as_str();
                        return Ok(Token::Reserved(Reserved::Constant(name.to_owned())));
                    }
                    _ => panic!("WHAT THE FUCK"),
                }
            }
            Rule::keyword => {
                return Ok(Token::Keyword(token_parse.as_str().to_owned()));
            }
            _ => return Err(ParseError::InvalidToken(token_str)),
        }
    }

    Err(ParseError::InvalidToken(token_str))
}

fn execute_token<T: Stack<Data>>(
    token: Token,
    stack: &mut T,
    state: &mut HashMap<String, Word>,
    memory: &mut Vec<Option<Data>>,
) -> Result<(), ExecutionError> {
    match token {
        Token::Int(x) => {
            stack.push(Data::Int(x));
        }
        Token::Char(x) => {
            stack.push(Data::Char(x));
        }
        Token::Float(x) => {
            stack.push(Data::Float(x));
        },
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
            }
            Func::Over => {
                if let Err(x) = stack.over() {
                    return Err(ExecutionError::EmptyStack);
                }
            }
            Func::Rotate => {
                if let Err(x) = stack.rotate() {
                    return Err(ExecutionError::EmptyStack);
                }
            }
            Func::Store => {
                if let (Some(Data::Int(memory_loc)), Some(val)) = (stack.pop(), stack.pop()) {
                    memory[memory_loc as usize] = Some(val);
                } else {
                    return Err(ExecutionError::ImproperArgument);
                }
            }
            Func::Retrieve => {
                if let Some(Data::Int(memory_loc)) = stack.pop() {
                    if let Some(data) = memory[memory_loc as usize] {
                        stack.push(data);
                    } else {
                        return Err(ExecutionError::InvalidMemory);
                    }
                } else {
                    return Err(ExecutionError::ImproperArgument);
                }
            }
        },
        Token::Reserved(reserved) => match reserved {
            Reserved::Variable(keyword) => {
                let memory_loc = memory.len() as i32;
                state.insert(keyword, Word::Data(Data::Int(memory_loc)));
                memory.push(None);
            }
            Reserved::Constant(keyword) => {
                if let Some(data) = stack.pop() {
                    state.insert(keyword, Word::Data(data));
                }
                else {
                    return Err(ExecutionError::EmptyStack)
                }
            },
        },
        Token::Keyword(word) => {
            if let Some(word) = state.get(&word) {
                match word {
                    Word::Word(pair) => todo!(),
                    Word::Data(data) => {
                        stack.push(*data);
                    }
                }
            } else {
                return Err(ExecutionError::InvalidWord(word));
            }
        }
    }
    Ok(())
}

enum Word<'a> {
    // store ast of the thing lmao
    Word(pest::iterators::Pair<'a, Rule>),
    // for accessing memory
    Data(Data),
}

fn main() {
    let mut stack: Vec<Data> = vec![];
    let mut state: HashMap<String, Word> = HashMap::new();
    let mut memory: Vec<Option<Data>> = vec![];

    let file = std::fs::read_to_string("./test.forth").unwrap();

    // let test: &str = "1 2 3 + + . $";
    let ast = ForthParser::parse(Rule::tokens, file.as_str())
        .unwrap()
        .next()
        .unwrap();
    // println!("{:?}", ast);
    for token in ast.into_inner() {
        let token = parse_token(token).expect("Parse Error");
        if let Err(error) = execute_token(token, &mut stack, &mut state, &mut memory) {
            println!("Execution error: {error}");
            exit(1);
        }
    }
    // for str_token in file.split_ascii_whitespace() {
    //     // println!("{str_token}");
    //     let token = parse_token(str_token).expect("Parse Error");
    //     if let Err(error) = execute_token(token, &mut stack) {
    //         println!("Execution error: {error}");
    //         exit(1);
    //     }
    // }
    println!();
    print_stack(&stack);
}
