use crate::parser::ast::*;

pub fn data_to_statement(file: &ScratchFile, block: ScratchBlock, op: String) -> Stmt {
    match op.as_str() {
        "addtolist" => Stmt::AddToList {
            name: block.fields["LIST"][0].to_string(),
            val: scratch_val_to_expr(file, block.inputs["ITEM"].1.clone()),
            id: block.fields["LIST"][1].to_string(),
        },

        "setvariableto" => Stmt::SetVariable {
            name: block.fields["VARIABLE"][0].to_string(),
            val: scratch_val_to_expr(file, block.inputs["VALUE"].1.clone()),
            id: block.fields["VARIABLE"][1].to_string(),
        },

        _ => todo!("data_{}", op),
    }
}
