use std::io::{self, Write};

mod scanner;
use scanner::{OperatorType::*, Token, Token::*};

fn repl() {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        line.clear();
        if io::stdin().read_line(&mut line).is_err() {
            println!();
            break;
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        let mut scanner = scanner::Scanner::new(input);
        let infix_tokens = scanner.scan_tokens().unwrap();
        println!("Infix input:{infix_tokens:?}");
        match parse_tokens(infix_tokens) {
            Ok(postfix_tokens) => {
                println!("Postfix    :{postfix_tokens:?}");
                match eval_rpn(postfix_tokens) {
                    Ok(n) => println!("Result: {n}"),
                    Err(m) => println!("Error: {m}"),
                }
            }
            Err(m) => println!("Error: {m}"),
        };
    }
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    for token in tokens {
        match token {
            Number(_) => output_queue.push(token),
            LeftParen | Function(_) => operator_stack.push(token),
            RightParen => loop {
                let Some(optoken) = operator_stack.pop() else {
                    return Err("mismatched parentheses".to_string());
                };
                if optoken == LeftParen {
                    break;
                }
                output_queue.push(optoken);
            },
            Operator(o1) => {
                while let Some(Operator(o2)) = operator_stack.last() {
                    if o2.precedence() > o1.precedence()
                        || o2.precedence() == o1.precedence() && o1.is_left_associative()
                    {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(Operator(o1));
            }

            Comma => {
                while let Some(token) = operator_stack.pop() {
                    if token == LeftParen {
                        operator_stack.push(token);
                        break;
                    }
                    output_queue.push(token);
                }
            }
        }
    }

    while let Some(token) = operator_stack.pop() {
        match token {
            Token::LeftParen => return Err("mismatched parentheses".to_string()),
            _ => output_queue.push(token),
        }
    }

    Ok(output_queue)
}

fn eval_rpn(output_queue: Vec<Token>) -> Result<f64, String> {
    // evaluate expression in Reverse Polish Notation
    let mut stack = Vec::new();
    for token in output_queue {
        match token {
            Number(_) => stack.push(token),
            Operator(o) => {
                let Some(Number(a)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                };
                let Some(Number(b)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                };
                match o {
                    Plus => stack.push(Number(a + b)),
                    Minus => stack.push(Number(b - a)),
                    Multiply => stack.push(Number(a * b)),
                    Divide => stack.push(Number(b / a)),
                    Exp => stack.push(Number(b.powf(a))),
                }
            }
            Function(s) if s == "max" => {
                let Some(Number(a)) = stack.pop() else {
                    return Err("bad function call - expected a number on the stack".to_string());
                };
                let Some(Number(b)) = stack.pop() else {
                    return Err("bad function call - expected a number on the stack".to_string());
                };
                stack.push(Number(if a > b { a } else { b }));
            }
            Function(s) if s == "min" => {
                let Some(Number(a)) = stack.pop() else {
                    return Err("bad function call - expected a number on the stack".to_string());
                };
                let Some(Number(b)) = stack.pop() else {
                    return Err("bad function call - expected a number on the stack".to_string());
                };
                stack.push(Number(if a < b { a } else { b }));
            }
            Function(s) if s == "cos" => {
                let Some(Number(a)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                };
                stack.push(Number(a.cos()))
            }
            Function(s) if s == "sin" => {
                let Some(Number(a)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                };
                stack.push(Number(a.sin()))
            }
            _ => return Err(format!("Unknown function: {token:?}")),
        }
    }

    if let [Number(a)] = stack[..] {
        return Ok(a);
    }
    Err("bad expression".to_string())
}

fn main() {
    repl();
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn eval(s: &str, expected: f64) {
        let mut scanner = scanner::Scanner::new(s);
        let infix_tokens = scanner.scan_tokens().unwrap();
        let pfix_tokens = parse_tokens(infix_tokens).unwrap();
        let n = eval_rpn(pfix_tokens).unwrap();
        let epsilon = 1e-9;
        assert!(
            (n - expected).abs() < epsilon,
            "Expected {expected}, got {n}"
        );
    }

    #[test]
    fn test_eval() {
        for (s, expected) in [
            ("3-4", -1.0),
            ("3+4*2", 11.0),
            ("sin ( max ( 2, 3 ) / 3 * 3.14 )", 0.0015926529164868282),
            ("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3", 3.0001220703125),
        ] {
            eval(s, expected);
        }
    }
}
