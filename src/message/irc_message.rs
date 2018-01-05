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
    pub tags: Vec<MessageTag>,
    pub prefix: String,
    pub command: String,
    pub params: Vec<String>,
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

        let mut tags: Vec<MessageTag> = Vec::new();
        let mut prefix = String::new();
        let mut command = String::new();
        let mut params: Vec<String> = Vec::new();

        // Parse message tags
        if _msg.starts_with("@") {
            match _msg.find(' ') {
                Some(whitespace) => {
                    let _tags: &Vec<&str> = &_msg[1..whitespace].split(';').collect();
                    tags = _tags
                        .iter()
                        .map(|x| MessageTag::from_str(x).unwrap())
                        .collect::<Vec<MessageTag>>();
                }
                None => return Err(String::from("Error parsing message tags (invalid format)")),
            }
        }

        // Parse prefix
        if _msg.starts_with(":") {
            match _msg.find(' ') {
                Some(whitespace) => {
                    prefix = match String::from_str(&_msg[1..whitespace]) {
                        Ok(x) => x,
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

                    while _msg != "" {
                        if _msg.starts_with(":") {
                            params.push(String::from(&_msg[1..]));
                            break;
                        }

                        if !_msg.contains(' ') {
                            params.push(String::from(_msg));
                            let _msg: &str;
                            break;
                        }

                        match _msg.find(' ') {
                            Some(whitespace) => {
                                params.push(String::from(&_msg[..whitespace]));
                                _msg = &_msg[whitespace + 1..];
                            }
                            None => {
                                return Err(String::from(
                                    "Error parsing message parameters (invalid format)",
                                ))
                            }
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
