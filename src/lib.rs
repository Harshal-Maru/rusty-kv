use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCommand { description: String },
    MissingArguments { description: String },
}

pub fn parse_command(input: &str) -> Result<Command, ParseError> {
    let mut parts = input.split_whitespace();

    let cmd = parts.next();

    match cmd {
        Some("Set") => {
            let key = parts.next();
            let value = parts.next();

            match key {
                Some(k) => match value {
                    Some(v) => {
                        return Ok(Command::Set {
                            key: k.to_string(),
                            value: v.to_string(),
                        });
                    }
                    None => {
                        return Err(ParseError::MissingArguments {
                            description: ("Value is Missing".to_string()),
                        });
                    }
                },
                None => {
                    return Err(ParseError::MissingArguments {
                        description: ("Key is Missing".to_string()),
                    });
                }
            }
        }
        Some("Get") => {
            let key = parts.next();

            match key {
                Some(k) => {
                    return Ok(Command::Get { key: k.to_string() });
                }
                None => {
                    return Err(ParseError::MissingArguments {
                        description: ("Key is Missing".to_string()),
                    });
                }
            }
        }
        Some("Remove") => {
            let key = parts.next();

            match key {
                Some(k) => {
                    return Ok(Command::Remove { key: k.to_string() });
                }
                None => {
                    return Err(ParseError::MissingArguments {
                        description: ("Key is Missing".to_string()),
                    });
                }
            }
        }
        None => {
            return Err(ParseError::MissingArguments {
                description: "Command is Missing".to_string(),
            });
        }
        _ => {
            return Err(ParseError::InvalidCommand {
                description: "Command is Wrong".to_string(),
            });
        }
    }
}

pub struct KvStore {
    entries: HashMap<String, String>,
}

impl KvStore {
    // Create a new, empty Key-Value Store
    pub fn new() -> Self {
        KvStore {
            entries: HashMap::new(),
        }
    }

    // Set a key value pair
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.entries.insert(key, value)
    }

    // Get a key value pair, given the key
    pub fn get(&self, key: String) -> Option<String> {
        self.entries.get(&key).cloned()
    }

    // Remove a key value pair, given the key and return the value
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.entries.remove(&key)
    }
}

#[cfg(test)]
mod tests {
    // Import everything from the outer module (your KvStore)
    use super::*;

    // The #[test] annotation marks a function as a unit test.
    #[test]
    fn test_set_and_get() {
        let mut store = KvStore::new();

        // We use .to_string() to convert string literals (&str) into owned Strings.
        store.set("key1".to_string(), "value1".to_string());
        store.set("key2".to_string(), "value1".to_string());
        store.set("key2".to_string(), "value2".to_string());
        store.set("Name".to_string(), "Harshal".to_string());

        // assert_eq! checks if both sides are equal. If not, the test fails.
        assert_eq!(store.get("key1".to_string()), Some("value1".to_string()));
        assert_eq!(store.get("key2".to_string()), Some("value2".to_string()));
        assert_eq!(store.get("Name".to_string()), Some("Harshal".to_string()));
        assert_eq!(store.remove("key1".to_string()), Some("value1".to_string()));
        assert_eq!(store.remove("key1".to_string()), None);
    }

    #[test]
    fn test_parse_valid_commands() {
        // Test SET
        assert_eq!(
            parse_command("Set name Harshal"),
            Ok(Command::Set {
                key: "name".to_string(),
                value: "Harshal".to_string()
            })
        );

        // Test GET
        assert_eq!(
            parse_command("Get name"),
            Ok(Command::Get {
                key: "name".to_string()
            })
        );

        // Test Remove
        assert_eq!(
            parse_command("Remove name"),
            Ok(Command::Remove {
                key: "name".to_string()
            })
        );
    }

    #[test]
    fn test_parse_missing_arguments() {
        // GET missing its key
        assert_eq!(
            parse_command("Get"),
            Err(ParseError::MissingArguments {
                description: "Key is Missing".to_string()
            })
        );

        // SET missing its value
        assert_eq!(
            parse_command("Set mykey"),
            Err(ParseError::MissingArguments {
                description: "Value is Missing".to_string()
            })
        );
    }

    #[test]
    fn test_parse_invalid_command() {
        // A command that doesn't exist
        assert_eq!(
            parse_command("HELLO world"),
            Err(ParseError::InvalidCommand {
                description: "Command is Wrong".to_string()
            })
        );
    }
}
