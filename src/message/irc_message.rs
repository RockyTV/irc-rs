use std::str::FromStr;

#[derive(Debug)]
pub struct IrcMessage {
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

        let mut prefix = String::new();
        let mut command = String::new();
        let mut params: Vec<String> = Vec::new();

        if _msg.starts_with(":") {
            match _msg.find(' ') {
                Some(whitespace) => {
                    prefix = match String::from_str(&_msg[1..whitespace]) {
                        Ok(x) => x,
                        Err(_) => return Err(String::from("Error parsing message prefix")),
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

        if _msg.contains(' ') {
            match _msg.find(' ') {
                Some(whitespace) => {
                    command = match String::from_str(&_msg[..whitespace]) {
                        Ok(x) => x,
                        Err(_) => return Err(String::from("Error parsing message command")),
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
                            None => return Err(String::from("Error parsing message parameters")),
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
            prefix: prefix,
            command: command,
            params: params,
            raw_message: raw_msg,
        })
    }
}
