# Rebourne intented behaviour
> **NOTE: Since Rebourne is meant to be BASH compatible, any incompatibility in this document is by definition a bug**
The purpuse of this document is to indicate the intented behaviour of the shell to make differentiating design and implementation problems easier.

## REPL mode
If invoked without an argument, Rebourne goes to REPL mode. As the name implies, REPL mode is a Read Evaluate Print Loop.

In REPL mode, the following events are repeated:
1. The prompt(env variable `PS1`) gets printed
2. The shell listens for user input
3. The entered string gets interpreted as a command

### prompt escape sequences
The prompt has some extra escape sequences:
* `w` -> the current working directory
* `W` -> the basename of the current working directory
* `v` -> the version of the shell ("maj.min")
* `V` -> the version of the shell ("maj.min.patch")

## Script execution
If invoked with the first argument being a path to a file, the whole contents of the file are executed as if it was a single command.

## Command language
A command is parsed into a hierarchical tree of syntax(AST):
> root sequence > command expressions > commands > expressions; sequences

Command expressions can be separated with semicolons(`;`).

### && and ||
AND(&&) and OR(||) are command expressions that conditionally execute the command after them based on the exit code of the one before them:
* AND executes the second commmand if the first one **succeeded**,
* OR executes the second command if the first one **failed**

### Escaping
If any character (outside of Qoutes and DQoutes) is preceeded by a backslash(`\`), it gets into the word as-is, not as an operator.

### Qoutes and DQoutes
All characters are treated literally between qoutes(`'`) and double qoutes(`"`) do the same except with the option for escape with backslash(`\`).

#### Qouted escaping
Inside qoutes, escape sequences(backslash + single character) turn into an untypable character:
* `a` -> ASCII `BEL` character
* `b` -> ASCII `BS`  character
* `e` -> `0x1B`
* `f` -> ASCII `FF`  character
* `n` -> ASCII `LF`  character
* `r` -> ASCII `CR`  character
* `t` -> ASCII `TAB` character
* `v` -> ASCII `VT`  character

The matching qoute(`'` if in Qoute mode, `"` if in DQoute mode) character will print it literally, and any other combination will print both the backslash and the character.
