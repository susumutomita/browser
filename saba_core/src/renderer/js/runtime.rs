use crate::renderer::js::ast::Node;
use crate::renderer::js::ast::Program;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::cell::RefCell;
use core::ops::{Add, Sub};

type VariableMap = Vec<(String, Option<RuntimeValue>)>;

/// https://262.ecma-international.org/#sec-environment-records
#[derive(Debug, Clone)]
pub struct Environment {
    variables: VariableMap,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            variables: VariableMap::new(),
            outer,
        }
    }
    pub fn get_variable(&self, name: String) -> Option<RuntimeValue> {
        for variable in &self.variables {
            if variable.0 == name {
                return variable.1.clone(); // (d1)
            }
        }
        if let Some(env) = &self.outer {
            env.borrow_mut().get_variable(name) // (d2)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
#[warn(dead_code)]
pub struct JsRuntime {
    env: Rc<RefCell<Environment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(u64),
}

impl Add<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;
    fn add(self, rhs: RuntimeValue) -> RuntimeValue {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (self, rhs);
        // 不要な return を削除
        RuntimeValue::Number(left_num + right_num)
    }
}

impl Sub<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;
    fn sub(self, rhs: RuntimeValue) -> RuntimeValue {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (self, rhs);
        RuntimeValue::Number(left_num - right_num)
    }
}

impl JsRuntime {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    pub fn execute(&mut self, program: &Program) {
        for node in program.body() {
            // node を clone して Some(...) で包む
            self.eval(&Some(node.clone()));
        }
    }

    // 再帰専用の引数に対する警告を抑制したい場合は、関数定義に以下のアトリビュートを付与:
    #[allow(clippy::only_used_in_recursion)]
    fn eval(&mut self, node: &Option<Rc<Node>>) -> Option<RuntimeValue> {
        // `match Some(n) => n, None => return None` を `?` にする
        let node = node.as_ref()?;

        match node.borrow() {
            // 不要な return を削除 & 不要な借用を削除
            Node::ExpressionStatement(expr) => self.eval(expr),

            Node::AdditiveExpression {
                operator,
                left,
                right,
            } => {
                // match ... { Some(value)=> value, None => return None } を ? 演算子に
                let left_value = self.eval(left)?;
                let right_value = self.eval(right)?;
                if operator == &'+' {
                    Some(left_value + right_value)
                } else if operator == &'-' {
                    Some(left_value - right_value)
                } else {
                    None
                }
            }

            Node::AssignmentExpression { .. } => None,
            Node::MemberExpression { .. } => None,
            Node::NumericLiteral(value) => Some(RuntimeValue::Number(*value)),

            // 上記で扱っていないバリアントをまとめて無視するならこう書く：
            _ => None,
        }
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;
    use crate::renderer::js::ast::JsParser;
    use crate::renderer::js::token::JsLexer;

    #[test]
    fn test_num() {
        let input = "42".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(42))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }
    #[test]
    fn test_add_nums() {
        let input = "1 + 2".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(3))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }

    #[test]
    fn test_sub_nums() {
        let input = "2 - 1".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(1))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }
}
