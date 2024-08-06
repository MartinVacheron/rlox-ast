# Rizon
 
<p align=center>
    <img src="icon.png" alt="drawing" width="200" align="center"/>
</p>

## Interpreted language for fast iteration and type safety
>[!IMPORTANT]
>Rizon is still in development, use it for personal purpose only.
>
>It is entirely made by my self in my free time. If you want to give it a try and report any bug fell free to do so.

Rizon is a language that aims to be handful for quick prototyping thanks to type inference and cohercion and light syntax (minimum use of '()' and no ';') while being type safe, preventing al lot of runetime crash.

It's inspired by Rust's type system as well as Zig's error handling with some other features I found neat as:
- type union
- nullable type
- ...

Right know at runtime the interpreter walks an AST generated by a recursive descent parser. This is not optimal as it is slow, in the future it will be implemented as a bytecode compiler running on its own Rust VM.

## CLI usage
You can use ```rizon.exe``` command alone to enter REPL mode.

You can pass additional arguments:
| Shorthand | Full             | Description                                                               | Default |
|-----------|------------------|---------------------------------------------------------------------------|---------|
| -f        | --file           | path to the file to run                                                   | ""      |
| -i        | --inter          | enters REPL mode after executing file                                     | false   |
|           | --print-tokens   | prints the output of the lexer                                            | false   |
| -s        | --static-analyse | only runs the static analysis (lexer, parser, static analyzer)            | false   |
| -h        | --help           | shows help message and exits                                              | false   |
| -v        | --version        | prints version information and exits                                      | false   |

## Statically typed


## Tools
You can use the official VSCode plugin to work with the language to have basic language support. You can find the **vsix** file in the official repo: [rizon-vscode-tools](https://github.com/MartinVacheron/rizon-vscode-tools).

## Road map
For version v0.2, the following features are gonna be added:

- [ ] ```break``` statement in for and while loops
- [ ] ```else if``` branch
- [ ] Scientific notation for ```int``` and ```float``` (1e-4)
- [ ] Ternary operator: ```var a = foo == bar1 ? 1 : 0```
- [ ] Compound assign: ```a += 1```
- [ ] Coma declaration for variables: ```var a, b, c: int```
