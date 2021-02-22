#[derive(Debug)]
pub struct Config {
    pub input: String,
    pub help: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let mut input = String::new();
        let mut help = false;
        let mut input_set = false;
        let mut first_argument = true;
        for i in args {
            if first_argument {
                first_argument = false;
                continue;
            }
            if !input_set {
                input = i;
                input_set = true;
            } else {
                // Parsing flags
                match i.as_str() {
                    "-h" => help = true,
                    _ => (),
                }
            }
        }
        Self {
            input: input,
            help: help,
        }
    }
}
