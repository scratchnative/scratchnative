use crate::parser::*;

fn bin_op_to_str(op: OpType) -> String {
    match op {
        OpType::Add => "+",
        OpType::And => "&&",
        OpType::Divide => "/",
        OpType::Equals => "==",
        OpType::GreaterThan => ">",
        OpType::LessThan => "<",
        OpType::Modulo => "%",
        OpType::Multiply => "*",
        OpType::Not => "!",
        OpType::Or => "||",
        OpType::Subtract => "-",
        _ => todo!("{:#?}", op),
    }
    .to_string()
}
fn codegen_expr(expr: Expr) -> String {
    let mut str: String = "".to_string();

    match expr {
        Expr::BinOp { op, lhs, rhs } => match op {
            OpType::Add
            | OpType::Subtract
            | OpType::Multiply
            | OpType::Divide
            | OpType::Modulo
            | OpType::And
            | OpType::GreaterThan
            | OpType::LessThan
            | OpType::Equals
            | OpType::Or => str.push_str(&format!(
                "({}) {} ({})",
                codegen_expr(*lhs),
                bin_op_to_str(op),
                codegen_expr(*rhs)
            )),

            _ => todo!("{:#?}", op),
        },

        Expr::SingleOp { op, expr } => match op {
            OpType::Not => str.push_str(&format!("!({})", codegen_expr(*expr))),
            _ => todo!("{:#?}", op),
        },

        Expr::Val(x) => match x {
            Value::Number(x) => str.push_str(&format!("{}", x)),
            Value::String(x) => str.push_str(&format!("\"{}\"", x)),
        },

        Expr::ItemOf { list_name, index } => str.push_str(&format!(
            "{}[static_cast<int>(({}-1).get<double>())]",
            list_name.replace(' ', "_"),
            codegen_expr(*index)
        )),

        Expr::Var(name) | Expr::Param(name) => str.push_str(&format!("{}", name.replace(' ', "_"))),
    }

    str
}

fn codegen_stmt(statement: Stmt) -> String {
    let mut str: String = "".to_string();

    fn gen_block(block: BlockStmt, str: &mut String) {
        for stmt in block.stmts {
            str.push_str(&codegen_stmt(stmt).to_string())
        }
    }

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

        Stmt::If { condition, block } => {
            str.push_str(&format!("if ({}) {{\n", codegen_expr(condition)));

            gen_block(block, &mut str);

            str.push_str("}\n");
        }

        Stmt::IfElse {
            condition,
            if_block,
            else_block,
        } => {
            str.push_str(&format!("if ({}) {{\n", codegen_expr(condition)));

            gen_block(if_block, &mut str);

            str.push_str("} else {\n");

            gen_block(else_block, &mut str);

            str.push_str("}\n");
        }

        Stmt::Repeat { times, block } => {
            str.push_str(&format!("for (auto _ = {}; _--;){{\n", codegen_expr(times)));

            for stmt in block.stmts {
                str.push_str(&codegen_stmt(stmt).to_string());
            }
            str.push_str("}\n");
        }

        Stmt::RepeatUntil { condition, block } => {
            str.push_str(&format!("while(!({})) {{\n", codegen_expr(condition)));
            for stmt in block.stmts {
                str.push_str(&codegen_stmt(stmt).to_string());
            }
            str.push_str("}\n");
        }

        Stmt::ChangeBy { var_name, inc } => str.push_str(&format!(
            "{} += {};",
            var_name.replace(' ', "_"),
            codegen_expr(inc)
        )),

        Stmt::DeleteAllOfList { name } => {
            str.push_str(&format!("{}.clear();", name.replace(' ', "_")))
        }

        Stmt::AddToList { name, val, .. } => str.push_str(&format!(
            "{}.push_back({});",
            name.replace(' ', "_"),
            codegen_expr(val)
        )),

        Stmt::ProcedureCall { proc, params } => str.push_str(&format!(
            "{}({});",
            proc,
            params
                .iter()
                .map(|x| codegen_expr(x.clone()))
                .collect::<Vec<_>>()
                .join(",")
        )),

        Stmt::ProcedureDefinition { prototype, body } => {
            str.push_str(&format!(
                "auto {} = [&]({}) {{",
                prototype.name,
                prototype
                    .param_order
                    .iter()
                    .map(|x| (format!("ScratchValue {}", x)))
                    .collect::<Vec<_>>()
                    .join(","),
            ));

            gen_block(body, &mut str);

            str.push_str("};");
        }

        _ => todo!("{:#?}", statement),
    }

    str
}

pub fn codegen_project(project: Project) -> String {
    let mut str = r#"#include <runtime/scratchnative.hpp>
int main(void)
{
"#
    .to_string();

    for var in project.variables {
        str.push_str(&format!("ScratchValue {} = {{}};\n", var.replace(' ', "_")));
    }

    for list in project.lists {
        str.push_str(&format!("ScratchList {} = {{}};\n", list.replace(' ', "_")));
    }

    for proc in project.procedures {
        str.push_str(&codegen_stmt(proc));
    }

    str.push_str(&codegen_stmt(project.body));
    str.push_str("\nreturn 0;\n\n}");

    str
}
