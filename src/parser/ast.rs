use crate::parser::{ScratchBlock, ScratchFile, ScratchValue, ScratchValueData};

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
    SetVariable { name: String, id: String, val: Expr },
    AddToList { name: String, id: String, val: Expr },
}

#[derive(Debug)]
pub struct Project {
    pub body: Stmt,
}

fn block_chain_to_vec(file: &ScratchFile, root_block: ScratchBlock) -> Vec<Stmt> {
    let mut curr_block = &root_block;
    let mut ret: Vec<Stmt> = vec![];

    loop {
        let next = &curr_block.next;

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

fn expr_from_operator(file: &ScratchFile, block: ScratchBlock, operator: &str) -> Expr {
    match operator {
        "add" | "subtract" | "multiply" | "divide" | "gt" | "lt" | "and" | "or" | "not"
        | "random" | "mod" => Expr::BinOp {
            lhs: Box::new(scratch_val_to_expr(file, block.inputs["NUM1"].1.clone())),
            rhs: Box::new(scratch_val_to_expr(file, block.inputs["NUM2"].1.clone())),
            op: OpType::from_str(operator),
        },
        _ => todo!("{}", operator),
    }
}

fn expr_from_block(file: &ScratchFile, block: ScratchBlock) -> Expr {
    let str_array: Vec<&str> = block.opcode.as_str().split('_').collect();

    match str_array[0] {
        "operator" => expr_from_operator(file, block.clone(), str_array[1]),
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

    match block.opcode.as_str() {
        "event_whenflagclicked" => Stmt::WhenFlagClicked(BlockStmt {
            stmts: block_chain_to_vec(file, next_block.unwrap()),
        }),

        "data_addtolist" => Stmt::AddToList {
            name: block.fields["LIST"][0].to_string(),
            val: scratch_val_to_expr(file, block.inputs["ITEM"].1.clone()),
            id: block.fields["LIST"][1].to_string(),
        },

        "data_setvariableto" => Stmt::SetVariable {
            name: block.fields["VARIABLE"][0].to_string(),
            val: scratch_val_to_expr(file, block.inputs["VALUE"].1.clone()),
            id: block.fields["VARIABLE"][1].to_string(),
        },

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
