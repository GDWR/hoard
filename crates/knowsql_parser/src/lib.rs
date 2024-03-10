use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::alphanumeric1,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Get(&'a str),
    Set(KeyValue<'a>),
    MSet(Vec<KeyValue<'a>>),
    Increment(&'a str),
    List,
    Exit,
}

fn parse_key(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn parse_set(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("set ")(input)?;
    let (input, key) = parse_key(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, value) = parse_value(input)?;
    Ok((input, Command::Set(KeyValue { key, value })))
}

fn parse_mset(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("mset ")(input)?;
    let (input, key_values) =
        separated_list1(tag(" "), separated_pair(parse_key, tag(" "), parse_value))(input)?;

    let x = key_values
        .iter()
        .map(|(key, value)| KeyValue { key, value })
        .collect::<Vec<KeyValue>>();

    Ok((input, Command::MSet(x)))
}

fn parse_get(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("get ")(input)?;
    let (input, key) = parse_key(input)?;
    Ok((input, Command::Get(key)))
}

fn parse_increment(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("incr ")(input)?;
    let (input, key) = parse_key(input)?;
    Ok((input, Command::Increment(key)))
}

fn parse_list(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("list")(input)?;
    Ok((input, Command::List))
}

fn parse_exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("exit")(input)?;
    Ok((input, Command::Exit))
}

pub fn parse_command(input: &str) -> Option<Command> {
    let mut parser = alt((
        parse_get,
        parse_set,
        parse_mset,
        parse_increment,
        parse_list,
        parse_exit,
    ));

    match parser(input) {
        Ok((_, command)) => Some(command),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get() {
        assert_eq!(parse_get("get key"), Ok(("", Command::Get("key"))));
    }

    #[test]
    fn test_parse_set() {
        assert_eq!(
            parse_set("set key value"),
            Ok((
                "",
                Command::Set(KeyValue {
                    key: "key",
                    value: "value"
                })
            ))
        );
    }

    #[test]
    fn test_parse_mset() {
        assert_eq!(
            parse_mset("mset key1 value1 key2 value2"),
            Ok((
                "",
                Command::MSet(vec![
                    KeyValue {
                        key: "key1",
                        value: "value1"
                    },
                    KeyValue {
                        key: "key2",
                        value: "value2"
                    }
                ])
            ))
        );
    }

    #[test]
    fn test_parse_increment() {
        assert_eq!(
            parse_increment("incr key"),
            Ok(("", Command::Increment("key")))
        );
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(parse_list("list"), Ok(("", Command::List)));
    }

    #[test]
    fn test_parse_exit() {
        assert_eq!(parse_exit("exit"), Ok(("", Command::Exit)));
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("get key"), Some(Command::Get("key")));
        assert_eq!(
            parse_command("set key value"),
            Some(Command::Set(KeyValue {
                key: "key",
                value: "value"
            }))
        );
        assert_eq!(
            parse_command("mset key1 value1 key2 value2"),
            Some(Command::MSet(vec![
                KeyValue {
                    key: "key1",
                    value: "value1"
                },
                KeyValue {
                    key: "key2",
                    value: "value2"
                }
            ]))
        );
        assert_eq!(parse_command("incr key"), Some(Command::Increment("key")));
        assert_eq!(parse_command("list"), Some(Command::List));
        assert_eq!(parse_command("exit"), Some(Command::Exit));
        assert_eq!(parse_command("invalid"), None);
    }

    #[test]
    fn test_parse_command_case_insensitive() {
        assert_eq!(parse_command("GEt key"), Some(Command::Get("key")));
        assert_eq!(
            parse_command("SeT key value"),
            Some(Command::Set(KeyValue {
                key: "key",
                value: "value"
            }))
        );
        assert_eq!(parse_command("LIST"), Some(Command::List));
        assert_eq!(parse_command("eXIT"), Some(Command::Exit));
    }
}
