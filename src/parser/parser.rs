use super::*;
use super::peeker::Peeker;

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
    let mut backgrounded = false;

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
        if peeker.peek(0) == LexToken::Bg {
            backgrounded = true;
            break;
        }

        let expr = parse_expr(peeker);

        if expr == AstNode::ParseEnd {
            break;
        }

        args.push(expr);
    };

    AstNode::Command(Box::new(command), args, backgrounded)
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

pub fn parse_sequence(mut peeker: Peeker) -> AstNode{
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
