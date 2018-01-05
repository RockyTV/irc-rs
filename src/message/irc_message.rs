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
        self.raw_message == other.raw_message
    }
}

impl FromStr for IrcMessage {
    type Err = String;
    fn from_str(s: &str) -> Result<IrcMessage, String> {
        // :this.is.a.prefix COMMAND param1 param2 :param 3 with spaces
        let mut msg = s.trim();
        let mut raw_msg = String::from(msg);

        let mut prefix = String::new();
        let mut command = String::new();
        let mut params: Vec<String> = Vec::new();

        if msg.starts_with(":") {
            match msg.find(' ') {
                Some(whitespace) => {
                    prefix = match String::from_str(&msg[1..whitespace]) {
                        Ok(x) => x,
                        Err(_) => return Err(String::from("Error parsing message prefix")),
                    };

                    msg = &msg[whitespace + 1..];
                }
                None => {
                    return Err(String::from(
                        "Error parsing message prefix (invalid format)",
                    ))
                }
            };
        }

        if msg.contains(' ') {
            match msg.find(' ') {
                Some(whitespace) => {
                    command = match String::from_str(&msg[..whitespace]) {
                        Ok(x) => x,
                        Err(_) => return Err(String::from("Error parsing message command")),
                    };
                    msg = &msg[whitespace + 1..];

                    while msg != "" {
                        if msg.starts_with(":") {
                            params.push(String::from(&msg[1..]));
                            break;
                        }

                        if !msg.contains(' ') {
                            params.push(String::from(msg));
                            msg = "";
                            break;
                        }

                        match msg.find(' ') {
                            Some(whitespace) => {
                                params.push(String::from(&msg[..whitespace]));
                                msg = &msg[whitespace + 1..];
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
