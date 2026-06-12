# Learning Plan - Simple Punjabi

Eh plan chhota rakheya ae. Har step vich pehla role, phir reason, phir practice.

## 1. Pehla Run

Command:

```bash
cargo run -- run examples/hello.pun
```

Eh file run karega:

```pun
likho "sat sri akal"
likho "Punjabi language chal rehi ae"
```

Samjho:

- `likho` da matlab print.
- Quote `"..."` de andar text hunda ae.
- Output terminal te aunda ae.

## 2. Project Da Flow

```text
.pun file -> lexer -> parser -> interpreter -> output
```

Easy meaning:

- Lexer: text nu chhotay parts vich torrda ae.
- Parser: parts nu program di shape bananda ae.
- Interpreter: program nu chalanda ae.

## 3. Files Da Role

### `src/main.rs`

Role: terminal wali command handle karda ae.

Example:

```bash
cargo run -- run examples/hello.pun
```

Eh `main.rs` nu kehnda ae: file kholo te run karo.

### `src/token.rs`

Role: token names rakhta ae.

Example:

```pun
rakho x = 5
```

Tokens:

- `rakho`
- `x`
- `=`
- `5`

### `src/lexer.rs`

Role: raw text nu tokens bananda ae.

Debug:

```bash
cargo run -- --tokens examples/hello.pun
```

Eh dikhavega ke code kinay chhotay parts vich divide hoya.

### `src/ast.rs`

Role: program di shape define karda ae.

Example:

```pun
likho x
```

AST meaning: print statement, jide andar variable `x` ae.

### `src/parser.rs`

Role: tokens nu AST bananda ae.

Debug:

```bash
cargo run -- --ast examples/if_else.pun
```

Common error:

```pun
rakho x 5
```

Eh galat ae, kyonke `=` missing ae.

### `src/value.rs`

Role: runtime values rakhta ae.

Types:

- number: `5`
- string: `"salam"`
- bool: `sach`, `jhooth`
- null: `khali`

### `src/interpreter.rs`

Role: actual program chalanda ae.

Eh kaam karda ae:

- variable save
- math
- print
- if/else
- loop

### `src/error.rs`

Role: easy error message dinda ae.

Example:

```pun
likho x
```

Agar `x` pehlan `rakho x = ...` naal nai baneya, error aayega.

## 4. Syntax Practice

### Variable

```pun
rakho naam = "Ali"
likho naam
```

### Math

```pun
rakho x = 5
rakho y = 2
likho x + y
```

### If Else

```pun
rakho score = 80

je score >= 50 {
  likho "pass"
} nai_ta {
  likho "fail"
}
```

### Loop

```pun
rakho count = 1

jadd_tak count <= 5 {
  likho count
  count = count + 1
}
```

## 5. Learning Order

1. Pehle examples run karo.
2. Phir `--tokens` command dekho.
3. Phir `--ast` command dekho.
4. Phir `src/main.rs` read karo.
5. Phir `lexer.rs`, `parser.rs`, `interpreter.rs` read karo.

## 6. Mini Tasks

- `examples/my_name.pun` banao, apna naam print karo.
- `examples/add.pun` banao, 2 numbers add karo.
- `examples/pass_fail.pun` banao, score check karo.
- `examples/count.pun` banao, 1 to 10 loop chalao.

## 7. Short Rule

Jadon confusion hove, eh yaad rakho:

```text
Lexer samjhda ae: words kehry ne?
Parser samjhda ae: sentence sahi ae?
Interpreter samjhda ae: hun chalana ki ae?
```
