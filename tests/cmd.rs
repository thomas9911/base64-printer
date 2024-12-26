use assert_cmd::Command;

#[test]
fn single_line() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .write_stdin("config: dGhpcyBpcyBhIHRlc3Q=")
        .assert()
        .success();

    assert.stdout("config: this is a test\n");
}

#[test]
fn multi_line() {
    let text = r#"
stuff:
  secret: aGFsbG8K
  configMap:
    key1: value1
    key2: this is a very good secret

  something:
    else: true
    "#;
    let out = r#"
stuff:
  secret: hallo

  configMap:
    key1: value1
    key2: this is a very good secret

  something:
    else: true
    
"#;
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.write_stdin(text).assert().success();

    assert.stdout(out);
}
