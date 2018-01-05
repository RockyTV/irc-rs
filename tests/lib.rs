extern crate irc;

use irc::*;

#[test]
fn test_irc_message_from_str() {
    let raw_str = ":irc.server NOTICE * :*** Looking up your hostname...";
    let generated = Ok(IrcMessage {
        prefix: String::from("irc.server"),
        command: String::from("NOTICE"),
        params: vec![
            String::from("*"),
            String::from("*** Looking up your hostname..."),
        ],
        raw_message: String::from(raw_str),
    });
    let correct = IrcMessage::from_str(raw_str);

    assert_eq!(correct, generated);
}

#[test]
fn test_irc_message_from_str_prefix() {
    let raw_str = ":this.is.a.prefix NOTICE * :*** Looking up your hostname...";
    let correct = IrcMessage::from_str(raw_str).unwrap();

    assert_eq!(&correct.prefix, "this.is.a.prefix");
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

    let compare = vec!["*", "*** Looking up your hostname..."];

    assert_eq!(correct.params, compare);
}

#[test]
fn test_irc_message_from_str_params_2() {
    let raw_str = ":this.is.a.prefix NOTICE * :*** Looking up your hostname...";
    let correct = IrcMessage::from_str(raw_str).unwrap();

    let compare = vec!["*", ":*** Looking up your hostname..."];

    assert_ne!(correct.params, compare);
}
