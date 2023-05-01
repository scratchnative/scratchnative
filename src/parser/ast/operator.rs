use crate::parser::ast::*;

pub fn expr_from_operator(file: &ScratchFile, block: ScratchBlock, operator: &str) -> Expr {
    match operator {
        "add" | "subtract" | "multiply" | "divide" | "and" | "or" | "random" | "mod" => {
            Expr::BinOp {
                lhs: Box::new(scratch_val_to_expr(file, block.inputs["NUM1"].1.clone())),
                rhs: Box::new(scratch_val_to_expr(file, block.inputs["NUM2"].1.clone())),
                op: OpType::from_str(operator),
            }
        }

        "gt" | "lt" | "equals" => Expr::BinOp {
            lhs: Box::new(scratch_val_to_expr(
                file,
                block.inputs["OPERAND1"].1.clone(),
            )),
            rhs: Box::new(scratch_val_to_expr(
                file,
                block.inputs["OPERAND2"].1.clone(),
            )),
            op: OpType::from_str(operator),
        },

        "not" => Expr::SingleOp {
            expr: Box::new(scratch_val_to_expr(file, block.inputs["OPERAND"].1.clone())),
            op: OpType::from_str(operator),
        },

        _ => todo!("{}", operator),
    }
}
