/*
   Used during get_snippet request. This handles the logic for parsing the
   bodies of the snippets and turning them into a string of tokens, which
   are then converted into components

   Note: I plan on refactoring this process so that the parser creates components
   Directly, should cut down on the processing time for get_snippet request
*/
use nom::{
    self,
    branch::alt,
    bytes::streaming::{tag, take_till, take_until, take_while},
    character::streaming::{alphanumeric1, char, digit1},
    combinator::{iterator, map, map_res, opt},
    error::ParseError,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    ///a tabstop, the Option<vec<Token>> is a vector of default values
    TabstopToken(u32, Option<Vec<Token>>),
    ///aka "kitchen sink" Token, captures the unmanipulated raw text
    TextToken(String),
    ///snippet variables which warrant actions, the second optional field is for transforms
    VariableToken(String, Option<String>),
    ///basically the name of another snippet, to be recursively parse
    SnippetToken(String), //,Option<Vec<String>>),
}

//This will call text followed by non_text in a cycle, until the end of the stream
///Top level function for the parser, probably the only one you want to use unless extending the
/// parser itself
/// this takes a snippet string and returns a vector of Tokens
pub(crate) fn snippet_component(snippet_string: &str) -> Vec<Token> {
    iterator(
        snippet_string,
        fold_many1(
            pair(opt(text), opt(non_text_token)),
            Vec::new,
            |mut acc: Vec<_>, (first, second)| {
                if let Some(res) = first {
                    acc.push(res)
                };
                if let Some(res) = second {
                    acc.push(res)
                };
                acc
            },
        ),
    )
    .flatten()
    .collect::<Vec<_>>()
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
    map(take_until("$"), |s: &str| Token::TextToken(s.into()))(snippet_string)
}

///function for everything that isn't raw text,children parsers are called depending on presence of brackets
fn non_text_token(snippet_string: &str) -> IResult<&str, Token> {
    //$!if, {!elif}
    //$TM_SELECTED_TEXT, ${TM_FILENAME/(.*)\..+$/$1/}
    //$1, ${1:expression}, ${1|text,alternative|}
    preceded(char('$'), alt((nested_component, raw_component)))(snippet_string)
}

///Used for non_text_tokens without brackets, which don't take arguments
fn raw_component(snippet_string: &str) -> IResult<&str, Token> {
    alt((
        tabstop,      //$1
        snippet_name, //$!if
        variable,     //TM_SELECTED_TEXT
    ))(snippet_string)
}

fn nested_component(snippet_string: &str) -> IResult<&str, Token> {
    delimited(
        char('{'),
        alt((
            placeholder,     //{1}, {1:text}, {1|text,alternative|}
            snippet_object,  //${!if}, etc...
            nested_variable, //{!if}, {TM_FILENAME/(.*)\..+$/$1/}
        )),
        char('}'),
    )(snippet_string)
}

//NOTE: working out the details on how to support programmatic snippets
//but I'm thinking about handling variables as functors with only one step

///basic variable without transform
fn variable(snippet_string: &str) -> IResult<&str, Token> {
    //TM_SELECTED_TEXT
    map(alphanumeric1, |s: &str| -> Token {
        Token::VariableToken(s.to_string(), None)
    })(snippet_string)
}

///variable in bracket which may have a transform
fn nested_variable(snippet_string: &str) -> IResult<&str, Token> {
    //{TM_SELECTED_TEXT}
    //${TM_FILENAME/(.*)\..+$/$1/}
    let (snippet_string, (res, args)) = pair(
        alphanumeric1,
        opt(map(preceded(tag("/"), take_until("}")), |s: &str| {
            s.to_string()
        })),
    )(snippet_string)?;

    return Ok((snippet_string, Token::VariableToken(res.into(), args)));
}

///used for basic tabstops which don't have optional arguments
fn tabstop(snippet_string: &str) -> IResult<&str, Token> {
    //$1
    //NOTE: may simplify tabstop, placeholder, and placeholder arguments to a single function
    let (snippet_string, tabstop_value) =
        map_res(digit1, |s: &str| s.parse::<u32>())(snippet_string)?;

    Ok((snippet_string, Token::TabstopToken(tabstop_value, None)))
}

///used for placeholders which may have values or a list of possible values
fn placeholder(snippet_string: &str) -> IResult<&str, Token> {
    //${1:another ${2:placeholder}}
    //{1},{1:text},{1|text,alternative|}
    let (snippet_string, tabstop_value) =
        map_res(digit1, |s: &str| s.parse::<u32>())(snippet_string)?;
    let (snippet_string, tabstop_args) = placeholder_arguments(snippet_string)?;
    Ok((
        snippet_string,
        Token::TabstopToken(tabstop_value, Some(tabstop_args)),
    ))
}

///Used for the content of the placeholders, which are snippet tokens themselves
/// this could be used for an almost endless customization options even without programmatic snippets
fn placeholder_arguments(snippet_string: &str) -> IResult<&str, Vec<Token>> {
    //:another ${2:placeholder}
    //:text},|text,alternative|
    let (snippet_string, placeholder_args) = alt((
        map(preceded(char(':'), placeholder_text), |res| vec![res]),
        delimited(
            char('|'),
            separated_list1(
                char(','),
                map(alphanumeric1, |s: &str| -> Token {
                    Token::TextToken(s.into())
                }),
            ), //TODO: could rework the syntax to support list of snippet components
            char('|'),
        ),
    ))(snippet_string)?;
    Ok((snippet_string, placeholder_args))
}

///used for the text which composes a placeholder argument, which needs to stop in multiple cases
/// may wind up needing to split this into multiple functions
fn placeholder_text(snippet_string: &str) -> IResult<&str, Token> {
    //text}
    //|text,alternative|
    //another ${2:placeholder}
    let (snippet_string, snip_component): _ = take_till(
        |c| c == '$' || c == '}', //|| c == '|' || c == ','
    )(snippet_string)?;

    return Ok((snippet_string, Token::TextToken(snip_component.to_string())));
}

///Used for the names of snippets, may be extended to take arguments for those snippets
fn snippet_name(snippet_string: &str) -> IResult<&str, Token> {
    //$!if
    //{!elif} NOTE: still working out what snippet args should look like
    map(alphanumeric1, |s: &str| Token::SnippetToken(s.into()))(snippet_string)
}

///placeholder for function which will handle nested snippet=s with optional arguments
fn snippet_object(snippet_string: &str) -> IResult<&str, Token> {
    snippet_name(snippet_string)
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
        assert!(Token::TextToken("if ".to_string()) == res.1); //had to figure out to implement partialeq on the enum Token the hard way
    }

    #[test]
    fn test_non_text() {
        assert_eq!(
            non_text_token("${1:expression}:"),
            Ok(("{1:expression}", Token::TextToken("if ".into())))
        )
    }

    #[test]
    fn test_nested_component() {
        assert_eq!(
            nested_component("{1:another ${2:placeholder}}"),
            Ok((
                "",
                Token::TabstopToken(
                    1,
                    Some(vec![
                        Token::TextToken("another ".to_string()),
                        Token::TabstopToken(
                            2,
                            Some(vec![Token::TextToken("placeholder".to_string())])
                        )
                    ])
                )
            ))
        ) //WIP
    }
}
