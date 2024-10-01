mod assembler;
mod runtime;
mod AST;
use runtime::Runtime;
const CODE: &'static str = r#"
    fn foo () {}
    let x = 20 + 32;
    let y = 20 * x;
    print y;
"#;
fn main() {
    let mut runtime = Runtime::new(String::from(CODE));
    let _ = runtime.execute();
}
