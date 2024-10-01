use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "test.pest"]
pub struct ExprParser;


enum ASTNode{
    Program(Vec<AstNode>),
    FunctionDecl{
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>
    },
    Return(),
    Print(),
    Assignment{},
    Reassignment{},
    Expression(),
    Identifier()
}

