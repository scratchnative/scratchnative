use crate::parser::ast::*;

pub fn event_to_statement(
    file: &ScratchFile,
    _block: ScratchBlock,
    next_block: Option<ScratchBlock>,
    event: String,
) -> Stmt {
    match event.as_str() {
        "whenflagclicked" => Stmt::WhenFlagClicked(BlockStmt {
            stmts: block_chain_to_vec(file, next_block.unwrap()),
        }),
        _ => todo!("event_{}", event),
    }
}
