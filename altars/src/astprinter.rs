use crate::ast::Expr;
use crate::ast::Stmt;
use crate::ast::Visitor;
use crate::literals::Literal;

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_stmt(&mut self, s: &Stmt) -> String {
        todo!()
    }

    fn visit_expr(&mut self, e: &Expr) -> String {
        //return "".to_string();
        match &*e {
            Expr::Assign(_, _) => todo!(),
            Expr::Binary(left, op, right) => {
                //return parenthesize(op.lexeme, vec![*left, *right]);
                let exprs = vec![*left.clone(), *right.clone()];
                let text = parenthesize(op.lexeme.clone(), exprs);
                return text;
            },
            Expr::Call(_, _, _) => todo!(),
            Expr::Get(_, _) => todo!(),
            Expr::Grouping(expr) => {
                parenthesize("group".to_string(), vec![*expr.clone()])
            },
            Expr::Literal(expr) => {
                if *expr == Literal::Empty  {
                    return "Empty".to_string();
                } else {
                    return format!("{}", *expr);
                }
            },
            Expr::Logic(_, _, _) => todo!(),
            Expr::Set(_, _, _) => todo!(),
            Expr::This(_) => todo!(),
            Expr::Unary(op, right) => {
                return parenthesize(op.lexeme.clone(), vec![*right.clone()]);
            },
            Expr::Variable(_) => todo!(),
        }
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String {
        self.visit_expr(&expr)
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
