// -- core parser combinators

pub type ParseResult<'a, T> = Result<(&'a str, T), (&'static str, &'a str)>;

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T>;
}

impl<'a, F, T> Parser<'a, T> for F where F: Fn(&'a str) -> ParseResult<T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
        self(input)
    }
}

pub fn map<'a, P, F, A, B>(parser: P, f: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B
{
    move |input|
        parser.parse(input)
              .map(|(rest, result)| (rest, f(result)))
}

pub fn seq<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, (R1, R2)>
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
    map(seq(p1, p2), |(r, _)| r)
}

pub fn second<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, R2>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>
{
    map(seq(p1, p2), |(_, r)| r)
}

// ---- parser primitives

pub fn digit(input: &str) -> ParseResult<i64> {
    match input.chars().next() {
        Some(c) if c.is_digit(10) => {
            let rest = &input[c.len_utf8()..];
            Ok((rest, (c as i64) - 48))
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

pub fn integer(input: &str) -> ParseResult<i64> {
    if let Ok((rest, first_digit)) = digit(input) {
        let mut i = first_digit;
        let mut remainder = rest;
        while let Ok((rest, next_digit)) = digit(remainder) {
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

pub fn one_or_more<'a, P, A>(p: P) -> impl Parser<'a, Vec<A>>
    where
        P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = p.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(("one or more", input));
        }

        while let Ok((next_input, next_item)) = p.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}