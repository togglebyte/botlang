use botlang::context::Context;
use botlang::lexer::lex;
use botlang::pratt::parse;
use botlang::eval::eval;

fn main() {
    let mut ctx = Context::new(());

    let src = "!test2";
    let tokens = lex(src).unwrap();
    let expr = parse(tokens).unwrap();
    eprintln!("{expr:?}");
    let expr = eval(expr, &mut ctx, "bob");

    eprintln!("{expr:?}");
}
