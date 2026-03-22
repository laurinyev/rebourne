#[derive(Debug ,PartialEq, Eq, Clone)]
enum LexToken {
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
    Command(Box<AstNode>,Vec<AstNode>),
    And(Box<AstNode>,Box<AstNode>),
    Or(Box<AstNode>,Box<AstNode>),
    EnvVarSet(Box<AstNode>,Box<AstNode>),
    ParseEnd,
}

#[derive(PartialEq,Clone, Copy)]
pub enum LexerMode {
    Normal,
    Comment,
    Escape,
    Qoute,
    DQoute
}

fn flush_buf(tokens: &mut Vec<LexToken>, buffer: &mut String, symbols: bool) {
    if buffer.len() < 1 {
        return;
    }

    match buffer.as_str() {
        "&&" if symbols => tokens.push(LexToken::And),
        "||" if symbols=> tokens.push(LexToken::Or),
        "&"  if symbols => tokens.push(LexToken::Bg),
        _    => tokens.push(LexToken::Word(buffer.clone()))
    }

    buffer.clear();
}

fn lex<'a>(cmd: &'a str) -> Vec<LexToken> {
    let mut tokens: Vec<LexToken> = vec![]; 
    let mut word_buffer = String::new();
    let mut misc_buffer = String::new();
    let mut lexer_mode = LexerMode::Normal;
    let mut last_mode = LexerMode::Normal;

    for c in cmd.chars() {
        match lexer_mode {
            LexerMode::Normal => {
                 match c {
                    ' ' => {
                        flush_buf(&mut tokens, &mut word_buffer, false); 
                        flush_buf(&mut tokens, &mut misc_buffer, true); 
                    },
                    '\n' => {
                        flush_buf(&mut tokens, &mut word_buffer, false); 
                        flush_buf(&mut tokens, &mut misc_buffer, true); 
                        //when duplicate newlines fuck with your parser, you just have to get a li'l crafty
                        if tokens.last() != Some(&LexToken::Newline) {
                            tokens.push(LexToken::Newline);
                        }
                    },
                    ';' => {
                        flush_buf(&mut tokens, &mut word_buffer, false); 
                        flush_buf(&mut tokens, &mut misc_buffer, true);
                        tokens.push(LexToken::Semicolon);
                    },
                    '=' => {
                        flush_buf(&mut tokens, &mut word_buffer, false); 
                        flush_buf(&mut tokens, &mut misc_buffer, true);
                        tokens.push(LexToken::Eq);
                    },
                    '#' => {
                        if word_buffer.len() == 0 && misc_buffer.len() == 0 {
                            lexer_mode = LexerMode::Comment;
                        } else {
                            word_buffer.push(c);
                        }
                    },
                    '\'' =>{
                        lexer_mode = LexerMode::Qoute;
                    },
                    '"' =>{
                        lexer_mode = LexerMode::DQoute;
                    },
                    '\\' => {
                        lexer_mode = LexerMode::Escape;
                        last_mode = LexerMode::Normal;
                    },
                    '&' => {
                        misc_buffer.push(c);
                    },
                    '|' => {
                        misc_buffer.push(c);
                    },
                    _ => {
                        word_buffer.push(c);
                    }
                }       
            }, 
            LexerMode::Comment => {
                match c {
                    '\n' => {
                        lexer_mode = LexerMode::Normal;
                        tokens.push(LexToken::Newline);
                    },
                    
                    _ => ()
                }
            },
            LexerMode::Qoute => {
                match c {
                    '\'' =>{
                        lexer_mode = LexerMode::Normal;
                    },
                    '\\' => {
                        lexer_mode = LexerMode::Escape;
                        last_mode = LexerMode::Qoute;
                    },
                    _ => {
                       word_buffer.push(c);
                    }
                }
            },
            LexerMode::DQoute => {
                match c {
                    '"' =>{
                        lexer_mode = LexerMode::Normal;
                    },
                    '\\' => {
                        lexer_mode = LexerMode::Escape;
                        last_mode = LexerMode::DQoute;
                    },
                    _ => {
                        word_buffer.push(c);
                    }
                }
            },
            LexerMode::Escape => {
                if last_mode == LexerMode::Normal ||
                        (last_mode == LexerMode::Qoute && c == '\'') ||
                        (last_mode == LexerMode::DQoute && c == '"') {
                    word_buffer.push(c);
                } else {
                    match c {
                        '\\' => word_buffer.push('\\'),
                        'a' => word_buffer.push('\x07'),
                        'b' => word_buffer.push('\x08'),
                        'e' => word_buffer.push('\x1B'),
                        'f' => word_buffer.push('\x0C'),
                        'n' => word_buffer.push('\n'),
                        'r' => word_buffer.push('\r'),
                        't' => word_buffer.push('\t'),
                        'v' => word_buffer.push('\x0B'),
                        _ => {
                            word_buffer.push('\\');
                            word_buffer.push(c)
                        }, 
                    }        
                }
                
                lexer_mode = last_mode;
            }
        }
    }

    if word_buffer.len() > 0 || misc_buffer.len() > 0 {
        flush_buf(&mut tokens, &mut word_buffer, false); 
        flush_buf(&mut tokens, &mut misc_buffer, true);
    }

    return tokens;
}

