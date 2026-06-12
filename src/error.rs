use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PunjabiError {
    Lex {
        line: usize,
        column: usize,
        message: String,
    },
    Parse {
        line: usize,
        column: usize,
        message: String,
    },
    Runtime {
        message: String,
    },
    Usage {
        message: String,
    },
}

impl PunjabiError {
    pub fn lex(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::Lex {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn parse(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::Parse {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        Self::Runtime {
            message: message.into(),
        }
    }

    pub fn usage(message: impl Into<String>) -> Self {
        Self::Usage {
            message: message.into(),
        }
    }
}

impl fmt::Display for PunjabiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PunjabiError::Lex {
                line,
                column,
                message,
            } => write!(
                formatter,
                "Lexer error line {line}, column {column}: {message}\nLexer raw text nu chhotay tokens vich torrda ae."
            ),
            PunjabiError::Parse {
                line,
                column,
                message,
            } => write!(
                formatter,
                "Parser error line {line}, column {column}: {message}\nParser tokens nu program di shape, yani AST, bananda ae."
            ),
            PunjabiError::Runtime { message } => write!(
                formatter,
                "Runtime error: {message}\nInterpreter program chalanda ae; masla run time te aaya."
            ),
            PunjabiError::Usage { message } => write!(formatter, "Usage error: {message}"),
        }
    }
}

impl std::error::Error for PunjabiError {}
