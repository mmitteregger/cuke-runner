use super::expression::*;

const ESCAPING_CHAR: char = '\\';

pub fn parse(infix: &str) -> Result<Expression, String> {
    let tokens = tokenize(infix);

    if tokens.is_empty() {
        return Ok(Expression::True);
    }

    let mut operators = Vec::new();
    let mut expressions = Vec::new();
    let mut expected_token_type = TokenType::Operand;

    for token in tokens {
        if is_unary(&token) {
            check(expected_token_type, TokenType::Operand)?;
            operators.push(token);
            expected_token_type = TokenType::Operand;
        } else if is_binary(&token) {
            check(expected_token_type, TokenType::Operator)?;
            while !operators.is_empty() && is_operator(operators.last().unwrap()) && (
                (get_assoc(&token) == Some(Assoc::Left) && get_prec(&token) <= get_prec(operators.last().unwrap()))
                    ||
                    (get_assoc(&token) == Some(Assoc::Right) && get_prec(&token) < get_prec(operators.last().unwrap()))) {
                let expression = pop(&mut operators)?;
                push_expr(expression, &mut expressions)?;
            }
            operators.push(token);
            expected_token_type = TokenType::Operand;
        } else if token == "(" {
            check(expected_token_type, TokenType::Operand)?;
            operators.push(token);
            expected_token_type = TokenType::Operand;
        } else if token == ")" {
            check(expected_token_type, TokenType::Operator)?;
            while !operators.is_empty() && operators.last().unwrap() != "(" {
                let expression = pop(&mut operators)?;
                push_expr(expression, &mut expressions)?;
            }
            if operators.is_empty() {
                return Err(String::from("missing opening parentheses '('"));
            }
            if operators.last().unwrap() == "(" {
                pop(&mut operators)?;
            }
            expected_token_type = TokenType::Operator;
        } else {
            check(expected_token_type, TokenType::Operand)?;
            push_expr(token, &mut expressions)?;
            expected_token_type = TokenType::Operator;
        }
    }

    while !operators.is_empty() {
        if operators.last().unwrap() == "(" {
            return Err(String::from("missing closing parentheses ')'"));
        }
        let expression = pop(&mut operators)?;
        push_expr(expression, &mut expressions)?;
    }

    let expression = pop(&mut expressions)?;
    Ok(expression)
}

fn tokenize(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    let mut is_escaped = false;
    let mut token: Option<String> = None;

    for c in expr.chars() {
        if c == ESCAPING_CHAR {
            is_escaped = true;
        } else {
            if c.is_whitespace() { // skip
                if let Some(t) = token.take() { // end of token
                    tokens.push(t);
                }
            } else {
                match c {
                    '(' | ')' => {
                        if !is_escaped {
                            if let Some(t) = token.take() { // end of token
                                tokens.push(t);
                            }
                            tokens.push(c.to_string());
                        }
                    },
                    _ => {
                        match token {
                            Some(ref mut t) => t.push(c),
                            None => token = Some(c.to_string()), // start of token
                        }
                    }
                }
            }
            is_escaped = false;
        }
    }

    if let Some(t) = token {
        tokens.push(t);
    }

    tokens
}

fn check(expected_token_type: TokenType, token_type: TokenType) -> Result<(), String> {
    if expected_token_type != token_type {
        return Err(format!("expression is incomplete, expected {} but got {}",
            expected_token_type.as_str().to_lowercase(), token_type.as_str().to_lowercase()));
    }

    Ok(())
}

fn pop<T>(stack: &mut Vec<T>) -> Result<T, String> {
    stack.pop()
        .ok_or_else(|| String::from("expression is incomplete"))
}

fn push_expr(token: String, stack: &mut Vec<Expression>) -> Result<(), String> {
    match token.as_ref() {
        "and" => {
            let right = pop(stack)?;
            let left = pop(stack)?;

            stack.push(Expression::And(AndExpression {
                left: Box::new(left),
                right: Box::new(right),
            }));
        },
        "or" => {
            let right = pop(stack)?;
            let left = pop(stack)?;

            stack.push(Expression::Or(OrExpression {
                left: Box::new(left),
                right: Box::new(right),
            }));
        },
        "not" => {
            let expression = pop(stack)?;
            stack.push(Expression::Not(NotExpression {
                expression: Box::new(expression),
            }));
        }
        _ => {
            stack.push(Expression::Literal(LiteralExpression {
                value: token
            }));
        }
    }

    Ok(())
}

fn is_unary(token: &str) -> bool {
    token == "not"
}

fn is_binary(token: &str) -> bool {
    token == "or" || token == "and"
}

fn is_operator(token: &str) -> bool {
    get_assoc(token).is_some()
}

#[derive(Debug, Eq, PartialEq)]
enum TokenType {
    Operand,
    Operator,
}

impl TokenType {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        use self::TokenType::*;

        match self {
            Operand => "Operand",
            Operator => "Operator",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Assoc {
    Left,
    Right,
}

fn get_assoc(token: &str) -> Option<Assoc> {
    match token {
        "or" => Some(Assoc::Left),
        "and" => Some(Assoc::Left),
        "not" => Some(Assoc::Right),
        _ => None,
    }
}

fn get_prec(token: &str) -> i8 {
    match token {
        "(" => -2,
        ")" => -1,
        "or" => 0,
        "and" => 1,
        "not" => 2,
        _ => panic!("cannot get precedence from invalid token: {}", token),
    }
}
