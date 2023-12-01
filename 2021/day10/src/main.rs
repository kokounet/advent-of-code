use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use Token::*;
use Frag::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Frag {
    Paren,
    Square,
    Curly,
    Angle,
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Token {
    Open(Frag),
    Close(Frag),
}


fn score1(frag: &Frag) -> i64 {
    match frag {
        Paren => 3,
        Square => 57,
        Curly => 1197,
        Angle => 25137,
    }
}


fn score2(frag: &Frag) -> i64 {
    match frag {
        Paren => 1,
        Square => 2,
        Curly => 3,
        Angle => 4,
    }
}


fn tokenize(line: &String) -> Vec<Token> {
    line.chars()
        .map(|c| match c {
            '(' => Open(Paren),
            ')' => Close(Paren),
            '[' => Open(Square),
            ']' => Close(Square),
            '{' => Open(Curly),
            '}' => Close(Curly),
            '<' => Open(Angle),
            '>' => Close(Angle),
            _ => unimplemented!("error management!"),
        })
        .collect()
}

fn parse1(tokens: Vec<Token>) -> i64 {
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Open(frag) => stack.push(frag),
            Close(frag) => match stack.pop() {
                Some(expected) if expected != frag => {
                    return score1(&frag);
                }
                None => break,
                _ => {},
            }
        }
    }
    0
}


fn parse2(tokens: Vec<Token>) -> Option<i64> {
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Open(frag) => stack.push(frag),
            Close(frag) => match stack.pop() {
                Some(expected) if expected != frag => return None,
                None => return None,
                _ => {},
            }
        }
    }
    stack.reverse();
    Some(stack.iter().map(score2).fold(0, |acc, score| 5*acc + score))
}

fn solution1(lines: &Vec<String>) -> i64 {
    lines
        .into_iter()
        .map(tokenize)
        .map(parse1)
        .sum()
}


fn solution2(lines: &Vec<String>) -> i64 {
    let mut scores = lines
        .into_iter()
        .map(tokenize)
        .filter_map(parse2).collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}


fn main() -> Result<(), Error> {
    let lines = BufReader::new(File::open("input.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    println!("{}", solution1(&lines));
    println!("{}", solution2(&lines));
    Ok(())
}
