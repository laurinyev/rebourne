use super::*;

pub struct Peeker {
    tokens: Vec<LexToken>
}

impl Peeker {
    pub fn new(tokens: Vec<LexToken>) -> Self {
        Self { tokens }
    }

    pub fn peek(&self,i: usize) -> LexToken {
        if i >= self.tokens.len() {
            return LexToken::EOF;
        }
        self.tokens[i].clone()
    }
    
    pub fn consume(&mut self) -> LexToken {
        if 1 > self.tokens.len() {
            return LexToken::EOF;
        }
        self.tokens.remove(0)
    }

}
