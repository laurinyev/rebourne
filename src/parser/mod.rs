mod lexer;
mod defines;
mod peeker;
mod parser;

pub use defines::*;

pub fn parse<'a>(cmd: &'a str) -> AstNode {
    let lexed = lexer::lex(cmd);
    
    #[cfg(feature = "debug_lexer")]
    println!("{lexed:?}");
    
    let peeker = peeker::Peeker::new(lexed);
    let root = parser::parse_sequence(peeker);
    
    #[cfg(feature = "debug_parser")]
    println!("{:?}",root);
    
    return root;
}
