use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(value) => *value,
            Value::Null => false,
            Value::Number(value) => *value != 0.0,
            Value::Str(value) => !value.is_empty(),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Str(_) => "string",
            Value::Bool(_) => "boolean",
            Value::Null => "null",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => {
                if value.fract() == 0.0 {
                    write!(formatter, "{value:.0}")
                } else {
                    write!(formatter, "{value}")
                }
            }
            Value::Str(value) => write!(formatter, "{value}"),
            Value::Bool(true) => write!(formatter, "sach"),
            Value::Bool(false) => write!(formatter, "jhooth"),
            Value::Null => write!(formatter, "khali"),
        }
    }
}
