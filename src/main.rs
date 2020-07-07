mod interpreter;

fn main() {
    let code = r#"
+++++[>+>++<<-]>>+++ sadly 14 chars to get to 13 so a good example for caution in multiplication
                     might be redeemed by the 5 13 arrangement it really is
[>+++++>+++<<-]<     5 0 65 39 pointing at 5
[>+>.<<-]>>>-------. makes a 5 65 32 pattern while printing 'AAAAA ' (pointing at 32)
<<[>.<-]             prints the other 5 A's
"#;
    for output in interpreter::InterpreterRun::new(code.to_string(), || 0) {
        if let Some(ch) = output {
	    print!("{}", ch as u8 as char);
	}
    }
}
