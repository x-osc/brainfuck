// slow and bad brainfuck interpreter

use std::io::{self, Write};

pub fn run<W: Write>(source: String, input: String, mut out: W) -> io::Result<()> {
    let program: Vec<char> = source.chars().collect();
    let mut input = input.bytes();

    let mut instruction = 0;
    let mut pointer: usize = 0;
    let mut memory: [u8; 30000] = [0; 30000];

    loop {
        let Some(command) = program.get(instruction) else {
            break;
        };

        match command {
            '>' => pointer = (pointer + 1) % memory.len(),
            '<' => pointer = (pointer + memory.len() - 1) % memory.len(),
            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            ',' => {
                if let Some(inp) = input.next() {
                    memory[pointer] = inp as u8;
                }
            }
            '.' => out.write_all(&[memory[pointer]])?,
            '[' => {
                if memory[pointer] == 0 {
                    let mut bracket_count = 1;
                    while bracket_count > 0 {
                        instruction += 1;
                        match program[instruction] {
                            '[' => bracket_count += 1,
                            ']' => bracket_count -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                let mut bracket_count = 1;
                while bracket_count > 0 {
                    instruction -= 1;
                    match program[instruction] {
                        ']' => bracket_count += 1,
                        '[' => bracket_count -= 1,
                        _ => {}
                    }
                }
                instruction = instruction.saturating_sub(1);
            }
            _ => {}
        }

        instruction += 1;
    }

    out.flush()
}
