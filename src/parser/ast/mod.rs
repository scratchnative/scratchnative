use std::collections::HashMap;

use log::debug;

use crate::parser::{ScratchBlock, ScratchFile, ScratchValue, ScratchValueData};

mod control;
mod data;
mod event;
mod operator;
mod procedures;

use super::ScratchTypes;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone)]
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
    Length,
    LetterOf,
    Join,
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
            "length" => OpType::Length,
            "letter_of" => OpType::LetterOf,
            "join" => OpType::Join,

            _ => todo!("{}", str),
        }
    }
}

#[derive(Debug, Clone)]
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

    LetterOf {
        val: Box<Expr>,
        index: Box<Expr>,
    },

    ItemOf {
        list_name: String,
        index: Box<Expr>,
    },

    Val(Value),
    Var(String),
    Param(String),
}

#[derive(Debug)]
pub struct EmptyStmt {}

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct ProcedurePrototype {
    // id: name
    pub params: HashMap<String, String>,
    pub param_order: Vec<String>,
    pub name: String,
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

    DeleteAllOfList {
        name: String,
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

    ChangeBy {
        var_name: String,
        inc: Expr,
    },

    ProcedureCall {
        proc: String,
        params: Vec<Expr>,
    },
    ProcedureDefinition {
        prototype: ProcedurePrototype,
        body: BlockStmt,
    },

    Empty,
}

#[derive(Debug)]
pub struct Project {
    pub body: Stmt,
    pub variables: Vec<String>,
    pub lists: Vec<String>,
    pub procedures: Vec<Stmt>,
}

fn block_chain_to_vec(file: &ScratchFile, root_block: ScratchBlock) -> Vec<Stmt> {
    let mut curr_block = &root_block;
    let mut ret: Vec<Stmt> = vec![];

    loop {
        let next = &curr_block.next;

        ret.push(scratch_block_to_statement(file.clone(), curr_block.clone()));

        if next.is_none() {
            break;
        }

        curr_block = file.targets[curr_block.target]
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
        "data" => data::expr_from_data(file, block.clone(), str_array[1]),
        "argument" => Expr::Param(
            block.fields["VALUE"].to_vec()[0]
                .as_str()
                .unwrap()
                .to_string(),
        ),
        _ => todo!("{}", block.opcode.as_str()),
    }
}

fn scratch_val_to_expr(file: &ScratchFile, val: ScratchValue, orig_block: &ScratchBlock) -> Expr {
    let block = match val.1.clone() {
        ScratchValueData::BlockCall(x) => Some(x),
        _ => None,
    };

    debug!("{:#?}", val);
    match val.0 {
        ScratchTypes::Number => Expr::Val(scratch_val_data_to_val(&val.1)),
        ScratchTypes::String => Expr::Val(scratch_val_data_to_val(&val.1)),
        ScratchTypes::BlockCall => expr_from_block(
            file,
            file.targets[orig_block.target].blocks[&block.unwrap()].clone(),
        ),

        ScratchTypes::Variable => {
            if let ScratchValueData::Variable(x) = val.1 {
                Expr::Var(x)
            } else {
                unreachable!()
            }
        }
    }
}

fn scratch_block_to_statement(file: ScratchFile, block: ScratchBlock) -> Stmt {
    let mut next_block: Option<ScratchBlock> = None;
    if block.next.is_some() {
        next_block = Some(
            file.targets[block.target]
                .blocks
                .get(&block.next.clone().unwrap())
                .unwrap()
                .clone(),
        );
    }

    let str_array: Vec<&str> = block.opcode.as_str().splitn(2, '_').collect();

    match str_array[0] {
        "event" => {
            event::event_to_statement(&file, block.clone(), next_block, str_array[1].to_string())
        }

        "data" => data::data_to_statement(&file, block.clone(), str_array[1].to_string()),

        "control" => control::control_to_statement(
            &file,
            block.clone(),
            next_block,
            str_array[1].to_string(),
        ),

        "procedures" => procedures::procedures_to_statement(&file, block.clone(), str_array[1]),

        _ => todo!("{}", block.opcode.as_str()),
    }
}

pub fn scratch_file_to_project(mut file: ScratchFile) -> Project {
    let mut root_block: Option<ScratchBlock> = None;

    let mut vars: Vec<String> = vec![];
    let mut lists: Vec<String> = vec![];
    let mut procedures: Vec<Stmt> = vec![];

    for (i, target) in file.targets.iter_mut().enumerate() {
        for var in &target.variables {
            vars.push(var.1 .0.to_string());
        }

        for list in &target.lists {
            lists.push(list.1 .0.to_string());
        }

        for block in &mut target.blocks {
            block.1.target = i;

            log::info!("{}", i);

            if block.1.parent.is_none() && block.1.opcode == *"event_whenflagclicked" {
                root_block = Some(block.1.clone());
            }

            if block.1.parent.is_none() && block.1.opcode == *"procedures_definition" {
                procedures.push(scratch_block_to_statement(file, block.1.clone()));
            }
        }
    }

    debug!("root block is {:#?}", root_block);

    Project {
        body: scratch_block_to_statement(file, root_block.unwrap()),
        variables: vars,
        lists,
        procedures,
    }
}
