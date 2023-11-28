use crate::parser::ast::*;

pub fn control_to_statement(
    file: &ScratchFile,
    block: ScratchBlock,
    _next_block: Option<ScratchBlock>,
    op: String,
) -> Stmt {
    let get_input_block_name = |name: &str| -> Option<String> {
        if block.inputs.get(name).is_some() {
            if let ScratchValueData::BlockCall(x) = &block.inputs[name].1 .1 {
                Some(x.to_string())
            } else {
                None
            }
        } else {
            None
        }
    };

    match op.as_str() {
        "if" => {
            let condition_block_name = get_input_block_name("CONDITION");
            let body_block_name = get_input_block_name("SUBSTACK");

            if body_block_name.is_none() {
                return Stmt::Empty;
            }

            Stmt::If {
                condition: expr_from_block(
                    file,
                    file.targets[0].blocks[&condition_block_name.unwrap()].clone(),
                ),
                block: BlockStmt {
                    stmts: block_chain_to_vec(
                        file,
                        file.targets[0].blocks[&body_block_name.unwrap()].clone(),
                    ),
                },
            }
        }

        "repeat" => {
            let body_block_name = get_input_block_name("SUBSTACK");

            Stmt::Repeat {
                times: scratch_val_to_expr(file, block.inputs["TIMES"].1.clone(), &block),
                block: BlockStmt {
                    stmts: block_chain_to_vec(
                        file,
                        file.targets[0].blocks[&body_block_name.unwrap()].clone(),
                    ),
                },
            }
        }

        "repeat_until" => {
            let body_block_name = get_input_block_name("SUBSTACK");
            let condition_block_name = get_input_block_name("CONDITION");

            Stmt::RepeatUntil {
                condition: expr_from_block(
                    file,
                    file.targets[0].blocks[&condition_block_name.unwrap()].clone(),
                ),
                block: BlockStmt {
                    stmts: block_chain_to_vec(
                        file,
                        file.targets[0].blocks[&body_block_name.unwrap()].clone(),
                    ),
                },
            }
        }

        "if_else" => {
            let condition_block_name = get_input_block_name("CONDITION");
            let if_body_block_name = get_input_block_name("SUBSTACK");
            let else_body_block_name = get_input_block_name("SUBSTACK2");
            let else_stmts: Vec<Stmt>;

            // handle empty else bodies: if(condition) { do_something; } else {}
            if if_body_block_name.is_none() && else_body_block_name.is_none() {
                return Stmt::Empty;
            } else if else_body_block_name.is_none() {
                else_stmts = vec![];
            } else {
                else_stmts = block_chain_to_vec(
                    file,
                    file.targets[0].blocks[&else_body_block_name.unwrap()].clone(),
                )
            }

            Stmt::IfElse {
                condition: expr_from_block(
                    file,
                    file.targets[0].blocks[&condition_block_name.unwrap()].clone(),
                ),

                if_block: BlockStmt {
                    stmts: block_chain_to_vec(
                        file,
                        file.targets[0].blocks[&if_body_block_name.unwrap()].clone(),
                    ),
                },

                else_block: BlockStmt { stmts: else_stmts },
            }
        }

        _ => todo!("control_{}", op),
    }
}
