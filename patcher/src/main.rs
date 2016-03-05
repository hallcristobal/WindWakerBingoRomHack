mod dol;
mod assembler;

use std::io::prelude::*;
use std::fs::File;
use assembler::Assembler;

fn main() {
    let mut asm = String::new();
    let _ = File::open("../src/src/patch.asm")
                .unwrap()
                .read_to_string(&mut asm);

    let lines = &asm.lines().collect::<Vec<_>>();

    let mut assembler = Assembler::new("../build/intermediate.elf");
    let instructions = &assembler.assemble_all_lines(lines);

    let mut original = Vec::new();
    let _ = File::open("../game/original.dol")
                .unwrap()
                .read_to_end(&mut original);

    let mut intermediate = Vec::new();
    let _ = File::open("../build/intermediate.dol")
                .unwrap()
                .read_to_end(&mut intermediate);

    let mut original = dol::DolFile::new(&original);
    let intermediate = dol::DolFile::new(&intermediate);
    original.append(intermediate);

    // println!("{:#?}", tww);

    original.patch(instructions);

    let data = original.to_bytes();
    let mut file = File::create("../game/sys/main.dol")
                       .unwrap();

    let _ = file.write(&data);
}
