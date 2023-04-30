use std::collections::HashMap;

use crate::parser::*;
use log::*;

fn iterate_through_blocks(root_block: &ScratchBlock, blocks: &HashMap<String, ScratchBlock>) {
    let mut curr_block = root_block;

    loop {
        let next = &curr_block.next;

        trace!("Compiling {:#?}", curr_block);

        if next.is_none() {
            break;
        }

        curr_block = blocks.get(&next.clone().unwrap().to_string()).unwrap();
    }
}

pub fn codegen_file(file: ScratchFile) {
    let mut root_block: Option<&ScratchBlock> = None;

    for block in file.targets[0].blocks.iter() {
        if block.1.parent.is_none() && block.1.opcode != "procedures_definition" {
            root_block = Some(block.1);
            break;
        }
    }

    debug!("Root block is {:#?}", root_block);

    iterate_through_blocks(root_block.unwrap(), &file.targets[0].blocks);
}
