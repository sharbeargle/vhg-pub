/// Specify what type the argument value should be
#[derive(PartialEq, Eq, Debug)]
pub enum ArgType {
    Boolean,
    Character,
    Float,
    Integer,
    String,
}

pub struct FlagConfig {
    name: String,
    shortFlag: char,
    longFlag: String,
    required: bool,
    hasArg: bool,
    argType: Option<ArgType>,
    description: String,
}

pub struct ArgConfig {
    name: String,
    argType: Option<ArgType>,
    required: bool,
    description: String,
}

// TODO: Define and implement how configuration will be stored
pub struct Parser {
    description: String,
}

pub fn new(description: String) -> Parser {
    Parser { description }
}

// TODO: decide how flags syntax
// Should it be -f=<arg>, or -f <arg>, or -f<arg>
// Maybe support all of those cases
impl Parser {
    /// Add a flag to the configuration
    /// A flag with no value is a boolean.
    pub fn add_flag(mut self, flag_config: FlagConfig) -> Self {
        self
    }

    /// Add a positional argument to the configuration
    /// Parsed in order added. Adding required args after unrequired args will have undefined behavior.
    /// arg_type: Not used right now. Just put None.
    pub fn add_arg(mut self, arg_config: ArgConfig) -> Self {
        self
    }

    /// Print the help screen
    pub fn print_help(&self) {}

    /// Parse the command line arguments
    /// Validate configuration.
    pub fn parse(mut self) -> Self {
        // TODO: define how the flags will be Parsed
        let mut args = std::env::args();

        self
    }

    pub fn get_flag_as_boolean(&self, name: &str) -> Option<bool> {
        None
    }

    pub fn get_flag_as_character(&self, name: &str) -> Option<char> {
        None
    }

    pub fn get_flag_as_integer(&self, name: &str) -> Option<i32> {
        None
    }

    pub fn get_flag_as_float(&self, name: &str) -> Option<f32> {
        None
    }

    pub fn get_flag_as_string(&self, name: &str) -> Option<&str> {
        None
    }

    /// Get all positional argument
    pub fn get_arg_as_string(&self, name: &str) -> Option<&Vec<String>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
