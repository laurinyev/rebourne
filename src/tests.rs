use super::{
    interp::*,
};

#[test]
fn basic_echo() {
    assert_eq!(run("echo \"Werx\""), (0, "Werx\n".to_string()));
}

#[test]
fn semi_echo() {
    assert_eq!(run("echo \"Werx\"; echo \"Other part too!\""), (0, "Werx\nOther part too!\n".to_string()));
}

#[test]
fn multiline_echo() {
    assert_eq!(run("echo \"Werx\"\necho \"Other line too!\""), (0, "Werx\nOther line too!\n".to_string()));
}

#[test]
fn looong_semi() {
    assert_eq!(run("echo \"a\"; echo \"s\"; echo \"d\"; echo \"f\"; echo \"g\"; echo \"h\""), (0, "a\ns\nd\nf\ng\nh\n".to_string()));
}

#[test]
fn true_and() {
    assert_eq!(run("true && echo \"H\""), (0, "H\n".to_string()));
}

#[test]
fn false_and() {
    assert_eq!(run("false && echo \"H\""), (1, "".to_string()));
}

#[test]
fn true_or() {
    assert_eq!(run("true || echo \"H\""), (0, "".to_string()));
}

#[test]
fn false_or() {
    assert_eq!(run("false || echo \"H\""), (0, "H\n".to_string()));
}

#[test]
fn true_and_split1() {
    assert_eq!(run("true \n && echo \"H\""), (0, "H\n".to_string()));
}

#[test]
fn false_and_split1() {
    assert_eq!(run("false \n && echo \"H\""), (1, "".to_string()));
}

#[test]
fn true_or_split1() {
    assert_eq!(run("true \n || echo \"H\""), (0, "".to_string()));
}

#[test]
fn false_or_split1() {
    assert_eq!(run("false \n || echo \"H\""), (0, "H\n".to_string()));
}

#[test]
fn true_and_split2() {
    assert_eq!(run("true && \n echo \"H\""), (0, "H\n".to_string()));
}

#[test]
fn false_and_split2() {
    assert_eq!(run("false && \n echo \"H\""), (1, "".to_string()));
}

#[test]
fn true_or_split2() {
    assert_eq!(run("true || \n echo \"H\""), (0, "".to_string()));
}

#[test]
fn false_or_split2() {
    assert_eq!(run("false || \n echo \"H\""), (0, "H\n".to_string()));
}

#[test]
fn escape1() {
    assert_eq!(run("echo hello \\\nworld"), (0, "hello \nworld\n".to_string()));
}

#[test]
fn escape2() {
    assert_eq!(run("echo cat \\&\\& meow"), (0, "cat && meow\n".to_string()));
}

#[test]
fn qoute_escape() {
    assert_eq!(run("echo '\\a\\b\\e\\f\\n\\r\\t\\v\\w'"), (0, "\\a\\b\\e\\f\\n\\r\\t\\v\\w\n".to_string()));
}

#[test]
fn dqoute_escape() {
    assert_eq!(run("echo \"\\a\\b\\e\\f\\n\\r\\t\\v\\\"\\w\""), (0, "\x07\x08\x1B\x0C\n\r\t\x0B\"\\w\n".to_string()));
}



