use colored::*;
use log::*;
use std::collections::HashMap;

pub use self::ast::*;
pub use self::json::*;
mod ast;
mod json;

#[derive(Debug)]
pub struct ScratchMetadata {
    pub user_agent: String,
    pub semantic_version: String,
    pub vm_version: String,
}

#[derive(Debug)]
pub enum ScratchInitializer {
    List(Vec<()>),
    Int(i64),
}

#[derive(Debug)]
//  "variables": { "`jEk@4|i[#Fk?(8x)AV.-my variable": ["my variable", 0] },
pub struct ScratchVariableDecl(String, ScratchInitializer);

#[derive(Debug, Clone)]
pub enum ScratchValueData {
    Int(i64),
    String(String),
    BlockCall(String),
}

#[derive(Debug, Clone)]
pub enum ScratchTypes {
    String = 10,
    Number = 4,
    BlockCall = 3,
}

impl ScratchTypes {
    fn from_i64(value: i64) -> ScratchTypes {
        match value {
            10 => ScratchTypes::String,
            4 => ScratchTypes::Number,
            3 => ScratchTypes::BlockCall,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
//  [10, "1"]
pub struct ScratchValue(ScratchTypes, ScratchValueData);

#[derive(Debug, Clone)]
// [1, [10, "1"]]
pub struct ScratchInput(i64, ScratchValue);

#[derive(Debug, Clone)]
pub struct ScratchBlock {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: HashMap<String, ScratchInput>,
    pub fields: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct ScratchTarget {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, ScratchVariableDecl>,
    pub lists: HashMap<String, ScratchVariableDecl>,
    pub blocks: HashMap<String, ScratchBlock>,
}

#[derive(Debug)]
pub struct ScratchFile {
    pub metadata: ScratchMetadata,
    pub targets: Vec<ScratchTarget>,
}

pub fn show_info(file: &ScratchFile) {
    println!("{}", "File metadata: ".white().bold());

    let print_val =
        |name: String, val: &String| println!("  {} : {}", name.white(), val.cyan().bold());

    print_val("User Agent".to_string(), &file.metadata.user_agent);
    print_val(
        "Semantic version".to_string(),
        &file.metadata.semantic_version,
    );
    print_val("Virtual machine".to_string(), &file.metadata.vm_version);
}

fn scratch_variable_decl_of_json(vec: Vec<serde_json::Value>) -> ScratchVariableDecl {
    let var_type = match (vec[1].is_array(), vec[1].is_i64()) {
        (true, false) => ScratchInitializer::List(vec![]),
        (false, true) => ScratchInitializer::Int(0),
        (_, _) => unreachable!(),
    };

    ScratchVariableDecl(vec[0].to_string(), var_type)
}

fn scratch_value_of_array(array: Vec<serde_json::Value>) -> ScratchValue {
    let val_type: ScratchTypes = ScratchTypes::from_i64(array[0].as_i64().unwrap());

    let val = array[1].as_str().unwrap().to_string();

    let data: ScratchValueData = match val_type {
        ScratchTypes::String => ScratchValueData::String(val),
        ScratchTypes::Number => ScratchValueData::Int(val.parse::<i64>().unwrap()),
        ScratchTypes::BlockCall => ScratchValueData::BlockCall(val),
    };

    ScratchValue(val_type, data)
}

fn scratch_block_of_json(block: &JsonScratchBlock) -> ScratchBlock {
    let mut inputs: HashMap<String, ScratchInput> = Default::default();
    let mut fields: HashMap<String, Vec<String>> = Default::default();

    let next = if block.next.is_null() {
        None
    } else {
        Some(block.next.as_str().unwrap().to_string())
    };

    let parent = if block.parent.is_null() {
        None
    } else {
        Some(block.parent.as_str().unwrap().to_string())
    };

    for field in &block.fields {
        fields.insert(field.0.to_string(), field.1.to_vec());
    }

    for input in &block.inputs {
        let array = input.1.as_array().unwrap().to_vec();
        let val: ScratchValue = if array[1].is_string() {
            scratch_value_of_array(array.clone())
        } else {
            scratch_value_of_array(array[1].as_array().unwrap().to_vec())
        };

        let _input = ScratchInput(array[0].as_i64().unwrap(), val);

        inputs.insert(input.0.to_string(), _input);
    }

    ScratchBlock {
        opcode: block.opcode.to_string(),
        next,
        parent,
        inputs,
        fields,
    }
}

fn scratch_target_of_json(target: &JsonScratchTarget) -> ScratchTarget {
    let mut variables: HashMap<String, ScratchVariableDecl> = Default::default();
    let mut lists: HashMap<String, ScratchVariableDecl> = Default::default();
    let mut blocks: HashMap<String, ScratchBlock> = Default::default();

    for var in &target.variables {
        variables.insert(
            var.0.to_string(),
            scratch_variable_decl_of_json(var.1.to_vec()),
        );
    }

    for list in &target.lists {
        lists.insert(
            list.0.to_string(),
            scratch_variable_decl_of_json(list.1.to_vec()),
        );
    }

    for block in &target.blocks {
        blocks.insert(block.0.to_string(), scratch_block_of_json(block.1));
    }

    ScratchTarget {
        is_stage: target.is_stage,
        name: target.name.to_string(),
        variables,
        lists,
        blocks,
    }
}

pub fn parse_scratch_file(contents: String) -> ScratchFile {
    let json: JsonScratchFile = serde_json::from_str(&contents).unwrap();
    let metadata = ScratchMetadata {
        semantic_version: json.meta.semver,
        user_agent: json.meta.agent,
        vm_version: json.meta.vm,
    };

    let mut targets: Vec<ScratchTarget> = vec![];

    for val in json.targets.iter() {
        targets.push(scratch_target_of_json(val));
    }

    ScratchFile { metadata, targets }
}