struct Peeker {
    tokens: Vec<LexToken>
}

impl Peeker {
    fn new(tokens: Vec<LexToken>) -> Self {
        Self { tokens }
    }

    fn peek(&self,i: usize) -> LexToken {
        if i >= self.tokens.len() {
            return LexToken::EOF;
        }
        self.tokens[i].clone()
    }
    
    fn consume(&mut self) -> LexToken {
        if 1 > self.tokens.len() {
            return LexToken::EOF;
        }
        self.tokens.remove(0)
    }

}

fn parse_expr(peeker: &mut Peeker) -> AstNode{
    if let LexToken::Word(w) = peeker.peek(0) {
        peeker.consume(); // only consume it if it matches 
        AstNode::ConstantString(w) 
    } else {
        AstNode::ParseEnd 
    }
}

fn parse_command (peeker: &mut Peeker) -> AstNode {
    let command = parse_expr(peeker);
    
    if command == AstNode::ParseEnd {
        return AstNode::ParseEnd;
    }
    
    if peeker.peek(0) == LexToken::Eq{
        peeker.consume();
        let val = parse_expr(peeker);

        return AstNode::Sequence(vec![
            AstNode::EnvVarSet(Box::new(command), Box::new(val)),
            parse_command(peeker)
        ])
    }
    
    let mut args: Vec<AstNode> = vec![];

    loop {
        let expr = parse_expr(peeker);

        if expr == AstNode::ParseEnd {
            break;
        }

        args.push(expr);
    };

    AstNode::Command(Box::new(command), args)
}

fn parse_command_expr (peeker: &mut Peeker) -> AstNode{
    let mut expr = AstNode::ParseEnd;
    loop {
        let next = peeker.peek(0);
        if next == LexToken::And {
            peeker.consume();
            
            //since ands are splittable, we have to get rid of newlines here
            if peeker.peek(0) == LexToken::Newline { peeker.consume(); }
            
            let command = parse_command(peeker);
            expr = AstNode::And(Box::new(expr), Box::new(command)); 
        } else if next == LexToken::Or {
            peeker.consume();
           
            //since ands are splittable, we have to get rid of newlines here
            if peeker.peek(0) == LexToken::Newline { peeker.consume(); }
            
            let command = parse_command(peeker);
            expr = AstNode::Or(Box::new(expr), Box::new(command)); 
        } else if next == LexToken::Semicolon || next == LexToken::EOF {
            peeker.consume();
            return expr;
        } else if next == LexToken::Newline {
            peeker.consume();
        } else {
            if expr != AstNode::ParseEnd{
                return expr;
            } 

            let command = parse_command(peeker);  
            if command == AstNode::ParseEnd {
                return expr;
            } else {
                expr = command;
            }
        }
    }    
}

fn parse_sequence(mut peeker: Peeker) -> AstNode{
    let mut nodes = Vec::<AstNode>::new();
   
    loop {
        let cmdexpr = parse_command_expr(&mut peeker);

        if cmdexpr == AstNode::ParseEnd {
            break;
        } else {
            nodes.push(cmdexpr);
            if peeker.peek(0) == LexToken::EOF {
                break;
            }
        }
    }

    return AstNode::Sequence(nodes);
}

pub fn parse<'a>(cmd: &'a str) -> AstNode {
    let lexed = lex(cmd);
    
    #[cfg(feature = "debug_parser")]
    println!("{lexed:?}");
    
    let peeker = Peeker::new(lexed);

    return parse_sequence(peeker);
}
