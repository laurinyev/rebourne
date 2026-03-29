use super::*;

pub struct Peeker {
    tokens: Vec<LexToken>,
    pos: usize
}

impl Peeker {
    pub fn new(tokens: Vec<LexToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn peek(&self,i: usize) -> LexToken {
        self.tokens.get(self.pos + i).cloned().unwrap_or(LexToken::EOF) 
    }
    
    pub fn consume(&mut self) -> LexToken {
        let tok = self.peek(0);
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }
}
