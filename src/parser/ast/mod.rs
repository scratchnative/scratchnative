use log::info;

use crate::parser::{ScratchBlock, ScratchFile, ScratchValue, ScratchValueData};

mod control;
mod data;
mod event;
mod operator;

use super::ScratchTypes;

#[derive(Debug)]
pub enum Value {
    Number(i64),
    String(String),
}

#[derive(Debug)]
pub enum OpType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
    Random,
    Modulo,
}

impl OpType {
    fn from_str(str: &str) -> OpType {
        match str {
            "add" => OpType::Add,
            "subtract" => OpType::Subtract,
            "multiply" => OpType::Multiply,
            "divide" => OpType::Divide,
            "equals" => OpType::Equals,
            "gt" => OpType::GreaterThan,
            "lt" => OpType::LessThan,
            "and" => OpType::And,
            "or" => OpType::Or,
            "not" => OpType::Not,
            "random" => OpType::Random,
            "mod" => OpType::Modulo,

            _ => todo!("{}", str),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    BinOp {
        op: OpType,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    SingleOp {
        op: OpType,
        expr: Box<Expr>,
    },

    Val(Value),
    Var(String),
}

#[derive(Debug)]
pub struct EmptyStmt {}

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    WhenFlagClicked(BlockStmt),
    SetVariable {
        name: String,
        id: String,
        val: Expr,
    },
    AddToList {
        name: String,
        id: String,
        val: Expr,
    },
    Repeat {
        times: Expr,
        block: BlockStmt,
    },
    RepeatUntil {
        condition: Expr,
        block: BlockStmt,
    },
    If {
        condition: Expr,
        block: BlockStmt,
    },
    IfElse {
        condition: Expr,
        if_block: BlockStmt,
        else_block: BlockStmt,
    },

    Empty,
}

#[derive(Debug)]
pub struct Project {
    pub body: Stmt,
    pub variables: Vec<String>,
}

fn block_chain_to_vec(file: &ScratchFile, root_block: ScratchBlock) -> Vec<Stmt> {
    let mut curr_block = &root_block;
    let mut ret: Vec<Stmt> = vec![];

    loop {
        let next = &curr_block.next;

        info!("curr_block is {:#?}", curr_block);

        ret.push(scratch_block_to_statement(file, curr_block.clone()));

        if next.is_none() {
            break;
        }

        curr_block = file.targets[0]
            .blocks
            .get(&next.clone().unwrap().to_string())
            .unwrap();
    }

    ret
}

fn scratch_val_data_to_val(data: &ScratchValueData) -> Value {
    match &data {
        ScratchValueData::Int(x) => Value::Number(*x),
        ScratchValueData::String(x) => Value::String(x.to_string()),
        _ => todo!(),
    }
}

fn expr_from_block(file: &ScratchFile, block: ScratchBlock) -> Expr {
    let str_array: Vec<&str> = block.opcode.as_str().splitn(2, '_').collect();

    match str_array[0] {
        "operator" => operator::expr_from_operator(file, block.clone(), str_array[1]),
        _ => todo!("{}", str_array[0]),
    }
}

fn scratch_val_to_expr(file: &ScratchFile, val: ScratchValue) -> Expr {
    let block = match val.1.clone() {
        ScratchValueData::BlockCall(x) => Some(x),
        _ => None,
    };

    match val.0 {
        ScratchTypes::Number => Expr::Val(scratch_val_data_to_val(&val.1)),
        ScratchTypes::String => Expr::Val(scratch_val_data_to_val(&val.1)),
        ScratchTypes::BlockCall => {
            expr_from_block(file, file.targets[0].blocks[&block.unwrap()].clone())
        }

        ScratchTypes::Variable => {
            if let ScratchValueData::Variable(x) = val.1 {
                Expr::Var(x)
            } else {
                unreachable!()
            }
        }
    }
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

    let str_array: Vec<&str> = block.opcode.as_str().splitn(2, '_').collect();

    match str_array[0] {
        "event" => {
            event::event_to_statement(file, block.clone(), next_block, str_array[1].to_string())
        }

        "data" => data::data_to_statement(file, block.clone(), str_array[1].to_string()),

        "control" => {
            control::control_to_statement(file, block.clone(), next_block, str_array[1].to_string())
        }

        x => todo!("{}", x),
    }
}

fn scratch_file_to_body(file: &ScratchFile, root_block: ScratchBlock) -> Stmt {
    scratch_block_to_statement(file, root_block)
}

pub fn scratch_file_to_project(file: &ScratchFile) -> Project {
    let mut root_block: Option<ScratchBlock> = None;

    for block in file.targets[0].blocks.iter() {
        if block.1.parent.is_none() && block.1.opcode != *"procedures_definition" {
            root_block = Some(block.1.clone());
            break;
        }
    }

    let mut vars: Vec<String> = vec![];

    for var in &file.targets[0].variables {
        vars.push(var.1 .0.to_string());
    }

    Project {
        body: scratch_file_to_body(file, root_block.unwrap()),
        variables: vars,
    }
}
