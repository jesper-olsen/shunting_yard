//#[derive(Clone, Copy, PartialEq)]
#[derive(PartialEq, Debug)]
enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exp,
}
impl OperatorType {
    fn precedence(&self) -> u8 {
        use OperatorType::*;
        match self {
            Plus | Minus => 2,
            Multiply | Divide => 3,
            Exp => 4,
        }
    }

    fn is_left_associative(&self) -> bool {
        use OperatorType::*;
        match self {
            Plus | Minus | Multiply | Divide => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Function(String),
    Operator(OperatorType),
    Comma,
    LeftParen,
    RightParen,
}

use OperatorType::*;
use Token::*;

fn parse_tokens(tokens: Vec<Token>) -> Result<f64, String> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    for token in tokens {
        println!("Input Token: {token:?}, ==>  {operator_stack:?}");
        match token {
            Number(_) => output_queue.push(token),
            LeftParen | Function(_) => operator_stack.push(token),
            RightParen => {
                while let Some(optoken) = operator_stack.pop() {
                    if optoken == LeftParen {
                        operator_stack.push(optoken);
                        break;
                    }
                    output_queue.push(optoken);
                }
                let Some(LeftParen) = operator_stack.pop() else {
                    return Err("mismatched parentheses".to_string());
                };
            }
            Operator(o1) => {
                while let Some(optoken) = operator_stack.pop() {
                    let Operator(o2) = optoken else {
                        operator_stack.push(optoken);
                        break;
                    };
                    if o2.precedence() > o1.precedence()
                        || o2.precedence() == o1.precedence() && o1.is_left_associative()
                    {
                        output_queue.push(Operator(o2));
                    } else {
                        operator_stack.push(Operator(o2));
                        break;
                    }
                }
                operator_stack.push(Operator(o1))
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

    println!("Operator Stack: {operator_stack:?}");
    while let Some(token) = operator_stack.pop() {
        match token {
            Token::LeftParen => return Err("mismatched parentheses".to_string()),
            _ => output_queue.push(token),
        }
    }

    println!("output queue: {output_queue:?}");
    // evaluate RPN expression
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
                    break;
                };
                match o {
                    Plus => stack.push(Number(a + b)),
                    Minus => stack.push(Number(b - a)),
                    Multiply => stack.push(Number(a * b)),
                    Divide => stack.push(Number(b / a)),
                    Exp => stack.push(Number(b.powf(a))),
                    _ => (),
                }
            }
            Function(s) if s == "max" => {
                let Some(Number(a)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                };
                let Some(Number(b)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                    break;
                };
                stack.push(Number(if a > b { a } else { b }));
            }
            Function(s) if s == "min" => {
                let Some(Number(a)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                };
                let Some(Number(b)) = stack.pop() else {
                    return Err("expected a number on the stack".to_string());
                    break;
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
            _ => todo!(),
        }
    }

    if let [Number(a)] = stack[..] {
        return Ok(a);
    }
    Err("bad expression".to_string())
}

fn main() {
    // 3+4
    let tokens = vec![Number(3.0), Operator(Plus), Number(4.0)];
    // 3-4
    let tokens = vec![Number(3.0), Operator(Minus), Number(4.0)];
    let tokens = vec![
        Number(3.0),
        Operator(Minus),
        Function("cos".to_string()),
        Number(4.0),
    ];
    //let tokens = vec![ Function("max".to_string()), LeftParen, Number(3.0), Comma, Number(4.0), RightParen, ];
    //let tokens = vec![ LeftParen, Number(3.0), Operator(Plus), Number(4.0), RightParen, ];
    // 3+4*2
    //let tokens = [ Number(3.0), Operator(Plus), Number(4.0), Operator(Multiply), Number(2.0)];
    // 3 + 4 × 2 ÷ ( 1 − 5 ) ^ 2 ^ 3
    //let tokens = [ Number(3.0), Operator(Plus), Number(4.0), Operator(Multiply), Number(2.0), Operator(Divide), Operator(LeftParen), Number(1.0), Operator(Minus), Number(5.0), Operator(RightParen), Operator(Exp), Number(2.0), Operator(Exp), Number(3.0)];
    // sin ( max ( 2, 3 ) ÷ 3 × π )
    let tokens = vec![
        Function("sin".to_string()),
        LeftParen,
        Function("max".to_string()),
        LeftParen,
        Number(2.0),
        Comma,
        Number(3.0),
        RightParen,
        Operator(Divide),
        Number(3.0),
        Operator(Multiply),
        Number(3.14),
        RightParen,
    ];
    match parse_tokens(tokens) {
        Ok(a) => println!("Result: {a}"),
        Err(m) => println!("Error: {m}"),
    };
}

// while there are tokens on the operator stack:
//     /* If the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses. */
//     {assert the operator on top of the stack is not a (left) parenthesis}
//     pop the operator from the operator stack onto the output queue

/* The functions referred to in this algorithm are simple single argument functions such as sine, inverse or factorial. */
/* This implementation does not implement composite functions, functions with a variable number of arguments, or unary operators. */

// while there are tokens to be read:
//     read a token
//     if the token is:
//     - a number:
//         put it into the output queue
//     - a function:
//         push it onto the operator stack
//     - an operator o1:
//         while (
//             there is an operator o2 at the top of the operator stack which is not a left parenthesis,
//             and (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1 is left-associative))
//         ):
//             pop o2 from the operator stack into the output queue
//         push o1 onto the operator stack
//     - a ",":
//         while the operator at the top of the operator stack is not a left parenthesis:
//              pop the operator from the operator stack into the output queue
//     - a left parenthesis (i.e. "("):
//         push it onto the operator stack
//     - a right parenthesis (i.e. ")"):
//         while the operator at the top of the operator stack is not a left parenthesis:
//             {assert the operator stack is not empty}
//             /* If the stack runs out without finding a left parenthesis, then there are mismatched parentheses. */
//             pop the operator from the operator stack into the output queue
//         {assert there is a left parenthesis at the top of the operator stack}
//         pop the left parenthesis from the operator stack and discard it
//         if there is a function token at the top of the operator stack, then:
//             pop the function from the operator stack into the output queue
// /* After the while loop, pop the remaining items from the operator stack into the output queue. */
// while there are tokens on the operator stack:
//     /* If the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses. */
//     {assert the operator on top of the stack is not a (left) parenthesis}
//     pop the operator from the operator stack onto the output queue
