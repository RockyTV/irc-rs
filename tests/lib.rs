extern crate irc;

use irc::*;

#[test]
fn test_irc_message_from_str() {
    let raw_str = ":irc.server NOTICE * :*** Looking up your hostname...";
    let generated = Ok(IrcMessage {
        tags: None,
        prefix: Some(String::from("irc.server")),
        command: String::from("NOTICE"),
        params: Some(vec![
            String::from("*"),
            String::from("*** Looking up your hostname..."),
        ]),
        raw_message: String::from(raw_str),
    });
    let correct = IrcMessage::from_str(raw_str);

    assert_eq!(correct, generated);
}

#[test]
fn test_irc_message_from_str_prefix() {
    let raw_str = ":this.is.a.prefix NOTICE * :*** Looking up your hostname...";
    let correct = IrcMessage::from_str(raw_str).unwrap();

    assert_eq!(&correct.prefix.unwrap(), "this.is.a.prefix");
}

#[test]
fn test_irc_message_from_str_command() {
    let raw_str = ":this.is.a.prefix NOTICE * :*** Looking up your hostname...";
    let correct = IrcMessage::from_str(raw_str).unwrap();

    assert_eq!(&correct.command, "NOTICE");
}

#[test]
fn test_irc_message_from_str_params() {
    let raw_str = ":this.is.a.prefix NOTICE * :*** Looking up your hostname...";
    let correct = IrcMessage::from_str(raw_str).unwrap();

    let compare = vec!["*", "*** Looking up your hostname..."]
        .iter()
        .map(|param| String::from(*param))
        .collect();

    assert_eq!(correct.params, Some(compare));
}

#[test]
fn test_irc_message_from_str_params_2() {
    let raw_str = ":this.is.a.prefix NOTICE * :*** Looking up your hostname...";
    let correct = IrcMessage::from_str(raw_str).unwrap();

    let compare = vec!["*", ":*** Looking up your hostname..."]
        .iter()
        .map(|param| String::from(*param))
        .collect();

    assert_ne!(correct.params, Some(compare));
}

#[test]
fn test_irc_message_from_str_no_prefix() {
    let raw_str = "NOTICE * :*** Looking up your hostname...";
    let correct = IrcMessage::from_str(raw_str).unwrap();

    assert_eq!(correct.prefix, None);
}

#[test]
fn test_irc_message_from_str_tags() {
    let raw_str = "@aaa=bbb;ccc;example.com/ddd=eee :nick!ident@host.com PRIVMSG me :Hello";
    let correct = IrcMessage::from_str(raw_str).unwrap();
    let compare = vec![
        MessageTag {
            key: String::from("aaa"),
            value: Some(String::from("bbb")),
        },
        MessageTag {
            key: String::from("ccc"),
            value: None,
        },
        MessageTag {
            key: String::from("example.com/ddd"),
            value: Some(String::from("eee")),
        },
    ];

    assert_eq!(correct.tags, Some(compare));
}

#[test]
fn test_irc_message_tag_from_str() {
    let raw_str = "example.com/ddd=eee";
    let correct = MessageTag::from_str(raw_str).unwrap();
    let generated = MessageTag {
        key: String::from("example.com/ddd"),
        value: Some(String::from("eee")),
    };

    assert_eq!(correct, generated);
}
