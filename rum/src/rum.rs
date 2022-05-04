#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use crate::instructions::*;

// General vm structure
pub struct vm {
	pub running: bool,
	pub registers: Vec<u32>,
	pub memory: Vec<Vec<u32>>,
	pub unmappedSegments: Vec<usize>,
	pub totalMappedSegments: usize,
	pub programCounter: u32,
}

impl vm {
	// Get Instruction
	pub fn getInstruction(&mut self) -> u32 {
		let curInstruction = self.memory[0][self.programCounter as usize];
		self.programCounter += 1;
		return curInstruction;
	}

	// Runs the instruction
	pub fn run(&mut self, instruction: u32){
		let command: u8 = bitpack::bitpack::getu(instruction.into(), 4, 28).try_into().unwrap();
		let realInstruction: u32 = instruction.try_into().unwrap();
		match command {
			0 =>  cdl(self, realInstruction),
			1 =>  sgl(self, realInstruction),
			2 =>  sgs(self, realInstruction),
			3 =>  add(self, realInstruction),
			4 =>  mul(self, realInstruction),
			5 =>  div(self, realInstruction),
			6 =>  nan(self, realInstruction),
			7 =>  hal(self),
			8 =>  msg(self, realInstruction),
			9 =>  usg(self, realInstruction),
			10 => out(self, realInstruction),
			11 => inp(self, realInstruction),
			12 => ldp(self, realInstruction),
			13 => ldv(self, realInstruction),
			_ => panic!("Invalid instruction.")
		};
	}
}