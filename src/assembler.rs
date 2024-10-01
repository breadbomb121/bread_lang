use std::collections::HashMap;
use std::fmt::Display;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INSTRUCTION_MAP: HashMap<&'static str, i64> = {
        let mut m = HashMap::new();
        m.insert("read", 0x0);
        m.insert("dv", 0x0);
        m.insert("write", 0x1);
        m.insert("load", 0x2);
        m.insert("loadmem", 0x3);
        m.insert("store", 0x4);
        m.insert("storemem", 0x5);
        m.insert("add", 0x6);
        m.insert("addi", 0x7);
        m.insert("sub", 0x8);
        m.insert("subi", 0x9);
        m.insert("mul", 0xA);
        m.insert("muli", 0xB);
        m.insert("div", 0xC);
        m.insert("divi", 0xD);
        m.insert("mod", 0xE);
        m.insert("modi", 0xF);
        m.insert("and", 0x10);
        m.insert("or", 0x11);
        m.insert("not", 0x12);
        m.insert("teststatus", 0x13);
        m.insert("clear", 0x14);
        m.insert("jumpifnotzero", 0x15);
        m.insert("jumpifzero", 0x16);
        m.insert("jump", 0x17);
        m.insert("halt", 0x18);
        m.insert("return", 0x19);
        m.insert("call", 0x1A);
        m.insert("incsp", 0x1B);
        m.insert("loadrel", 0x1C);
        m.insert("storerel", 0x1D);
        m
    };
}
#[derive(Debug)]
pub enum AssemblerError{
    InvalidLabel(String)
}
impl Display for AssemblerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::InvalidLabel(label) => {
                write!(f, "Error: invalid Label: {}", label)
            }
        }
    }
}
fn operation_to_code(val: &str) -> Option<i64> {
    return Some(INSTRUCTION_MAP.get(val.trim())?.to_owned());
}
#[derive(Clone, Debug, PartialEq)]
enum Token {
    Label(String),
    Operation(i64),
    Operand(i64),
    Mode(i64)
}
pub fn assemble(assembly_code: String) -> Result<Vec<i64>, AssemblerError> {
    let mut token_code: Vec<Vec<Token>> = Vec::new();
    let mut symbol_hash_table: HashMap<String, usize> = HashMap::new();
    //Tokenize Code
    for (i, line) in assembly_code.split("\n").into_iter().enumerate(){
        if line.trim().is_empty(){
            continue;
        }
        let mut token_line: Vec<Token> = Vec::new();
        let mut current_line: Vec<String> = line.split(" ").into_iter().map(|f| f.trim().to_lowercase().to_owned()).collect();
        if current_line.len() <= 0 {
            continue;
        }
        //Remove the label from the front if it is there 
        let first_val = current_line.get(0).unwrap_or(&String::new()).to_owned();
        if let None = operation_to_code(&first_val){//Check if the first value is not a valid operation
            symbol_hash_table.insert(first_val, i);
            current_line.remove(0);
        }
        if current_line.len() <= 0 {
            continue;
        }
        //Tokenize the code
        while let Some(val) = current_line.get(0){
            //Check if we have an operation
            if let Some(operation) = operation_to_code(val.trim()){
                token_line.push(Token::Operation(operation));
            }else if let Ok(num) = i64::from_str_radix(val.trim(), 16){
                token_line.push(Token::Operand(num));
            }else {
                match val.as_str() {
                    "a" => {token_line.push(Token::Mode(0))},
                    "b" => {token_line.push(Token::Mode(1))},
                    _ => {token_line.push(Token::Label(val.to_string()))}
                }
            }
            current_line.remove(0);
        };
        token_code.push(token_line);
    }
    //Pass over the code and change labels for operands
    for line in token_code.iter_mut() {
        for token in line.iter_mut(){
            //The current token is a label and should be changed to an operand
            if let Token::Label(ref label) = token {
                if let Some(num) = symbol_hash_table.get(&label.trim().to_lowercase()){
                    *token = Token::Operand(num.to_owned() as i64);
                }else {
                    //We have an unknown label
                    return Err(AssemblerError::InvalidLabel(label.to_string()));
                };
            };
        }
    };
    let mut code: Vec<i64> = Vec::new();
    //Turn each line into an i64
    for line in token_code{
        let mut op: i64 = 0;
        let mode: i64 = 0;
        let mut num: i64 = 0;
        for val in line {
            match val {
                Token::Operation(new_op) => {op = new_op},
                Token::Mode(mode) => {op = mode},
                Token::Operand(operand) => {
                    if operand < 0 {
                        num = 0x100 - operand.abs();
                    }else {
                        num = operand
                    }
                     
                },
                _ => {println!("INVALID TOKEN")}//Error should probably be thrown
            }
        }
        let code_op = ((op << 1) + mode) * 0x100;
        code.push(code_op + num);
    }
    Ok(code)
}
