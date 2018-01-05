use std::str::FromStr;

#[derive(Debug)]
pub struct MessageTag {
    pub key: String,
    pub value: Option<String>,
}

impl PartialEq for MessageTag {
    fn eq(&self, other: &MessageTag) -> bool {
        return self.key == other.key && self.value == other.value;
    }
}

impl FromStr for MessageTag {
    type Err = String;
    fn from_str(s: &str) -> Result<MessageTag, String> {
        let mut _key = String::new();
        let mut _value: Option<String>;

        match s.find('=') {
            Some(equals) => {
                _key = String::from_str(&s[..equals]).unwrap();
                _value = Some(String::from_str(&s[equals + 1..]).unwrap());
            }
            None => {
                _key = String::from(s);
                _value = None;
            }
        }

        Ok(MessageTag {
            key: _key,
            value: _value,
        })
    }
}

#[derive(Debug)]
pub struct IrcMessage {
    pub tags: Option<Vec<MessageTag>>,
    pub prefix: Option<String>,
    pub command: String,
    pub params: Option<Vec<String>>,
    pub raw_message: String,
}

impl PartialEq for IrcMessage {
    fn eq(&self, other: &IrcMessage) -> bool {
        self.prefix == other.prefix && self.command == other.command && self.params == other.params
    }
}

impl FromStr for IrcMessage {
    type Err = String;
    fn from_str(s: &str) -> Result<IrcMessage, String> {
        // Inspired by SirCmpwn's ChatSharp library
        let mut _msg = s.trim();
        let raw_msg = String::from(_msg);

        let mut tags: Option<Vec<MessageTag>> = None;
        let mut prefix: Option<String> = None;
        let mut command = String::new();
        let mut params: Option<Vec<String>> = None;

        // Parse message tags
        if _msg.starts_with("@") {
            match _msg.find(' ') {
                Some(whitespace) => {
                    let _tags: &Vec<&str> = &_msg[1..whitespace].split(';').collect();
                    tags = Some(
                        _tags
                            .iter()
                            .map(|x| MessageTag::from_str(x).unwrap())
                            .collect::<Vec<MessageTag>>(),
                    );
                }
                None => return Err(String::from("Error parsing message tags (invalid format)")),
            }
        }

        // Parse prefix
        if _msg.starts_with(":") {
            match _msg.find(' ') {
                Some(whitespace) => {
                    prefix = match String::from_str(&_msg[1..whitespace]) {
                        Ok(x) => Some(x),
                        Err(_) => {
                            return Err(String::from(
                                "Error parsing message prefix (&str) to String",
                            ))
                        }
                    };

                    _msg = &_msg[whitespace + 1..];
                }
                None => {
                    return Err(String::from(
                        "Error parsing message prefix (invalid format)",
                    ))
                }
            };
        }

        // Parse command and treat the remaining strings as parameters
        if _msg.contains(' ') {
            match _msg.find(' ') {
                Some(whitespace) => {
                    command = match String::from_str(&_msg[..whitespace]) {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(String::from(
                                "Error parsing message command (&str) to String",
                            ))
                        }
                    };
                    _msg = &_msg[whitespace + 1..];

                    let mut _params: Vec<&str> = Vec::new();
                    match _msg.find(':') {
                        Some(colon) => {
                            let _raw_params = _msg.split_at(colon);
                            _params.extend(_raw_params.0.split_whitespace());
                            _params.push(&_raw_params.1[1..]);
                            params = Some(
                                _params
                                    .iter()
                                    .map(|x| String::from_str(x).unwrap())
                                    .collect::<Vec<String>>(),
                            );
                        }
                        None => {
                            _params.extend(_msg.split_whitespace());
                            params = Some(
                                _params
                                    .iter()
                                    .map(|param| String::from_str(param).unwrap())
                                    .collect::<Vec<String>>(),
                            );
                        }
                    }
                }
                None => {
                    return Err(String::from(
                        "Error parsing message command (invalid format)",
                    ))
                }
            };
        }

        Ok(IrcMessage {
            tags: tags,
            prefix: prefix,
            command: command,
            params: params,
            raw_message: raw_msg,
        })
    }
}
