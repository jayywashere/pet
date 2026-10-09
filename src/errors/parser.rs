#[derive(Debug)]
pub enum ParserError {
    MissingCommand,
    UnknownCommand(String), // Unknown command
    MissingFlag(String),    // Expected
    UnknownFlag {
        got: String,      // Unknown flag
        expected: String, // Expected flag
    },
    InvalidFlag {
        got: String,      // Flag value
        expected: String, // Expected flag
    },
    MissingArgument(String), // Flag
    UnknownArgument {
        argument: String, // Unknown argument
        flag: String,     // Flag
    },
    InvalidArgument {
        argument: String, // Invalid argument
        flag: String,     // Flag
        reason: String,   // Reason of invalidation
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCommand => {
                write!(f, "Missing command!")
            }

            Self::UnknownCommand(cmd) => {
                write!(f, "Unknown command: '{cmd}'!")
            }

            Self::MissingFlag(expected) => {
                write!(f, "Missing flag; expected: '{expected}'!")
            }

            Self::UnknownFlag { got, expected } => {
                write!(f, "Got flag '{got}', but expected '{expected}'!")
            }

            Self::InvalidFlag { got, expected } => {
                write!(
                    f,
                    "Got invalid flag '{got}', but expected '{expected}'. Make sure it has the prefix '--'!"
                )
            }

            Self::MissingArgument(flag) => {
                write!(f, "Missing argument for flag: '{flag}'!")
            }

            Self::UnknownArgument { argument, flag } => {
                write!(f, "Unknown argument '{argument}' for flag '{flag}'!")
            }

            Self::InvalidArgument {
                argument,
                flag,
                reason,
            } => {
                write!(
                    f,
                    "Invalid argument '{argument}' for flag '{flag}'. {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ParserError {}
