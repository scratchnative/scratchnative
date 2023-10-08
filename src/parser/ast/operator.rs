use crate::parser::ast::*;

pub fn expr_from_operator(file: &ScratchFile, block: ScratchBlock, operator: &str) -> Expr {
    debug!("{:#?} {}", block, operator);
    match operator {
        "add" | "subtract" | "multiply" | "divide" | "and" | "random" | "mod" => Expr::BinOp {
            lhs: Box::new(scratch_val_to_expr(file, block.inputs["NUM1"].1.clone())),
            rhs: Box::new(scratch_val_to_expr(file, block.inputs["NUM2"].1.clone())),
            op: OpType::from_str(operator),
        },

        "join" => Expr::BinOp {
            lhs: Box::new(scratch_val_to_expr(file, block.inputs["STRING1"].1.clone())),
            rhs: Box::new(scratch_val_to_expr(file, block.inputs["STRING2"].1.clone())),
            op: OpType::from_str(operator),
        },

        "gt" | "lt" | "equals" | "or" => Expr::BinOp {
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

        "length" => Expr::SingleOp {
            op: OpType::from_str(operator),
            expr: {
                let val = block.inputs.iter().collect::<Vec<_>>()[0].1 .1.clone();
                Box::new(scratch_val_to_expr(file, val))
            },
        },

        "letter_of" => Expr::LetterOf {
            val: Box::new(scratch_val_to_expr(file, block.inputs["STRING"].1.clone())),
            index: Box::new(scratch_val_to_expr(file, block.inputs["LETTER"].1.clone())),
        },

        _ => todo!("{}", operator),
    }
}
