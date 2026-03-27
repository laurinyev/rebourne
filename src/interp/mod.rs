use {
    crate::{parser::*, resolver::resolve}, std::{
        env::*,
        process::*
    }
};

fn interp_command(cmd: &AstNode, args: &Vec<AstNode>, backgrouned: bool, is_nested: bool) -> (i32, String) {
    let cmd_resolved = interp_ast(cmd, true).1;

    match cmd_resolved.as_str() {
        "cd" => {
            let mut dir = match args.get(0) {
                Some(v) => interp_ast(&v,true).1,
                None => "".to_string() 
            };
            if is_nested {
                return (0,"".to_string());
            }
            if dir == "" {
                 match var("HOME") {
                    Ok(v) => {
                        dir = v;
                    },
                    Err(..) => {
                        eprintln!("cd: HOME isn't set"); 
                        return (1,"".to_string());
                    }
                }
            }
            match set_current_dir(dir) {
                Ok(..) => unsafe {
                    set_var("PWD",current_dir().unwrap());
                },
                Err(e) => {
                    eprintln!("Couldn't change directory: {e}");
                    return (1,"".to_string());
                }
            };
            (0,"".to_string())
        },
        "exit" => {
            if !is_nested {
                exit(0)
            } else {
                (0,"".to_string())
            }
        },
        _ => {
            let util_path = resolve(&cmd_resolved); 
            if util_path.is_none() {
                eprintln!("command not found: {cmd_resolved}");
                return (127,"".to_string()); 
            }
            let mut child = Command::new(util_path.unwrap());

            child.args(args.iter().map(|a| interp_ast(a, true).1));
            
            if !is_nested && !cfg!(test) {
                child.stdin(Stdio::inherit());
                child.stdout(Stdio::inherit());
                child.stderr(Stdio::inherit());
            }

            if backgrouned {
                return match child.spawn() {
                    Ok(_) => (0,"".to_string()),
                    Err(_) => (1,"".to_string())
                };
            } else {
                let out = child.output(); 
                if is_nested || cfg!(test) {
                    match out {
                        Ok(o) => match String::from_utf8(o.stdout) {
                                Ok(s) => (o.status.code().unwrap_or(1),s),
                                Err(_) => (1,"".to_string()),
                        },
                        Err(_) => (1,"".to_string()), 
                    }
                } else {
                    match out {
                        Ok(o) => (o.status.code().unwrap_or(1), "".to_string()),
                        Err(_) => (1, "".to_string()),
                    }
                }
            }
        }
    }
}

fn interp_ast<'a>(node: &AstNode, is_nested: bool) -> (i32, String){
    match node {
        AstNode::Sequence(nodes) => {
            let mut exitcode = 0;
            let mut stdout = String::new();
            for n in nodes {
                let returned = interp_ast(n, is_nested);
                exitcode = returned.0;
                stdout  += &returned.1;
            }
            (exitcode, stdout)  
        },
        AstNode::ParseEnd => (0,"".to_string()),
        AstNode::ConstantString(v) => (0,v.to_string()),
        AstNode::And(a,b) => {
            let first = interp_ast(a, false);
            if first.0 == 0 {
                return interp_ast(b, false);
            } else {
                return first;
            }
        },
        AstNode::Or(a,b) => {
            let first = interp_ast(a, false);
            if first.0 != 0 {
                return interp_ast(b, false);
            } else {
                return first;
            }
        },
        AstNode::Command(cmd,args, backgrouned) => interp_command(cmd, args, *backgrouned, is_nested), 
        AstNode::EnvVarSet(var, val) => {
            let var_name = interp_ast(var, true).1;
            let val_val = interp_ast(val, true).1;
       
            unsafe {
                set_var(var_name, val_val);
            }

            (1, "".to_string())
        }
    }
}

pub fn run(cmd: &str) -> (i32, String) {
    let root = parse(cmd);
    
    return interp_ast(&root,false);
}
