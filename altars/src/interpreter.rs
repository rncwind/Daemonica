use crate::ast::Expr;
use crate::ast::Stmt;
use crate::ast::Visitor;

struct Interpreter;
impl<T> Visitor<T> for Interpreter {
    fn visit_stmt(&mut self, x: &Stmt) -> T {
        match &*x {
            Stmt::Block(stmts) => {
                todo!("Stmt::Block")
            }
            Stmt::Class(name, methods) => {
                todo!("Stmt::Class");
            }
            Stmt::Expression(expr) => todo!("Stmt::Expression"),
            Stmt::Function(name, params, body) => todo!(),
            Stmt::If(cond, thenb, elseb) => todo!(),
            Stmt::Return(keyword, value) => todo!(),
            Stmt::Var(name, init) => todo!(),
            Stmt::While(cond, body) => todo!(),
        }
    }

    fn visit_expr(&mut self, x: &Expr) -> T {
        match &*x {
            Expr::Assign(name, value) => todo!(),
            Expr::Binary(left, op, right) => todo!(),
            Expr::Call(callee, paren, args) => todo!(),
            Expr::Get(obj, name) => todo!(),
            Expr::Grouping(expr) => todo!(),
            Expr::Literal(value) => todo!(),
            Expr::Logic(left, op, right) => todo!(),
            Expr::Set(obj, name, value) => todo!(),
            Expr::This(keyword) => todo!(),
            Expr::Unary(op, right) => todo!(),
            Expr::Variable(name) => todo!(),
        }
    }
}
