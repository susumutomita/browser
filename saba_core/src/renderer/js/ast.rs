use crate::renderer::js::token::JsLexer;
use crate::renderer::js::token::Token;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    ExpressionStatement(Option<Rc<Node>>),
    AdditiveExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    AssignmentExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    MemberExpression {
        object: Option<Rc<Node>>,
        property: Option<Rc<Node>>,
    },
    NumericLiteral(u64),
    VariableDeclaration {
        declarations: Vec<Option<Rc<Node>>>,
    },
    VariableDeclarator {
        id: Option<Rc<Node>>,
        init: Option<Rc<Node>>,
    },
    Identifier(String),
    StringLiteral(String),
    BlockStatement {
        body: Vec<Option<Rc<Node>>>,
    },
    ReturnStatement {
        argument: Option<Rc<Node>>,
    },
    FunctionDeclaration {
        id: Option<Rc<Node>>,
        params: Vec<Option<Rc<Node>>>,
        body: Option<Rc<Node>>,
    },
    CallExpression {
        callee: Option<Rc<Node>>,
        arguments: Vec<Option<Rc<Node>>>,
    },
}

impl Node {
    pub fn new_expression_statement(expression: Option<Rc<Self>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::ExpressionStatement(expression)))
    }

    pub fn new_additive_expression(
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::AdditiveExpression {
            operator,
            left,
            right,
        }))
    }

    pub fn new_assignment_expression(
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::AssignmentExpression {
            operator,
            left,
            right,
        }))
    }

    pub fn new_member_expression(
        object: Option<Rc<Self>>,
        property: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::MemberExpression { object, property }))
    }

    pub fn new_numeric_literal(value: u64) -> Option<Rc<Self>> {
        Some(Rc::new(Node::NumericLiteral(value)))
    }

    pub fn new_variable_declarator(
        id: Option<Rc<Self>>,
        init: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::VariableDeclarator { id, init }))
    }

    pub fn new_variable_declaration(declarations: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::VariableDeclaration { declarations }))
    }

    pub fn new_identifier(name: String) -> Option<Rc<Self>> {
        Some(Rc::new(Node::Identifier(name)))
    }

    pub fn new_string_literal(value: String) -> Option<Rc<Self>> {
        Some(Rc::new(Node::StringLiteral(value)))
    }

    pub fn new_block_statement(body: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::BlockStatement { body }))
    }

    pub fn new_return_statement(argument: Option<Rc<Self>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::ReturnStatement { argument }))
    }

    pub fn new_function_declaration(
        id: Option<Rc<Self>>,
        params: Vec<Option<Rc<Self>>>,
        body: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::FunctionDeclaration { id, params, body }))
    }

    pub fn new_call_expression(
        callee: Option<Rc<Self>>,
        arguments: Vec<Option<Rc<Self>>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::CallExpression { callee, arguments }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    body: Vec<Rc<Node>>,
}

impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn set_body(&mut self, body: Vec<Rc<Node>>) {
        self.body = body;
    }

    pub fn body(&self) -> &Vec<Rc<Node>> {
        &self.body
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

pub struct JsParser {
    t: Peekable<JsLexer>,
}

#[allow(clippy::single_match)]
#[allow(clippy::redundant_guards)]
impl JsParser {
    pub fn new(t: JsLexer) -> Self {
        Self { t: t.peekable() }
    }

    fn primary_expression(&mut self) -> Option<Rc<Node>> {
        // Clippy: replace match with ?
        let t = self.t.next()?;

        match t {
            Token::Identifier(value) => Node::new_identifier(value),
            Token::StringLiteral(value) => Node::new_string_literal(value),
            Token::Number(value) => Node::new_numeric_literal(value),
            _ => None,
        }
    }

    fn member_expression(&mut self) -> Option<Rc<Node>> {
        let expr = self.primary_expression();

        // Clippy: replace match with ?
        let t = match self.t.peek() {
            Some(token) => token,
            None => return expr,
        };

        match t {
            Token::Punctuator(c) if c == &'.' => {
                // '.'を消費する
                let _ = self.t.next()?;
                Node::new_member_expression(expr, self.identifier())
            }
            _ => expr,
        }
    }

    fn arguments(&mut self) -> Vec<Option<Rc<Node>>> {
        let mut arguments = Vec::new();

        loop {
            match self.t.peek() {
                Some(Token::Punctuator(c)) if c == &')' => {
                    // ')'を消費
                    let _ = self.t.next();
                    return arguments;
                }
                Some(Token::Punctuator(c)) if c == &',' => {
                    // ','を消費
                    let _ = self.t.next();
                }
                Some(_) => {
                    arguments.push(self.assignment_expression());
                }
                None => {
                    return arguments;
                }
            }
        }
    }

    fn left_hand_side_expression(&mut self) -> Option<Rc<Node>> {
        let expr = self.member_expression();

        match self.t.peek() {
            Some(Token::Punctuator(c)) if c == &'(' => {
                // '('を消費
                let _ = self.t.next()?;
                // 関数呼び出しとして CallExpression ノードへ
                Node::new_call_expression(expr, self.arguments())
            }
            _ => expr,
        }
    }

    fn additive_expression(&mut self) -> Option<Rc<Node>> {
        let left = self.left_hand_side_expression();

        let t = match self.t.peek() {
            Some(token) => token.clone(),
            None => return left,
        };

        match t {
            Token::Punctuator(c @ ('+' | '-')) => {
                // '+' または '-' を消費
                let _ = self.t.next()?;
                Node::new_additive_expression(c, left, self.assignment_expression())
            }
            _ => left,
        }
    }

    fn assignment_expression(&mut self) -> Option<Rc<Node>> {
        let expr = self.additive_expression();

        match self.t.peek() {
            Some(Token::Punctuator('=')) => {
                // '='を消費
                let _ = self.t.next()?;
                Node::new_assignment_expression('=', expr, self.assignment_expression())
            }
            _ => expr,
        }
    }

    fn initialiser(&mut self) -> Option<Rc<Node>> {
        // Clippy: replace match with ?
        let t = self.t.next()?;

        // collapsible_match → Token::Punctuator('=') / Token::Punctuator(_)
        match t {
            Token::Punctuator('=') => self.assignment_expression(),
            Token::Punctuator(_) => None,
            _ => None,
        }
    }

    fn identifier(&mut self) -> Option<Rc<Node>> {
        let t = self.t.next()?;

        match t {
            Token::Identifier(name) => Node::new_identifier(name),
            _ => None,
        }
    }

    fn variable_declaration(&mut self) -> Option<Rc<Node>> {
        let ident = self.identifier();
        let declarator = Node::new_variable_declarator(ident, self.initialiser());

        // Clippy: use vec![...] instead of creating then push
        let declarations = vec![declarator];

        Node::new_variable_declaration(declarations)
    }

    fn statement(&mut self) -> Option<Rc<Node>> {
        // Clippy: replace match with ?
        let t = self.t.peek()?;

        let node = match t {
            Token::Keyword(keyword) if keyword == "var" => {
                let _ = self.t.next()?; // "var" 消費
                self.variable_declaration()
            }
            Token::Keyword(keyword) if keyword == "return" => {
                let _ = self.t.next()?; // "return" 消費
                Node::new_return_statement(self.assignment_expression())
            }
            _ => Node::new_expression_statement(self.assignment_expression()),
        };

        // セミコロンがあれば消費
        if let Some(Token::Punctuator(c)) = self.t.peek() {
            if c == &';' {
                let _ = self.t.next();
            }
        }

        node
    }

    fn function_body(&mut self) -> Option<Rc<Node>> {
        // '{' を消費
        let open_brace = self.t.next()?;
        match open_brace {
            Token::Punctuator(c) if c == '{' => {}
            _ => unimplemented!(
                "function should have open curly bracket but got {:?}",
                open_brace
            ),
        }

        let mut body = Vec::new();
        loop {
            match self.t.peek() {
                Some(Token::Punctuator(c)) if c == &'}' => {
                    // '}'を消費してBlockStatementを返す
                    let _ = self.t.next();
                    return Node::new_block_statement(body);
                }
                Some(_) => {
                    body.push(self.source_element());
                }
                None => {
                    // 実装の都合上 None でも抜ける
                    return Node::new_block_statement(body);
                }
            }
        }
    }

    fn parameter_list(&mut self) -> Vec<Option<Rc<Node>>> {
        let mut params = Vec::new();

        // '(' を取得
        // None ならパラメータ一覧が始まらなかったとみなして空のまま返す
        let open_paren = match self.t.next() {
            Some(tok) => tok,
            None => {
                // 必要に応じてここで panic! する or 空を返すなど
                return params;
            }
        };

        match open_paren {
            Token::Punctuator('(') => {
                // OK: 関数のパラメータリスト開始
            }
            _ => {
                unimplemented!("function should have '(' but got {:?}", open_paren);
            }
        }

        loop {
            match self.t.peek() {
                Some(Token::Punctuator(c)) if c == &')' => {
                    let _ = self.t.next(); // ')' を消費
                    return params;
                }
                Some(Token::Punctuator(c)) if c == &',' => {
                    let _ = self.t.next(); // ',' を消費
                }
                Some(_) => {
                    params.push(self.identifier());
                }
                None => {
                    return params;
                }
            }
        }
    }

    fn function_declaration(&mut self) -> Option<Rc<Node>> {
        let id = self.identifier();
        let params = self.parameter_list();
        Node::new_function_declaration(id, params, self.function_body())
    }

    fn source_element(&mut self) -> Option<Rc<Node>> {
        // Clippy: replace match with ?
        let t = self.t.peek()?;

        if let Token::Keyword(keyword) = t {
            if keyword == "function" {
                let _ = self.t.next()?; // "function" 消費
                return self.function_declaration();
            }
        }
        // それ以外は statement
        self.statement()
    }

    pub fn parse_ast(&mut self) -> Program {
        let mut program = Program::new();
        let mut body = Vec::new();

        loop {
            match self.source_element() {
                Some(n) => body.push(n),
                None => {
                    program.set_body(body);
                    return program;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_empty() {
        let input = "".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let expected = Program::new();
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_num() {
        let input = "42".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::ExpressionStatement(Some(Rc::new(
            Node::NumericLiteral(42),
        )))));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_add_nums() {
        let input = "1 + 2".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::ExpressionStatement(Some(Rc::new(
            Node::AdditiveExpression {
                operator: '+',
                left: Some(Rc::new(Node::NumericLiteral(1))),
                right: Some(Rc::new(Node::NumericLiteral(2))),
            },
        )))));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_assign_variable() {
        let input = "var foo=\"bar\";".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::VariableDeclaration {
            declarations: [Some(Rc::new(Node::VariableDeclarator {
                id: Some(Rc::new(Node::Identifier("foo".to_string()))),
                init: Some(Rc::new(Node::StringLiteral("bar".to_string()))),
            }))]
            .to_vec(),
        }));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_add_variable_and_num() {
        let input = "var foo=42; var result=foo+1;".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::VariableDeclaration {
            declarations: [Some(Rc::new(Node::VariableDeclarator {
                id: Some(Rc::new(Node::Identifier("foo".to_string()))),
                init: Some(Rc::new(Node::NumericLiteral(42))),
            }))]
            .to_vec(),
        }));
        body.push(Rc::new(Node::VariableDeclaration {
            declarations: [Some(Rc::new(Node::VariableDeclarator {
                id: Some(Rc::new(Node::Identifier("result".to_string()))),
                init: Some(Rc::new(Node::AdditiveExpression {
                    operator: '+',
                    left: Some(Rc::new(Node::Identifier("foo".to_string()))),
                    right: Some(Rc::new(Node::NumericLiteral(1))),
                })),
            }))]
            .to_vec(),
        }));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_define_function() {
        let input = "function foo() { return 42; }".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::FunctionDeclaration {
            id: Some(Rc::new(Node::Identifier("foo".to_string()))),
            params: [].to_vec(),
            body: Some(Rc::new(Node::BlockStatement {
                body: [Some(Rc::new(Node::ReturnStatement {
                    argument: Some(Rc::new(Node::NumericLiteral(42))),
                }))]
                .to_vec(),
            })),
        }));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_add_function_add_num() {
        let input = "function foo() { return 42; } var result = foo() + 1;".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::FunctionDeclaration {
            id: Some(Rc::new(Node::Identifier("foo".to_string()))),
            params: [].to_vec(),
            body: Some(Rc::new(Node::BlockStatement {
                body: [Some(Rc::new(Node::ReturnStatement {
                    argument: Some(Rc::new(Node::NumericLiteral(42))),
                }))]
                .to_vec(),
            })),
        }));
        body.push(Rc::new(Node::VariableDeclaration {
            declarations: [Some(Rc::new(Node::VariableDeclarator {
                id: Some(Rc::new(Node::Identifier("result".to_string()))),
                init: Some(Rc::new(Node::AdditiveExpression {
                    operator: '+',
                    left: Some(Rc::new(Node::CallExpression {
                        callee: Some(Rc::new(Node::Identifier("foo".to_string()))),
                        arguments: [].to_vec(),
                    })),
                    right: Some(Rc::new(Node::NumericLiteral(1))),
                })),
            }))]
            .to_vec(),
        }));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_define_function_with_args() {
        let input = "function foo(a, b) { return a+b; }".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::FunctionDeclaration {
            id: Some(Rc::new(Node::Identifier("foo".to_string()))),
            params: [
                Some(Rc::new(Node::Identifier("a".to_string()))),
                Some(Rc::new(Node::Identifier("b".to_string()))),
            ]
            .to_vec(),
            body: Some(Rc::new(Node::BlockStatement {
                body: [Some(Rc::new(Node::ReturnStatement {
                    argument: Some(Rc::new(Node::AdditiveExpression {
                        operator: '+',
                        left: Some(Rc::new(Node::Identifier("a".to_string()))),
                        right: Some(Rc::new(Node::Identifier("b".to_string()))),
                    })),
                }))]
                .to_vec(),
            })),
        }));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }
}
