use super::*;

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

pub fn lex<'a>(cmd: &'a str) -> Vec<LexToken> {
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
