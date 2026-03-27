#[derive(Debug ,PartialEq, Eq, Clone)]
pub enum LexToken {
    Word(String),
    Newline,
    Semicolon,
    Eq,
    And,
    Bg,
    Or,
    EOF
}

#[derive(Debug,PartialEq, Eq)]
pub enum AstNode {
    Sequence(Vec<AstNode>),
    ConstantString(String),
    Command(Box<AstNode>,Vec<AstNode>,bool),
    And(Box<AstNode>,Box<AstNode>),
    Or(Box<AstNode>,Box<AstNode>),
    EnvVarSet(Box<AstNode>,Box<AstNode>),
    ParseEnd,
}
