use pest::{iterators::{Pair, Pairs}, pratt_parser::{Assoc, Op, PrattParser}};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "test.pest"]
pub struct ExprParser;

fn pratt_parser() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left) | Op::infix(Rule::r#mod, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
        .op(Op::prefix(Rule::neg))
}

fn parse_expr(pair: Pair<Rule>, parser: &PrattParser<Rule>) -> ASTNode {
    parser
        .map_primary(|primary|{
            ASTNode::new(primary)
        }).parse(pair.into_inner())
}
fn create_expression(pair: Pair<Rule>) -> ASTNode {
    let parser = pratt_parser();
    parse_expr(pair, &parser)
}
enum ASTNode{
    Program(Vec<ASTNode>),
    FunctionDecl{
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>
    },
    Function{
        name: String,
        args: Vec<ASTNode>
    },
    Return(Box<ASTNode>),
    Print(Box<ASTNode>),
    Assignment{
        //HashMap
        name: String,
        value: Box<ASTNode>
    },
    Reassignment{
        name: String,
        value: Box<ASTNode>
    },
    UnaryOpeartion {
        op: String,
        expr: Box<ASTNode>
    },
    BinaryOperation{
        left: Box<ASTNode>,
        op: String,
        right: Box<ASTNode>
    },
    Expression(Box<ASTNode>),
    Identifier(String),
    Number(i32)

}
impl ASTNode{
    fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
                Rule::function => {
                    let mut inner = pair.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();

                    let params = inner.next().unwrap()
                        .into_inner()
                        .map(|param| param.as_str().to_string())
                        .collect::<Vec<_>>();
                    let body = Box::new(Self::new(inner.next().unwrap()));

                    ASTNode::FunctionDecl{name, params, body}
                },
                Rule::function_call => {
                    let mut inner = pair.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();

                    let args = inner.next().unwrap()
                        .into_inner()
                        .map(|arg| ASTNode::new(arg)) 
                        .collect::<Vec<_>>();
                    ASTNode::Function { name, args }  
                },
                Rule::assignment => {
                    let mut inner = pair.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    let value = Box::new(ASTNode::new(inner.next().unwrap()));
                    ASTNode::Assignment { name, value }
                },
                Rule::reassignment => {
                    let mut inner = pair.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    let value = Box::new(ASTNode::new(inner.next().unwrap()));
                    ASTNode::Reassignment { name, value }
                },
                Rule::print => {
                    let node =  ASTNode::new(pair.into_inner().next().unwrap());
                    ASTNode::Print(Box::new(node))

                },
                Rule::r#return => {
                    let node =  ASTNode::new(pair.into_inner().next().unwrap());
                    ASTNode::Print(Box::new(node))
                },
                Rule::identifier => (
                   unimplemented!() 
                ),
                Rule::expression => {
                    unimplemented!()
                },
                _ => unreachable!()
        }
    }
}

struct Program {
    code: ASTNode
}

impl Program {
    fn new(pair: Pair<Rule>) -> Self {
        //Handle non programs
        if pair.as_rule() != Rule::program{
            return Self {code: ASTNode::Program(vec![])};
        }
        let statements = pair.into_inner().filter(|x| x.as_rule() != Rule::EOI)
            .map(|p|ASTNode::new(p)).collect();
        Self { code: ASTNode::Program(statements) }
    } 
}

