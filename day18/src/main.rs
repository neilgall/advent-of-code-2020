use parser::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Num(i64),
    Add,
    Mul,
    Open,
    Close
}

fn tokenize(input: &str) -> ParseResult<Vec<Token>> {
    let token = whitespace_wrap(
        integer.map(Token::Num)
        .or(match_literal("+").means(Token::Add))
        .or(match_literal("*").means(Token::Mul))
        .or(match_literal("(").means(Token::Open))
        .or(match_literal(")").means(Token::Close))
    );

    one_or_more(token).parse(input)
}

fn shunting_yard(tokens: &[Token]) -> Vec<&Token> {
    let mut stack: Vec<&Token> = vec![];
    let mut result: Vec<&Token> = vec![];

    for token in tokens {
        match token {
            Token::Num(_) => {
                result.push(token)
            }

            Token::Add | Token::Mul => {
                while let Some(t) = stack.last() {
                    if t == &&Token::Add || t == &&Token::Mul {
                        result.push(*t);
                        stack.pop();
                    } else {
                        break;
                    }
                }
                stack.push(token)
            }

            Token::Open => {
                stack.push(token)
            }

            Token::Close => {
                while let Some(t) = stack.pop() {
                    if t == &Token::Open {
                        break
                    } else {
                        result.push(t);
                    }
                }
            }
        }   
    }

    while let Some(t) = stack.pop() {
        result.push(t);
    }

    result
}

fn eval_rp(tokens: &[&Token]) -> i64 {
    let mut stack: Vec<i64> = vec![];

    for token in tokens {
        match token {
            Token::Num(n) => {
                stack.push(*n)
            }

            Token::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }

            Token::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }

            _ => panic!("shunting yard should remove all parens!")
        }
    }

    stack.pop().unwrap()
}

fn eval(input: &str) -> i64 {
    let tokens = tokenize(input).unwrap().1;
    let rp = shunting_yard(&tokens);
    eval_rp(&rp)
}

fn part1(input: &str) -> i64 {
    input.lines().map(eval).sum()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    println!("part 1 {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        use Token::*;
        assert_eq!(tokenize("1 + 2 * (3+9)"), Ok(("", vec![
            Num(1), Add, Num(2), Mul, Open, Num(3), Add, Num(9), Close
        ])) );
    }

    #[test]
    fn test_shunting_yard_simple_add() {
        use Token::*;
        let input = [Num(1), Add, Num(2)];
        assert_eq!(shunting_yard(&input), vec![&Num(1), &Num(2), &Add])
    }

    #[test]
    fn test_shunting_yard_with_parens() {
        use Token::*;
        let input = [Num(1), Add, Open, Num(2), Mul, Num(3), Close, Add, Num(7)];
        assert_eq!(shunting_yard(&input), vec![&Num(1), &Num(2), &Num(3), &Mul, &Add, &Num(7), &Add])
    }

    #[test]
    fn test_eval_rp() {
        use Token::*;
        assert_eq!(eval_rp(&[&Num(1), &Num(2), &Num(3), &Mul, &Num(7), &Add, &Add]), 14);
    }

    #[test]
    fn test_eval() {
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }
}