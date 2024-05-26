use crate::expr::{BinaryExpr, Expr, GroupingExpr, IdentifierExpr, IntLiteralExpr, RealLiteralExpr, UnaryExpr};
use crate::results::ArcResult;
use crate::lexer::{Loc, Token, TokenKind};

pub struct Parser<'a> {
    tokens: &'a [Token],
    start_loc: usize,
    current: usize,
    nodes: Vec<Expr>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, start_loc: 0, current: 0, nodes: vec![] }
    }

    pub fn parse(&mut self) -> Result<&Vec<Expr>, Vec<ArcResult>> {
        let mut errors: Vec<ArcResult> = vec![];

        while !self.eof() {
            // If we have a new line to begin a statement/expr parsing,
            // we skip it. There are important only in parsing steps
            while !self.eof() && self.is_at(TokenKind::NewLine) {
                self.current += 1;
            }
            
            // We could have reached EOF while skipping new lines
            if self.eof() { break }

            self.start_loc = self.at().loc.start;

            match self.parse_expr() {
                Ok(expr) => self.nodes.push(expr),
                Err(e) => { errors.push(e) }
            }
        }
        
        if !errors.is_empty() {
            return Err(errors)
        }

        Ok(&self.nodes)
}

    fn parse_expr(&mut self) -> Result<Expr, ArcResult> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ArcResult> {
        let mut expr = self.parse_comparison()?;

        while self.is_at(TokenKind::EqualEqual) || self.is_at(TokenKind::BangEqual) {
            let operator = self.eat()?.value.clone();
            let right = self.parse_comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                loc: self.get_loc(),
            });
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ArcResult> {
        let mut expr = self.parse_term()?;

        while self.is_at(TokenKind::Less)
                || self.is_at(TokenKind::LessEqual)
                || self.is_at(TokenKind::Greater)
                || self.is_at(TokenKind::GreaterEqual)
        {
            let operator = self.eat()?.value.clone();
            let right = self.parse_term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                loc: self.get_loc(),
            });
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ArcResult> {
        let mut expr = self.parse_factor()?;

        while self.is_at(TokenKind::Minus) || self.is_at(TokenKind::Plus) {
            let operator = self.eat()?.value.clone();
            let right = self.parse_factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                loc: self.get_loc(),
            });
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ArcResult> {
        let mut expr = self.parse_unary()?;

        while self.is_at(TokenKind::Star) || self.is_at(TokenKind::Slash) {
            let operator = self.eat()?.value.clone();
            let right = self.parse_unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                loc: self.get_loc(),
            });
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ArcResult> {
        if self.is_at(TokenKind::Bang) || self.is_at(TokenKind::Minus) {
            let operator = self.eat()?.value.clone();
            let right = self.parse_primary()?;

            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
                loc: self.get_loc(),
            }))
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ArcResult> {
        match self.eat()?.kind {
            TokenKind::Identifier 
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Null => Ok(Expr::Identifier(
                IdentifierExpr {
                    name: self.prev().value.clone(),
                    loc: self.get_loc(),
                })
            ),
            TokenKind::Int => self.parse_int_literal(),
            TokenKind::Real => self.parse_real_literal(),
            TokenKind::OpenParen => self.parse_grouping(),
            TokenKind::NewLine => { Err(self.trigger_error("Unexpected end of line".into(), false)) },
            _ => Err(self.trigger_error(format!("Unknown token to parse: '{}'", self.prev()), true))
        }
    }

    fn parse_int_literal(&self) -> Result<Expr, ArcResult> {
        let tk = self.prev();
        let value = tk.value.parse::<i64>().map_err(|_| ArcResult::internal_error("Error parsing int from string".into()))?;

        Ok(Expr::IntLiteral(IntLiteralExpr { value, loc: self.get_loc() }))
    }

    fn parse_real_literal(&self) -> Result<Expr, ArcResult> {
        let tk = self.prev();
        let value = tk.value.parse::<f64>().map_err(|_| ArcResult::internal_error("Error parsing real from string".into()))?;

        Ok(Expr::RealLiteral(RealLiteralExpr { value, loc: self.get_loc() }))
    }

    fn parse_grouping(&mut self) -> Result<Expr, ArcResult> {
        let expr = self.parse_expr()?;
        println!("\nGrouping end, we are at: {:?}\n", self.at());
        self.expect(TokenKind::CloseParen)?;

        Ok(Expr::Grouping(GroupingExpr { expr: Box::new(expr), loc: self.get_loc() }))
    }

    fn at(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn eat(&mut self) -> Result<&Token, ArcResult> {
        if self.eof() {
            return Err(ArcResult::internal_error("Token access out of bound".into()))
        }

        self.current += 1;
        Ok(self.prev())
    }
    
    fn expect(&mut self, kind: TokenKind) -> Result<(), ArcResult> {
        let tk = self.eat()?;

        match tk.kind == kind {
            true => Ok(()),
            false => {
                let msg = format!("Expected token type '{:?}', found: {:?}", kind, tk.kind);
                Err(self.trigger_error(msg, true))
            }
        }
    }

    fn is_at(&self, kind: TokenKind) -> bool {
        self.at().kind == kind
    }

    fn prev(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
    
    fn eof(&self) -> bool {
        self.is_at(TokenKind::Eof)
    }

    // We dont have to activate the synchro each time, if the error occured
    // because we ate a '\n' that wasn't supposed to be here, we are already
    // past the error, we are on the new line. No need to synchronize
    fn trigger_error(&mut self, msg: String, synchro: bool) -> ArcResult {
        if synchro {
            self.synchronize();
        }
        
        ArcResult::parser_error(msg, self.get_loc())
    }

    // TODO: For now, we are only looking for new line token as we
    // don't have ';' to clearly know where the current statement stops.
    // It would be great to have an argument to this function that let
    // us know where we were when we got the error to know which corresponding
    // token to look for.

    // We are here in panic mode
    fn synchronize(&mut self) {
        // We parse potential other errors in statements
        while !self.eof() {
            match self.at().kind {
                TokenKind::NewLine
                | TokenKind::Struct
                | TokenKind::Fn
                | TokenKind::Var
                | TokenKind::Const
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => { let _ = self.eat(); }
            }
        }
    }

    fn get_loc(&self) -> Loc {
        Loc::new(self.start_loc, self.at().loc.start - 1)
    }
}

#[cfg(test)]
mod tests {
    use ecow::EcoString;

    use crate::lexer::{Lexer, Loc};
    use super::Parser;
    use crate::expr::*;

    #[test]
    fn parse_primary() {
        let code:String = "12 24. 54.678 (true) ( (null ))".into();
        let mut lexer = Lexer::new(code.as_str());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let nodes = parser.parse().unwrap();

        assert_eq!(
            nodes,
            &vec![
                Expr::IntLiteral(IntLiteralExpr {
                    value: 12,
                    loc: Loc::new(0, 2),
                }),
                Expr::RealLiteral(RealLiteralExpr {
                    value: 24.,
                    loc: Loc::new(4, 6),
                }),
                Expr::RealLiteral(RealLiteralExpr {
                    value: 54.678,
                    loc: Loc::new(8, 13),
                }),
                Expr::Grouping(GroupingExpr {
                    expr: Box::new(Expr::Identifier(IdentifierExpr {
                        name: EcoString::from("true"),
                        loc: Loc::new(16, 19),
                    })),
                    loc: Loc::new(15, 20),
                }),
                Expr::Grouping(GroupingExpr {
                    expr: Box::new(
                        Expr::Grouping(GroupingExpr {
                            expr: Box::new(Expr::Identifier(IdentifierExpr {
                                name: EcoString::from("null"),
                                loc: Loc::new(25, 28),
                            })),
                            loc: Loc::new(24, 30)
                        })
                    ),
                    loc: Loc::new(22, 31),
                }),
            ]
        )
    }

    #[test]
    fn parse_unary() {
        let code:String = "-12 -24. -54.67 !true".into();
        let mut lexer = Lexer::new(code.as_str());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let nodes = parser.parse().unwrap();

        assert_eq!(
            nodes,
            &vec![
                Expr::Unary(UnaryExpr {
                    operator: EcoString::from("-"),
                    right: Box::new(Expr::IntLiteral(IntLiteralExpr {
                        value: 12,
                        loc: Loc::new(0, 1),
                    })),
                    loc: Loc::new(0, 2),
                }),
                Expr::Unary(UnaryExpr {
                    operator: EcoString::from("-"),
                    right: Box::new(Expr::RealLiteral(RealLiteralExpr {
                        value: 24.,
                        loc: Loc::new(5, 7),
                    })),
                    loc: Loc::new(4, 7),
                }),
                Expr::Unary(UnaryExpr {
                    operator: EcoString::from("-"),
                    right: Box::new(Expr::RealLiteral(RealLiteralExpr {
                        value: 54.67,
                        loc: Loc::new(10, 14),
                    })),
                    loc: Loc::new(9, 14),
                }),
                Expr::Unary(UnaryExpr {
                    operator: EcoString::from("!"),
                    right: Box::new(Expr::Identifier(IdentifierExpr {
                        name: EcoString::from("true"),
                        loc: Loc::new(17, 20),
                    })),
                    loc: Loc::new(16, 20),
                }),
            ]
        )
    }
}