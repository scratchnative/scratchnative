use crate::parser::*;

use super::{block_chain_to_vec, scratch_val_to_expr};

pub fn procedures_to_statement(file: &ScratchFile, block: ScratchBlock, op: &str) -> Stmt {
    match op {
        "definition" => Stmt::ProcedureDefinition {
            prototype: {
                let proto_block_name = if block.inputs.get("custom_block").is_some() {
                    if let ScratchValueData::BlockCall(x) = &block.inputs["custom_block"].1 .1 {
                        Some(x.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                };

                let proto_block =
                    file.targets[block.target].blocks[&proto_block_name.unwrap()].clone();

                ProcedurePrototype {
                    params: {
                        let mut ret: HashMap<String, String> = Default::default();
                        let ids_arr: Vec<String> = serde_json::from_str(
                            proto_block.mutation["argumentids"].as_str().unwrap(),
                        )
                        .unwrap();
                        let names_arr: Vec<String> = serde_json::from_str(
                            proto_block.mutation["argumentnames"].as_str().unwrap(),
                        )
                        .unwrap();
                        for x in 0..ids_arr.len() {
                            ret.insert(ids_arr[x].to_string(), names_arr[x].to_string());
                        }
                        ret
                    },
                    param_order: serde_json::from_str(
                        proto_block.mutation["argumentnames"].as_str().unwrap(),
                    )
                    .unwrap(),
                    name: proto_block.mutation["proccode"]
                        .as_str()
                        .unwrap()
                        .replace(' ', "_")
                        .to_string(),
                }
            },
            body: BlockStmt {
                stmts: block_chain_to_vec(
                    file,
                    file.targets[block.target].blocks[&block.next.unwrap()].clone(),
                ),
            },
        },

        "call" => Stmt::ProcedureCall {
            proc: block.mutation["proccode"]
                .as_str()
                .unwrap()
                .replace(' ', "_")
                .to_string(),
            params: {
                let ids_arr: Vec<String> =
                    serde_json::from_str(block.mutation["argumentids"].as_str().unwrap()).unwrap();

                let mut params: Vec<Expr> = vec![];

                for x in ids_arr.iter() {
                    params.push(scratch_val_to_expr(file, block.inputs[x].clone().1, &block));
                }

                params
            },
        },

        _ => todo!("procedures_{}", op),
    }
}
