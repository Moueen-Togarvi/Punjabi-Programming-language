use punjabi_lang::{run_source, tokenize};

#[test]
fn hello_program_prints_text() {
    let output = run_source(r#"likho "sat sri akal""#).unwrap();
    assert_eq!(output, vec!["sat sri akal".to_string()]);
}

#[test]
fn token_debug_mode_has_eof() {
    let tokens = tokenize("likho 1").unwrap();
    assert!(format!("{tokens:?}").contains("Eof"));
}

#[test]
fn undefined_variable_returns_friendly_error() {
    let error = run_source("likho x").unwrap_err().to_string();
    assert!(error.contains("Variable 'x' nai mili"));
}

#[test]
fn pakistani_punjabi_aliases_work() {
    let source = r#"
rakho x = 0
jadd_tak x < 2 {
  x = x + 1
}
je x == 2 {
  likho nai jhooth
} nai_ta {
  likho "fail"
}
"#;

    let output = run_source(source).unwrap();
    assert_eq!(output, vec!["sach".to_string()]);
}
