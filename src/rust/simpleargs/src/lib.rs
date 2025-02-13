
/// Specify what type the flag value should be
#[derive(PartialEq, Eq, Debug)]
pub enum FlagValueType {
    Boolean,
    Character,
    Float,
    Integer,
    String,
}

// TODO: Define and implement how configuration will be stored
pub struct Parser {
    description: String,
}

pub fn new(description: String) -> Parser {

    Parser{description}
}

impl Parser {
    /// Add a flag to the configuration
    /// A flag with no value is a boolean.
    pub fn add_flag(mut self, name: String, short_flag: Option<char>, long_flag: Option<String>, has_value: bool, value_type: Option<FlagValueType>, description: String) -> Self {

        self
    }

    /// Add a positional argument to the configuration
    /// All positional arguments will be parsed as strings.
    pub fn add_positional_args(mut self, name: String, minimum_required: u32, description: String) -> Self {

        self
    }

    /// Print the help screen
    pub fn print_help(&self) {

    }

    /// Parse the command line arguments
    /// Validate configuration.
    pub fn parse(mut self) -> Self {

        self
    }

    pub fn get_boolean_flag(&self, name: &str) -> Option<bool> {
        None
    }

    pub fn get_character_flag(&self, name: &str) -> Option<char> {
        None
    }

    pub fn get_integer_flag(&self, name: &str) -> Option<i32> {
        None
    }

    pub fn get_float_flag(&self, name: &str) -> Option<f32> {
        None
    }

    pub fn get_string_flag(&self, name: &str) -> Option<&str> {
        None
    }

    pub fn get_positional_args(&self) -> Option<&Vec<String>> {

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
