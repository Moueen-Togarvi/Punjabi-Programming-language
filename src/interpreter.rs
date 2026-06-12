use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::error::PunjabiError;
use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Interpreter {
    variables: HashMap<String, Value>,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, statements: &[Stmt]) -> Result<Vec<String>, PunjabiError> {
        self.output.clear();
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(self.output.clone())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), PunjabiError> {
        match statement {
            Stmt::Print(expression) => {
                let value = self.evaluate(expression)?;
                self.output.push(value.to_string());
                Ok(())
            }
            Stmt::VarDecl { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.variables.insert(name.clone(), value);
                Ok(())
            }
            Stmt::Assign { name, value } => {
                if !self.variables.contains_key(name) {
                    return Err(PunjabiError::runtime(format!(
                        "Variable '{name}' pehlan declare nai hoi. Pehlan 'rakho {name} = ...' likho."
                    )));
                }
                let value = self.evaluate(value)?;
                self.variables.insert(name.clone(), value);
                Ok(())
            }
            Stmt::Block(statements) => {
                for statement in statements {
                    self.execute(statement)?;
                }
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                let mut guard = 0usize;
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                    guard += 1;
                    if guard > 100_000 {
                        return Err(PunjabiError::runtime(
                            "Loop bohat zyada chal reha ae. Condition check karo.",
                        ));
                    }
                }
                Ok(())
            }
        }
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Value, PunjabiError> {
        match expression {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Variable(name) => self.variables.get(name).cloned().ok_or_else(|| {
                PunjabiError::runtime(format!(
                    "Variable '{name}' nai mili. Pehlan 'rakho {name} = ...' naal banao."
                ))
            }),
            Expr::Grouping(expression) => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match operator {
                    UnaryOp::Negate => match right {
                        Value::Number(value) => Ok(Value::Number(-value)),
                        other => Err(PunjabiError::runtime(format!(
                            "'-' sirf number te lag sakda ae, {} te nai.",
                            other.type_name()
                        ))),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!right.is_truthy())),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, *operator, right),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &Expr,
        operator: BinaryOp,
        right: &Expr,
    ) -> Result<Value, PunjabiError> {
        if operator == BinaryOp::And {
            let left = self.evaluate(left)?;
            if !left.is_truthy() {
                return Ok(Value::Bool(false));
            }
            return Ok(Value::Bool(self.evaluate(right)?.is_truthy()));
        }

        if operator == BinaryOp::Or {
            let left = self.evaluate(left)?;
            if left.is_truthy() {
                return Ok(Value::Bool(true));
            }
            return Ok(Value::Bool(self.evaluate(right)?.is_truthy()));
        }

        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator {
            BinaryOp::Add => add_values(left, right),
            BinaryOp::Subtract => number_binary(left, right, "-", |a, b| a - b),
            BinaryOp::Multiply => number_binary(left, right, "*", |a, b| a * b),
            BinaryOp::Divide => {
                let (left, right) = numbers(left, right, "/")?;
                if right == 0.0 {
                    return Err(PunjabiError::runtime("Zero naal divide nai kar sakde."));
                }
                Ok(Value::Number(left / right))
            }
            BinaryOp::Modulo => {
                let (left, right) = numbers(left, right, "%")?;
                if right == 0.0 {
                    return Err(PunjabiError::runtime("Zero naal modulo nai kar sakde."));
                }
                Ok(Value::Number(left % right))
            }
            BinaryOp::Equal => Ok(Value::Bool(left == right)),
            BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOp::Greater => compare_numbers(left, right, ">", |a, b| a > b),
            BinaryOp::GreaterEqual => compare_numbers(left, right, ">=", |a, b| a >= b),
            BinaryOp::Less => compare_numbers(left, right, "<", |a, b| a < b),
            BinaryOp::LessEqual => compare_numbers(left, right, "<=", |a, b| a <= b),
            BinaryOp::And | BinaryOp::Or => unreachable!("logical operators are handled early"),
        }
    }
}

fn add_values(left: Value, right: Value) -> Result<Value, PunjabiError> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
        (Value::Str(left), Value::Str(right)) => Ok(Value::Str(format!("{left}{right}"))),
        (Value::Str(left), right) => Ok(Value::Str(format!("{left}{right}"))),
        (left, Value::Str(right)) => Ok(Value::Str(format!("{left}{right}"))),
        (left, right) => Err(PunjabiError::runtime(format!(
            "'+' number + number ya string jorran layi ae, {} + {} nai.",
            left.type_name(),
            right.type_name()
        ))),
    }
}

fn number_binary(
    left: Value,
    right: Value,
    operator: &str,
    operation: fn(f64, f64) -> f64,
) -> Result<Value, PunjabiError> {
    let (left, right) = numbers(left, right, operator)?;
    Ok(Value::Number(operation(left, right)))
}

fn compare_numbers(
    left: Value,
    right: Value,
    operator: &str,
    operation: fn(f64, f64) -> bool,
) -> Result<Value, PunjabiError> {
    let (left, right) = numbers(left, right, operator)?;
    Ok(Value::Bool(operation(left, right)))
}

fn numbers(left: Value, right: Value, operator: &str) -> Result<(f64, f64), PunjabiError> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok((left, right)),
        (left, right) => Err(PunjabiError::runtime(format!(
            "'{operator}' sirf numbers te lag sakda ae, {} te {} te nai.",
            left.type_name(),
            right.type_name()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::run_source;

    #[test]
    fn runs_variables_math_and_print() {
        let output = run_source("rakho x = 2 + 3 * 4\nlikho x").unwrap();
        assert_eq!(output, vec!["14".to_string()]);
    }

    #[test]
    fn runs_if_else_and_loop() {
        let source = r#"
rakho x = 0
jadd_tak x < 3 {
  x = x + 1
}
je x == 3 {
  likho "theek"
} nai_ta {
  likho "galat"
}
"#;
        let output = run_source(source).unwrap();
        assert_eq!(output, vec!["theek".to_string()]);
    }
}
