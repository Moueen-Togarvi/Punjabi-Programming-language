# Punjabi Programming Language

Eh Rust vich bani simple Roman Punjabi programming language ae.

Goal: tusi code likho, run karo, te sath sath vekho har file da kaam ki ae.

## Install

Rust chahida ae:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run

```bash
cargo run -- run examples/hello.pun
cargo run -- run examples/variables.pun
cargo run -- run examples/if_else.pun
cargo run -- run examples/loop.pun
```

Debug layi:

```bash
cargo run -- --tokens examples/hello.pun
cargo run -- --ast examples/if_else.pun
cargo run -- repl
```

Tests:

```bash
cargo test
```

## Easy Syntax

```pun
likho "salam"

rakho x = 5
x = x + 1

je x > 3 {
  likho "wadda"
} nai_ta {
  likho "chota"
}

jadd_tak x < 10 {
  x = x + 1
}
```

## Keywords

- `likho`: screen te print karo
- `rakho`: variable banao
- `je`: if
- `nai_ta`: else
- `jadd_tak`: while loop
- `sach`: true
- `jhooth`: false
- `te`: and
- `ya`: or
- `nai`: not

## Project Simple Map

- `src/main.rs`: command samjhda ae, file run karda ae
- `src/token.rs`: code de chhotay parts de names
- `src/lexer.rs`: text nu tokens vich torrda ae
- `src/ast.rs`: program di tree shape
- `src/parser.rs`: tokens nu AST bananda ae
- `src/value.rs`: number, string, bool values
- `src/interpreter.rs`: program chalanda ae
- `src/error.rs`: easy errors dinda ae

Simple flow:

```text
code.pun -> lexer -> parser -> interpreter -> output
```

Example:

```pun
likho "salam"
```

1. Lexer kehnda: eh `likho` token ae, eh string token ae.
2. Parser kehnda: eh print statement ae.
3. Interpreter kehnda: string screen te dikhao.
