//! Smoke tests for the parts of milktea that work end-to-end today.
//! These will start failing as soon as the lexer/parser are ported in;
//! that's expected — they're written against the target behavior, not
//! the current stubs.

#[test]
#[ignore = "lexer not yet ported"]
fn identity_function() {
    // λ> let id = \x:Int. x in id 42
    // expected: 42 : Int
}

#[test]
#[ignore = "lexer not yet ported"]
fn type_error_in_if_branches() {
    // λ> \x:Bool. if x then 1 else "two"
    // expected: type error
}
