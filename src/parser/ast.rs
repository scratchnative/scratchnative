use log::*;

use crate::parser::{ScratchBlock, ScratchFile, ScratchValue};

#[derive(Debug)]
pub struct Add {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct Value {
    val: ScratchValue,
}

#[derive(Debug)]
pub enum Expr {
    Add(Add),
    Val(Value),
}

#[derive(Debug)]
pub struct EmptyStmt {}

#[derive(Debug)]
pub struct BlockStmt {
    stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct SetVariable {
    var: String,
    val: Expr,
}

#[derive(Debug)]
pub enum Stmt {
    WhenFlagClicked(BlockStmt),
    SetVariable(SetVariable),
}

#[derive(Debug)]
pub struct Project {
    body: Stmt,
}

fn block_chain_to_vec(file: &ScratchFile, root_block: ScratchBlock) -> Vec<Stmt> {
    let mut curr_block = &root_block;

    loop {
        let next = &curr_block.next;

        info!("curr: {:#?} next: {:#?}", curr_block, next);
        scratch_block_to_statement(file, curr_block.clone());
        info!("Hi");

        if next.is_none() {
            break;
        }

        curr_block = file.targets[0]
            .blocks
            .get(&next.clone().unwrap().to_string())
            .unwrap();
    }

    vec![]
}

fn scratch_block_to_statement(file: &ScratchFile, block: ScratchBlock) -> Stmt {
    let mut next_block: Option<ScratchBlock> = None;

    if block.next.is_some() {
        next_block = Some(
            file.targets[0]
                .blocks
                .get(&block.next.clone().unwrap())
                .unwrap()
                .clone(),
        );
    }

    match block.opcode.as_str() {
        "event_whenflagclicked" => Stmt::WhenFlagClicked(BlockStmt {
            stmts: block_chain_to_vec(file, next_block.unwrap()),
        }),
        x => todo!("{}", x),
    }
}

fn scratch_file_to_body(file: &ScratchFile, root_block: ScratchBlock) -> Stmt {
    scratch_block_to_statement(file, root_block)
}

pub fn scratch_file_to_project(file: &ScratchFile) -> Project {
    let mut root_block: Option<ScratchBlock> = None;

    for block in file.targets[0].blocks.iter() {
        if block.1.parent.is_none() && block.1.opcode != "procedures_definition" {
            root_block = Some(block.1.clone());
            break;
        }
    }

    Project {
        body: scratch_file_to_body(file, root_block.unwrap()),
    }
}
