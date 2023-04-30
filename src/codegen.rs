use crate::parser::*;

fn codegen_expr(expr: Expr) -> String {
    let mut str: String = "".to_string();

    match expr {
        Expr::BinOp { op, lhs, rhs } => match op {
            OpType::Add => {
                str.push_str(&format!("{} + {}", codegen_expr(*lhs), codegen_expr(*rhs)))
            }
            _ => todo!("{:#?}", op),
        },

        Expr::Val(x) => match x {
            Value::Number(x) => str.push_str(&format!("{}", x)),
            Value::String(x) => str.push_str(&format!("\"{}\"", x)),
        },

        Expr::Var(name) => str.push_str(&format!("{}", name.replace(' ', "_"))),
    }

    str
}

fn codegen_stmt(statement: Stmt) -> String {
    let mut str: String = "".to_string();

    match statement {
        Stmt::WhenFlagClicked(x) => {
            for stmt in x.stmts {
                str.push_str(&codegen_stmt(stmt).to_string());
            }
        }

        Stmt::SetVariable { name, id: _, val } => {
            str.push_str(&format!(
                "{} = ({});\n",
                name.replace(' ', "_"),
                codegen_expr(val)
            ));
        }

        _ => todo!("{:#?}", statement),
    }

    str
}

pub fn codegen_project(project: Project) -> String {
    let mut str = r#"#include <scratchnative/runtime.hpp>
int main(void)
{
"#
    .to_string();

    str.push_str(&codegen_stmt(project.body));
    str.push_str("\nreturn 0;\n\n}");

    str
}
