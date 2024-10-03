use pest::{iterators::Pairs, pratt_parser::PrattParser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "test.pest"]
pub struct ExprParser;


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
    Return(ASTNode),
    Print(ASTNode),
    Assignment{
        //HashMap
        name: String,
        value: Box<ASTNode>
    },
    Reassignment{
        name: String,
        value: Box<ASTNode>
    },
    Expression(Box<ASTNode>),
    Identifier(String),
    Number(i32)

}
impl ASTNode{
    fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
                Rule::function => {
                    let inner = pair.into_inner();
                    let name = inner.next().as_str().to_string();

                    let params = inner.next().unwrap()
                        .into_inner()
                        .map(|param| param.as_str().to_string())
                        .collect::<Vec<_>>();
                    let body = Box::new(Self::new(inner.next().unwrap()));

                    ASTNode::FuncitonDecl{name, params, body}
                },
                Rule::function_call => {
                    let inner = pair.into_inner();
                    let name = inner.next().as_str().to_string();

                    let args = inner.next().unwrap()
                        .into_inner()
                        .map(|arg| ASTNode::new(arg)) 
                        .collect::<Vec<_>>();
                    ASTNode::Function { name, args }  
                },
                Rule::assignment => {
                    let inner = pair.into_inner();
                    let name = inner.next().as_str().to_string();
                    let value = Box::new(inner.next.unwrap());
                    ASTNode::Assignment { name, value }
                },
                Rule::reassignment => {
                    let inner = pair.into_inner();
                    let name = inner.next().as_str().to_string();
                    let value = Box::new(inner.next.unwrap());
                    ASTNode::Reassignment { name, value }
                },
                Rule::print => {
                    let node =  ASTNode::new(pair.into_inner().next().unwrap());
                    ASTNode::Print(node)

                },
                Rule::r#return => {
                    let node =  ASTNode::new(pair.into_inner().next().unwrap());
                    ASTNode::Print(node)
                },
                Rule::identifier => (
                    
                ),
                Rule::expression => {

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
            return Self {code: AstNode::Program(vec![])};
        }
        let statements = pair.into_inner().unwrap().filter(|x| x.as_rule() != Rule::EOI)
            .map(|p|{
               match p.as_rule() {
                _ => unreachable!()
               } 
            });
        

    } 
}

