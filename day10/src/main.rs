use nom::character::complete::char;
use nom::multi::many0;
use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use nom::error::{context, VerboseError};



type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug)]
enum ScopeType {
    Parenthesis,
    Brackets,
    CurlyBrackets,
    Chevrons,
}

#[derive(Debug)]
struct Scope {
    inner: Vec<Scope>,
    kind: ScopeType
}

fn parenthesis(input: &str) -> Res<&str, Scope> {
    let (input, (_, inner, _)) = context("parenthesis",
        tuple((char('('), many0(scope), char(')'))))(input)?;
    Ok((input, Scope{
        inner,
        kind: ScopeType::Parenthesis
    }))
}
fn brackets(input: &str) -> Res<&str, Scope> {
    let (input, (_, inner, _)) = context("brackets",
     tuple((char('['), many0(scope), char(']'))))(input)?;
    Ok((input, Scope{
        inner,
        kind: ScopeType::Brackets
    }))
}
fn curly_brackets(input: &str) -> Res<&str, Scope> {
    let (input, (_, inner, _)) = context("curly_brackets",
     tuple((char('{'), many0(scope), char('}'))))(input)?;
    Ok((input, Scope{
        inner,
        kind: ScopeType::CurlyBrackets
    }))
}
fn chevrons(input: &str) -> Res<&str, Scope> {
    let (input, (_, inner, _)) = context("chevrons",
     tuple((char('<'), many0(scope), char('>'))))(input)?;
    Ok((input, Scope{
        inner,
        kind: ScopeType::Chevrons
    }))
}

fn scope(input: &str) -> Res<&str, Scope> {
    alt((
        parenthesis,
        brackets,
        curly_brackets,
        chevrons,
    ))(input)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}


fn parse(input: &str) -> usize {
    0
}

fn solve_2(input: &str) -> usize {
    0
}

fn solve_1(input: &str) -> usize {
    0
}

#[test]
fn test_1line() {
    let line = "{([(<{}[<>[]}>{[]{[(<()>";
    let res = scope(line);

    match res {
        Ok((rest, scope)) => {
            println!("rest: {}, scope {:#?}", rest, scope);
        }
        Err(err) => {
            println!("err: {:#?}", err);
        }
        
    }
}

#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 26397)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 12)
}