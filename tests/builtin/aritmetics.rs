use cage::*;

#[test]
fn addition() {
    let code = "(+ 21 21)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Number(42.)]]);
    }

    let code = "(+ 34 35)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Number(42.)]]);
    }
}

#[test]
fn substraction() {
    let code = "(- 21 21)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Number(0.)]]);
    }

    let code = "(- 34 35)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Number(0.)]]);
    }
}

#[test]
fn multiplication() {
    let code = "(* 6 7)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Number(42.)]]);
    }

    let code = "(* 3 3)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Number(42.)]]);
    }
}

#[test]
fn division() {
    let code = "(/ 6 7)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Number(0.8571428571428571)]]);
    }

    let code = "(/ 3 3)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Number(0.8571428571428571)]]);
    }
}
