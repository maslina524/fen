use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::format;

#[derive(Debug, Clone)]
struct Arg {
    long: String,
    short: Option<char>,
    takes_value: bool
}

#[derive(Debug, Clone)]
pub struct ArgsParser {
    command_name: String,
    description: String,
    arguments: Vec<Arg>
}

#[derive(Debug, Clone)]
pub struct ArgResult {
    pub map: BTreeMap<String, String>,
    pub nn: Vec<String>,
    pub action: String
}

impl ArgsParser {
    pub fn new(command_name: &str, description: &str) -> Self {
        ArgsParser {
            command_name: command_name.to_owned(),
            description: description.to_owned(),
            arguments: Vec::new()
        }
    }

    pub fn add_arg(&mut self, long: &str, short: Option<char>, takes_value: bool) {
        self.arguments.push(Arg {
            long: long.to_owned(),
            short,
            takes_value,
        });
    }

    pub fn parse(&self, args: &[String]) -> Result<ArgResult, String> {
        let mut map = BTreeMap::new();
        let mut nn = Vec::new();
        let mut action = String::new();
        let mut action_set = false;
        let mut stop_parsing = false;
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if !stop_parsing && arg == "--" {
                stop_parsing = true;
                i += 1;
                continue;
            }

            if !stop_parsing && arg.starts_with("--") && arg.len() > 2 {
                let (name, value_from_arg) = if let Some(eq_pos) = arg.find('=') {
                    let name = &arg[2..eq_pos];
                    let value = &arg[eq_pos + 1..];
                    (name, Some(value))
                } else {
                    let name = &arg[2..];
                    (name, None)
                };

                if let Some(arg_def) = self.arguments.iter().find(|a| a.long == name) {
                    if arg_def.takes_value {
                        if let Some(val) = value_from_arg {
                            map.insert(name.to_owned(), val.to_owned());
                        } else {
                            if i + 1 < args.len() {
                                let next = &args[i + 1];
                                if !next.starts_with('-') || next == "--" {
                                    map.insert(name.to_owned(), next.clone());
                                    i += 1; // consume the value
                                } else {
                                    return Err(format!("option --{name} requires a value, but next argument is an option"));
                                }
                            } else {
                                return Err(format!("option --{name} requires a value, but none provided"));
                            }
                        }
                    } else {
                        if value_from_arg.is_some() {
                            return Err(format!("flag --{name} does not take a value"));
                        }
                        map.insert(name.to_owned(), "true".to_owned());
                    }
                } else {
                    return Err(format!("unknown long option: --{name}"));
                }
                i += 1;
                continue;
            }

            if !stop_parsing && arg.starts_with('-') && arg.len() > 1 {
                let chars: Vec<char> = arg[1..].chars().collect();
                let mut j = 0;
                while j < chars.len() {
                    let c = chars[j];
                    if let Some(arg_def) = self.arguments.iter().find(|a| a.short == Some(c)) {
                        if arg_def.takes_value {
                            if j + 1 < chars.len() {
                                let value: String = chars[j + 1..].iter().collect();
                                map.insert(arg_def.long.clone(), value);
                                break;
                            } else {
                                if i + 1 < args.len() {
                                    let next = &args[i + 1];
                                    if !next.starts_with('-') || next == "--" {
                                        map.insert(arg_def.long.clone(), next.clone());
                                        i += 1; // consume the value
                                    } else {
                                        return Err(format!("option -{c} requires a value, but next argument is an option"));
                                    }
                                } else {
                                    return Err(format!("option -{c} requires a value, but none provided"));
                                }
                            }
                        } else {
                            map.insert(arg_def.long.clone(), "true".to_owned());
                        }
                    } else {
                        return Err(format!("unknown short option: -{c}"));
                    }
                    j += 1;
                }
                i += 1;
                continue;
            }

            if !action_set {
                action = arg.clone();
                action_set = true;
            } else {
                nn.push(arg.clone());
            }
            i += 1;
        }

        Ok(ArgResult {
            map,
            nn,
            action,
        })
    }
}