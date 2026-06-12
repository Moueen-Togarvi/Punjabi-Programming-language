use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::error::PunjabiError;
use crate::token::{Token, TokenKind};
use crate::value::Value;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> Result<Vec<Stmt>, PunjabiError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            self.skip_separators();
            if self.is_at_end() {
                break;
            }
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, PunjabiError> {
        if self.match_kind(&TokenKind::Rakho) {
            return self.var_declaration();
        }
        if self.match_kind(&TokenKind::Likho) {
            return self.print_statement();
        }
        if self.match_kind(&TokenKind::Je) {
            return self.if_statement();
        }
        if self.match_kind(&TokenKind::JadTak) {
            return self.while_statement();
        }
        if self.match_kind(&TokenKind::LeftBrace) {
            return Ok(Stmt::Block(self.block()?));
        }
        if let Some(name) = self.match_identifier_assignment() {
            self.advance();
            return self.assignment(name);
        }

        let token = self.peek();
        Err(PunjabiError::parse(
            token.line,
            token.column,
            "Statement etho start nai ho sakdi. 'rakho', 'likho', 'je', ya 'jadd_tak' use karo.",
        ))
    }

    fn var_declaration(&mut self) -> Result<Stmt, PunjabiError> {
        let name = self.consume_identifier("Variable name chahida ae, example: rakho x = 5")?;
        self.consume(
            &TokenKind::Equal,
            "'=' chahida ae. Example: rakho x = 5",
        )?;
        let initializer = self.expression()?;
        self.consume_optional_separator();
        Ok(Stmt::VarDecl { name, initializer })
    }

    fn print_statement(&mut self) -> Result<Stmt, PunjabiError> {
        let expression = self.expression()?;
        self.consume_optional_separator();
        Ok(Stmt::Print(expression))
    }

    fn if_statement(&mut self) -> Result<Stmt, PunjabiError> {
        let condition = self.expression()?;
        let then_branch = Box::new(self.statement()?);

        self.skip_separators();
        let else_branch = if self.match_kind(&TokenKind::NahiTa) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, PunjabiError> {
        let condition = self.expression()?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn assignment(&mut self, name: String) -> Result<Stmt, PunjabiError> {
        self.consume(&TokenKind::Equal, "'=' ke baad value likho.")?;
        let value = self.expression()?;
        self.consume_optional_separator();
        Ok(Stmt::Assign { name, value })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, PunjabiError> {
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_separators();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            statements.push(self.statement()?);
        }

        self.consume(
            &TokenKind::RightBrace,
            "Block close karan layi '}' chahida ae.",
        )?;
        self.consume_optional_separator();
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, PunjabiError> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr, PunjabiError> {
        let mut expr = self.and()?;

        while self.match_kind(&TokenKind::Ya) {
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, PunjabiError> {
        let mut expr = self.equality()?;

        while self.match_kind(&TokenKind::Te) {
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, PunjabiError> {
        let mut expr = self.comparison()?;

        loop {
            let operator = if self.match_kind(&TokenKind::EqualEqual) {
                Some(BinaryOp::Equal)
            } else if self.match_kind(&TokenKind::BangEqual) {
                Some(BinaryOp::NotEqual)
            } else {
                None
            };

            if let Some(operator) = operator {
                let right = self.comparison()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, PunjabiError> {
        let mut expr = self.term()?;

        loop {
            let operator = if self.match_kind(&TokenKind::Greater) {
                Some(BinaryOp::Greater)
            } else if self.match_kind(&TokenKind::GreaterEqual) {
                Some(BinaryOp::GreaterEqual)
            } else if self.match_kind(&TokenKind::Less) {
                Some(BinaryOp::Less)
            } else if self.match_kind(&TokenKind::LessEqual) {
                Some(BinaryOp::LessEqual)
            } else {
                None
            };

            if let Some(operator) = operator {
                let right = self.term()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, PunjabiError> {
        let mut expr = self.factor()?;

        loop {
            let operator = if self.match_kind(&TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.match_kind(&TokenKind::Minus) {
                Some(BinaryOp::Subtract)
            } else {
                None
            };

            if let Some(operator) = operator {
                let right = self.factor()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, PunjabiError> {
        let mut expr = self.unary()?;

        loop {
            let operator = if self.match_kind(&TokenKind::Star) {
                Some(BinaryOp::Multiply)
            } else if self.match_kind(&TokenKind::Slash) {
                Some(BinaryOp::Divide)
            } else if self.match_kind(&TokenKind::Percent) {
                Some(BinaryOp::Modulo)
            } else {
                None
            };

            if let Some(operator) = operator {
                let right = self.unary()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, PunjabiError> {
        if self.match_kind(&TokenKind::Minus) {
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Negate,
                right: Box::new(right),
            });
        }

        if self.match_kind(&TokenKind::Nahi) {
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Not,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, PunjabiError> {
        if self.match_kind(&TokenKind::Sach) {
            return Ok(Expr::Literal(Value::Bool(true)));
        }
        if self.match_kind(&TokenKind::Jhooth) {
            return Ok(Expr::Literal(Value::Bool(false)));
        }

        match self.advance().kind.clone() {
            TokenKind::Number(value) => Ok(Expr::Literal(Value::Number(value))),
            TokenKind::String(value) => Ok(Expr::Literal(Value::Str(value))),
            TokenKind::Identifier(name) => Ok(Expr::Variable(name)),
            TokenKind::LeftParen => {
                let expression = self.expression()?;
                self.consume(
                    &TokenKind::RightParen,
                    "Grouping close karan layi ')' chahida ae.",
                )?;
                Ok(Expr::Grouping(Box::new(expression)))
            }
            _ => {
                let token = self.previous();
                Err(PunjabiError::parse(
                    token.line,
                    token.column,
                    "Expression chahidi si. Number, string, variable, ya '(' use karo.",
                ))
            }
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, PunjabiError> {
        match self.advance().kind.clone() {
            TokenKind::Identifier(name) => Ok(name),
            _ => {
                let token = self.previous();
                Err(PunjabiError::parse(token.line, token.column, message))
            }
        }
    }

    fn consume(&mut self, expected: &TokenKind, message: &str) -> Result<(), PunjabiError> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            let token = self.peek();
            Err(PunjabiError::parse(token.line, token.column, message))
        }
    }

    fn consume_optional_separator(&mut self) {
        if self.match_kind(&TokenKind::Semicolon) {
            self.skip_separators();
        } else if self.match_kind(&TokenKind::Newline) {
            self.skip_separators();
        }
    }

    fn skip_separators(&mut self) {
        while self.match_kind(&TokenKind::Newline) || self.match_kind(&TokenKind::Semicolon) {}
    }

    fn match_identifier_assignment(&self) -> Option<String> {
        let current = self.tokens.get(self.current)?;
        let next = self.tokens.get(self.current + 1)?;

        match (&current.kind, &next.kind) {
            (TokenKind::Identifier(name), TokenKind::Equal) => Some(name.clone()),
            _ => None,
        }
    }

    fn match_kind(&mut self, expected: &TokenKind) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, expected: &TokenKind) -> bool {
        if self.is_at_end() {
            return matches!(expected, TokenKind::Eof);
        }

        token_kind_matches(&self.peek().kind, expected)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

fn token_kind_matches(actual: &TokenKind, expected: &TokenKind) -> bool {
    std::mem::discriminant(actual) == std::mem::discriminant(expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn parses_print_and_variable_declaration() {
        let tokens = Lexer::new("rakho x = 2\nlikho x").scan_tokens().unwrap();
        let statements = Parser::new(tokens).parse().unwrap();

        assert_eq!(statements.len(), 2);
        assert!(matches!(statements[0], Stmt::VarDecl { .. }));
        assert!(matches!(statements[1], Stmt::Print(_)));
    }
}
