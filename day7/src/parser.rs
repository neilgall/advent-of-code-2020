// -- core parser trait

pub type ParseResult<'a, T> = Result<(&'a str, T), (&'static str, &'a str)>;

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T>;

    fn map<F, U>(self, map_fn: F) -> BoxedParser<'a, U>
        where
            Self: Sized + 'a,
            T: 'a,
            U: 'a,
            F: Fn(T) -> U + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn means<U>(self, value: U) -> BoxedParser<'a, U>
        where
            Self: Sized + 'a,
            T: 'a,
            U: Copy + 'a
    {
        BoxedParser::new(map(self, move |_| value))
    }

    fn or<P>(self, p: P) -> BoxedParser<'a, T>
        where
            Self: Sized + 'a,
            P: Parser<'a, T> + 'a,
            T: 'a 
    {
        BoxedParser::new(or(self, p))
    }

    fn and_then<F, P, U>(self, f: F) -> BoxedParser<'a, U>
        where
            Self: Sized + 'a,
            T: 'a,
            U: 'a,
            P: Parser<'a, U> + 'a,
            F: Fn(T) -> P + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }

}

impl<'a, F, T> Parser<'a, T> for F where F: Fn(&'a str) -> ParseResult<T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
        self(input)
    }
}

// --- boxed parser

pub struct BoxedParser<'a, T> {
    parser: Box<dyn Parser<'a, T> + 'a>,
}

impl<'a, T> BoxedParser<'a, T> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, T> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, T> Parser<'a, T> for BoxedParser<'a, T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
        self.parser.parse(input)
    }
}

// --- bare combinators

fn map<'a, P, F, A, B>(parser: P, f: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B
{
    move |input|
        parser.parse(input)
              .map(|(rest, result)| (rest, f(result)))
}

pub fn pair<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, (R1, R2)>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>
{
    move |input|
        p1.parse(input).and_then(|(rest, r1)|
            p2.parse(rest).map(|(rest2, r2)| (rest2, (r1, r2))))
} 

pub fn first<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, R1>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>
{
    map(pair(p1, p2), |(r, _)| r)
}

pub fn second<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, R2>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>
{
    map(pair(p1, p2), |(_, r)| r)
}

pub fn between<'a, P1, PS, P2, R1, RS, R2>(p1: P1, ps: PS, p2: P2) -> impl Parser<'a, (R1, R2)>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
        PS: Parser<'a, RS>
{
    pair(first(p1, ps), p2)
}

pub fn or<'a, P1, P2, R>(p1: P1, p2: P2) -> impl Parser<'a, R>
    where
        P1: Parser<'a, R>,
        P2: Parser<'a, R>
{
    move |input|
        p1.parse(input).or_else(|_| p2.parse(input))
}


pub fn one_or_more<'a, P, A>(p: P) -> impl Parser<'a, Vec<A>>
    where
        P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        match p.parse(input) {
            Ok((next_input, first_item)) => {
                input = next_input;
                result.push(first_item);
            }
            Err((parser, input)) => {
                return Err((parser, input));
            }
        }

        while let Ok((next_input, next_item)) = p.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

// ---- parser primitives

pub fn digit(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if c.is_digit(10) => {
            let rest = &input[c.len_utf8()..];
            Ok((rest, c))
        }
        _ => Err(("digit", input))
    }
}

pub fn letter(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if c.is_alphabetic() => {
            let rest = &input[c.len_utf8()..];
            Ok((rest, c))
        }
        _ => Err(("letter", input))
    }
}

pub fn non_whitespace(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if !c.is_whitespace() => {
            let rest = &input[c.len_utf8()..];
            Ok((rest, c))
        }
        _ => Err(("non_whitespace", input))
    }
}

pub fn integer(input: &str) -> ParseResult<i64> {
    let digit_as_num = map(digit, |d| (d as i64) - 48);

    if let Ok((rest, first_digit)) = digit_as_num.parse(input) {
        let mut i = first_digit;
        let mut remainder = rest;
        while let Ok((rest, next_digit)) = digit_as_num.parse(remainder) {
            i = i * 10 + next_digit;
            remainder = rest;
        }
        Ok((remainder, i))
    } else {
        Err(("integer", input))
    }
}

pub fn string<'a>(s: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| {
        match input.get(0..s.len()) {
            Some(r) if r == s => {
                let rest = &input[s.len()..];
                Ok((rest, ()))
            }
            _ => Err(("string", input))
        }
    }
}

pub fn whitespace(input: &str) -> ParseResult<()> {
    Ok((input.trim_start(), ()))
}
