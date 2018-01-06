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
                _key = s[..equals].to_string();
                _value = Some(s[equals + 1..].to_string());
            }
            None => {
                _key = s.to_string();
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
        let mut msg = s.trim();
        let raw_msg = msg.to_string();

        let mut tags: Option<Vec<MessageTag>> = None;
        let mut prefix: Option<String> = None;
        let mut command = String::new();
        let mut params: Option<Vec<String>> = None;

        // Parse message tags
        if msg.starts_with("@") {
            match msg.find(' ') {
                Some(whitespace) => {
                    match msg[1..whitespace]
                        .split(';')
                        .into_iter()
                        .map(|tag| MessageTag::from_str(tag))
                        .collect::<Result<Vec<MessageTag>, String>>()
                    {
                        Ok(x) => tags = Some(x),
                        Err(_) => return Err("Error parsing message tags".to_string()),
                    };
                }
                None => return Err("Error parsing message tags (invalid format)".to_string()),
            }
        }

        // Parse prefix
        if msg.starts_with(":") {
            match msg.find(' ') {
                Some(whitespace) => {
                    prefix = Some(msg[1..whitespace].to_string());
                    msg = &msg[whitespace + 1..];
                }
                None => return Err("Error parsing message prefix (invalid format)".to_string()),
            };
        }

        // Parse command and treat the remaining strings as parameters
        if msg.contains(' ') {
            match msg.find(' ') {
                Some(whitespace) => {
                    command = msg[..whitespace].to_string();
                    msg = &msg[whitespace + 1..];

                    let mut raw_params: Vec<&str> = Vec::new();
                    match msg.find(':') {
                        Some(colon) => {
                            let (params_string, last_param) = msg.split_at(colon);
                            raw_params.extend(params_string.split_whitespace());
                            raw_params.push(&last_param[1..]);
                            params = Some(
                                raw_params
                                    .into_iter()
                                    .map(|param| param.to_string())
                                    .collect::<Vec<String>>(),
                            );
                        }
                        None => {
                            raw_params.extend(msg.split_whitespace());
                            params = Some(
                                raw_params
                                    .into_iter()
                                    .map(|param| param.to_string())
                                    .collect::<Vec<String>>(),
                            );
                        }
                    }
                }
                None => return Err("Error parsing message command (invalid format)".to_string()),
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
