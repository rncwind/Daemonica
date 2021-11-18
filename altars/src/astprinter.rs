use crate::ast::Expr;
use crate::ast::Stmt;
use crate::ast::Visitor;
use crate::literals::Literal;

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_stmt(&mut self, s: &Stmt) -> String {
        match &*s {
            Stmt::Block(stmts) => {
                let mut rv = "".to_string();
                for stmt in stmts {
                    rv = format!("{} {}", rv, self.visit_stmt(stmt));
                }
                rv
            }
            Stmt::Class(_, _) => todo!(),
            Stmt::Expression(_) => todo!(),
            Stmt::Function(_, _, _) => todo!(),
            Stmt::If(_, _, _) => todo!(),
            Stmt::Return(_, _) => todo!(),
            Stmt::Var(_, _) => todo!(),
            Stmt::While(_, _) => todo!(),
            _ => todo!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> String {
        //return "".to_string();
        match &*e {
            Expr::Assign(_, _) => todo!(),
            Expr::Binary(left, op, right) => {
                let l = *left.clone();
                let r = *right.clone();
                let exprs = vec![l, r];
                let text = parenthesize(op.lexeme.clone(), exprs.clone());
                return text;
            }
            Expr::Call(_, _, _) => todo!(),
            Expr::Get(_, _) => todo!(),
            Expr::Grouping(expr) => parenthesize("group".to_string(), vec![*expr.clone()]),
            Expr::Literal(expr) => {
                if *expr == Literal::Empty {
                    return "Empty".to_string();
                } else {
                    return format!("{}", *expr);
                }
            }
            Expr::Logic(_, _, _) => todo!(),
            Expr::Set(_, _, _) => todo!(),
            Expr::This(_) => todo!(),
            Expr::Unary(op, right) => {
                return parenthesize(op.lexeme.clone(), vec![*right.clone()]);
            }
            Expr::Variable(_) => todo!(),
        }
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String {
        let s = self.visit_expr(&expr);
        println!("{}", s);
        s
    }
}

fn parenthesize(name: String, exprs: Vec<Expr>) -> String {
    let mut text = String::from("(");
    text = format!("{} {}", text, name);
    for e in exprs {
        text = format!("{} {} ", text, AstPrinter.visit_expr(&e.clone()));
    }
    text = format!("{})", text);
    return text;
}
