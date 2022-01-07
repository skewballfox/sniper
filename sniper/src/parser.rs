use std::borrow::Cow;
use std::str::FromStr;

use nom::{
    self,
    branch::alt,
    bytes::streaming::{tag, take_till, take_until, take_while},
    character::streaming::{alphanumeric1, char, digit1},
    combinator::{map, map_res, opt},
    error::ParseError,
    multi::{many_till, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult, Parser,
};

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Tabstop(u32, Option<Vec<Token>>),
    Text(String),
    Variable(String, Option<String>),
    Snippet(String), //,Option<Vec<String>>),
}
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

///Used for top level raw text, grab everything until you hit $
fn text(snippet_string: &str) -> IResult<&str, Token> {
    //if $
    //\t$
    map(take_until("$"), |s: &str| Token::Text(s.into()))(snippet_string)
}
///Used for the names of snippets, may be extended to take arguments for those snippets
fn snippet_name(snippet_string: &str) -> IResult<&str, Token> {
    //$!if
    //{!elif} NOTE: still working out what snippet args should look like
    map(alphanumeric1, |s: &str| Token::Snippet(s.into()))(snippet_string)
}
fn variable(snippet_string: &str) -> IResult<&str, Token> {
    //TM_SELECTED_TEXT
    map(alphanumeric1, |s: &str| -> Token {
        Token::Variable(s.to_string(), None)
    })(snippet_string)
}
fn nested_variable(snippet_string: &str) -> IResult<&str, Token> {
    //{TM_SELECTED_TEXT}
    //${TM_FILENAME/(.*)\..+$/$1/}
    let (snippet_string, (res, args)) = pair(
        alphanumeric1,
        opt(map(preceded(tag("/"), take_until("}")), |s: &str| {
            s.to_string()
        })),
    )(snippet_string)?;

    return Ok((snippet_string, Token::Variable(res.into(), args)));
}

fn placeholder_text(snippet_string: &str) -> IResult<&str, Token> {
    //text}
    //|text,alternative|
    //another ${2:placeholder}
    let (snippet_string, snip_component): _ = take_till(|c| c == '$' || c == '}')(snippet_string)?;

    return Ok((snippet_string, Token::Text(snip_component.to_string())));
}

///function for everything that isn't raw text,children parsers are called depending on presence of brackets
fn non_text_token(snippet_string: &str) -> IResult<&str, Token> {
    //$!if, {!elif}
    //$TM_SELECTED_TEXT, ${TM_FILENAME/(.*)\..+$/$1/}
    //$1, ${1:expression}, ${1|text,alternative|}
    preceded(char('$'), alt((nested_component, raw_component)))(snippet_string)
}

fn nested_component(snippet_string: &str) -> IResult<&str, Token> {
    delimited(
        char('{'),
        alt((
            placeholder,    //{1},{1:text},{1|text,alternative|}
            snippet_object, //{!if},${TM_FILENAME/(.*)\..+$/$1/}
        )),
        char('}'),
    )(snippet_string)
}

fn raw_component(snippet_string: &str) -> IResult<&str, Token> {
    alt((
        tabstop,        //$1
        snippet_object, //$!if
        variable,       //TM_SELECTED_TEXT
    ))(snippet_string)
}

fn tabstop(snippet_string: &str) -> IResult<&str, Token> {
    //$1
    //NOTE: may simplify tabstop, placeholder, and placeholder arguments to a single function
    let (snippet_string, tabstop_value) =
        map_res(digit1, |s: &str| s.parse::<u32>())(snippet_string)?;

    Ok((snippet_string, Token::Tabstop(tabstop_value, None)))
}
fn placeholder(snippet_string: &str) -> IResult<&str, Token> {
    //${1:another ${2:placeholder}}
    //{1},{1:text},{1|text,alternative|}
    let (snippet_string, tabstop_value) =
        map_res(digit1, |s: &str| s.parse::<u32>())(snippet_string)?;
    let (snippet_string, tabstop_args) = placeholder_arguments(snippet_string)?;
    Ok((
        snippet_string,
        Token::Tabstop(tabstop_value, Some(tabstop_args)),
    ))
}

fn placeholder_arguments(snippet_string: &str) -> IResult<&str, Vec<Token>> {
    //:another ${2:placeholder}
    //:text},|text,alternative|
    let (snippet_string, placeholder_args) = alt((
        map(preceded(char(':'), placeholder_text), |res| vec![res]),
        delimited(
            char('|'),
            separated_list1(
                char(','),
                map(alphanumeric1, |s: &str| -> Token { Token::Text(s.into()) }),
            ), //TODO: could rework the syntax to support list of snippet components
            char('|'),
        ),
    ))(snippet_string)?;
    Ok((snippet_string, placeholder_args))
}
fn snippet_object(snippet_string: &str) -> IResult<&str, Token> {
    alt((
        preceded(
            tag("!"), //NOTE: depending on the implementation extra char specifier may not be necessary
            snippet_name,
        ),
        variable,
    ))(snippet_string)
}
//This will call text followed by non_text in a cycle, until the end of the stream
pub fn snippet_component(snippet_string: &str) -> IDONTKNOWYET {
    many_till(pair(text, opt(non_text_token)), tag("eof"));
}

#[cfg(test)]
mod test {
    use super::*;
    struct Snips {
        ifv: Vec<String>,
        elifv: Vec<String>,
        elsev: Vec<String>,
        ifelifelsev: Vec<String>,
    }
    impl Snips {
        fn new() -> Self {
            Self {
                ifv: vec!["if ${1:expression}:".into(), "\t${2:pass}".into()],
                elifv: vec!["elif ${1:expression}:".into(), "\t${1:pass}".into()],
                elsev: vec!["else:".into(), "\t${1:pass}".into()],
                ifelifelsev: vec!["$!if".into(), "$!elif".into(), "$!else".into()],
            }
        }
    }
    #[test]
    fn test_text() {
        let snips = Snips::new();
        let res = text(&snips.ifv[0]).unwrap();
        assert!(res.0.eq("${1:expression}:"));
        assert!(Token::Text("if ".to_string()) == res.1); //had to figure out to implement partialeq on the enum Token the hard way
    }

    #[test]
    fn test_non_text() {
        assert_eq!(
            non_text_token("${1:expression}:"),
            Ok(("{1:expression}", Token::Text("if ".into())))
        )
    }

    #[test]
    fn test_nested_component() {
        assert_eq!(
            nested_component("{1:another ${2:placeholder}}"),
            Ok((
                "",
                Token::Tabstop(
                    1,
                    Some(vec![
                        Token::Text("another ".to_string()),
                        Token::Tabstop(2, Some(vec![Token::Text("placeholder".to_string())]))
                    ])
                )
            ))
        ) //WIP
    }
}
