use std::collections::HashMap;

use pest::{iterators::{Pair, Pairs}, pratt_parser::{Assoc, Op, PrattParser}, Parser};

use pest_derive::Parser;

fn print_pair(pair: &Pair<Rule>) {
    println!("{:?}", pair.as_rule());
    println!("{}", pair.as_str());
    println!("{:=>20}", "");
}

#[derive(Parser)]
#[grammar = "test.pest"]
pub struct ExprParser;

//To make my life really easy, all functions are pure, so there is no need to access global
//state
struct Function{
    pub loc: usize,
    pub params:Vec<String> 
}
impl Function {
    pub fn new(loc: usize, params: Vec<String>) -> Self {
        Self{
            loc,
            params
        }
    }
}
pub struct Runtime {
    code: String, 
    varibles: HashMap<String, i32>,
    functions: HashMap<String, Function>,
    pointer: usize,
    pointer_stack: Vec<usize>
}

impl Runtime {
    pub fn new(code: String) -> Self {
       Self {
           code,
           varibles: HashMap::new(),
           functions: HashMap::new(),
           pointer: 0,
           pointer_stack: vec![]
       } 
    }
    fn resolve_expr(&mut self, pairs: Pairs<Rule>) -> i32 {
        
        let pratt_parser = PrattParser::new()
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left) | Op::infix(Rule::r#mod, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
            .op(Op::prefix(Rule::neg));
        self.parse_expr(pairs, &pratt_parser)
    }

    fn parse_expr(&mut self, pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> i32 {
        pratt
            .map_primary(|primary| match primary.as_rule(){
                Rule::int => primary.as_str().parse().unwrap(),
                Rule::expression => self.parse_expr(primary.into_inner(), pratt),
                Rule::identifier => {
                    match self.varibles.get(primary.as_str()){
                        None => unimplemented!("unable to handle error"),
                        Some(num) => num.to_owned(),
                    }
                } 
                _ => unreachable!("Failed to parse {}", primary.as_str())
            }).map_prefix(|op, rhs| match op.as_rule() {
               Rule::neg => -rhs,
               _ => unreachable!()
            }).map_infix(|lhs, op, rhs|match op.as_rule(){
                Rule::add => lhs + rhs,
                Rule::sub => lhs - rhs,
                Rule::mul => lhs * rhs,
                Rule::div => lhs / rhs,
                Rule::r#mod => lhs % rhs,
                _ => unreachable!() 
            }).parse(pairs)
    }
    fn def_func(&mut self, pair: &Pair<Rule>){
        let mut pairs = pair.clone().into_inner();
        let ident = String::from(pairs.next().unwrap().as_str());
        let mut params: Vec<String> = vec![];
        let mut param_iter = pairs.next().unwrap().into_inner();
        while let Some(param) = param_iter.next() {
            params.push(String::from(param.as_str()))
        }
        let function = Function::new(self.pointer, params);
        self.functions.insert(ident, function); 
    }
    fn assign(&mut self, pair: &Pair<Rule>) {
        let mut iter = pair.clone().into_inner();
        let ident = String::from(iter.next().expect("Failed parsing assignment").as_str());
        let value= self.resolve_expr(iter.next().unwrap().into_inner());
        self.varibles.insert(ident, value);
    }
    fn print_expression(&mut self, pair: &Pair<Rule>){
        let expr = self.resolve_expr(pair.clone().into_inner());
        println!("{}", expr);
    }

    pub fn execute(&mut self) -> Result<Option<i32>, String> {
        let code = self.code.clone();
        let pairs:Vec<Pair<Rule>> = ExprParser::parse(Rule::program,&code).unwrap().next().unwrap().into_inner().collect();
        while let Some(pair) = pairs.get(self.pointer){
            match pair.as_rule() {
                Rule::function => {self.def_func(pair)}, //Mark the start of the function in the
                                                      //HashMap
                Rule::assignment => {self.assign(&pair)},
                Rule::reassignment => {unimplemented!()},
                Rule::function_call => {unimplemented!()},//Move to the functions with the same
                Rule::print => {self.print_expression(&pair)},
                Rule::r#return => {},
                Rule::identifier => (),
                Rule::expression => (),
                Rule::EOI => break,
                _ => unreachable!()
            }
            self.pointer += 1;
        }
        Ok(None)
    }
}


