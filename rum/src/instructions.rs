#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use crate::rum::vm;
use std::io;
use std::io::Read;

// Collect the 3 registers from the instruction
fn unpack3Registers(instr: u32) -> (usize, usize, usize) {
	let word = instr as u64;
	let a = bitpack::bitpack::getu(word, 3, 6).try_into().unwrap();
	let b = bitpack::bitpack::getu(word, 3, 3).try_into().unwrap();
	let c = bitpack::bitpack::getu(word, 3, 0).try_into().unwrap();
	return (a, b, c);
}

// Collect the 2 registers from the instruction
fn unpack2Registers(instr: u32) -> (usize, usize) {
	let word = instr as u64;
	let b = bitpack::bitpack::getu(word, 3, 3).try_into().unwrap();
	let c = bitpack::bitpack::getu(word, 3, 0).try_into().unwrap();
	return (b, c);
}

// Collect the register from the instruction
fn unpack1Register(instr: u32) -> usize {
    let word = instr as u64;
	let a = bitpack::bitpack::getu(word, 3, 0).try_into().unwrap();
    return a;
}

// Conditional Load
pub fn cdl(vm: &mut vm, instruction: u32) {
	let (a, b, c) = unpack3Registers(instruction);
	if vm.registers[c] != 0 {
		vm.registers[a] = vm.registers[b];
	}
}

// Segmented Load
pub fn sgl(vm: &mut vm, instruction: u32) {
	let (a, b, c) = unpack3Registers(instruction);
	vm.registers[a] = vm.memory[vm.registers[b] as usize][vm.registers[c] as usize];
}

// Segmented Store
pub fn sgs(vm: &mut vm, instruction: u32) {
	let (a, b, c) = unpack3Registers(instruction);
	vm.memory[vm.registers[a] as usize][vm.registers[b] as usize] = vm.registers[c];
}

// Add
pub fn add(vm: &mut vm, instruction: u32) {
	let (a, b, c) = unpack3Registers(instruction);
	vm.registers[a] = ((vm.registers[b] as u64 + vm.registers[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

// Multiply
pub fn mul(vm: &mut vm, instruction: u32) {
	let (a, b, c) = unpack3Registers(instruction);
	vm.registers[a] = ((vm.registers[b] as u64 * vm.registers[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

// Divide
pub fn div(vm: &mut vm, instruction: u32) {
	let (a, b, c) = unpack3Registers(instruction);
	vm.registers[a] = vm.registers[b] / vm.registers[c];	
}

// Bitwise NAND
pub fn nan(vm: &mut vm, instruction: u32) {
	let (a, b, c) = unpack3Registers(instruction);
	vm.registers[a] = !(vm.registers[b] & vm.registers[c]);	
}

// Halt
pub fn hal(vm: &mut vm) {
	vm.running = false;
}

// Map Segment
pub fn msg(vm: &mut vm, instruction: u32) {
	let (b, c) = unpack2Registers(instruction);
	if vm.unmappedSegments.len() != 0 {
		let segNumber = vm.unmappedSegments.pop().unwrap();
		vm.memory[segNumber] = vec![0; vm.registers[c] as usize];
		vm.registers[b] = segNumber as u32;
	} else {
		vm.totalMappedSegments += 1;
		vm.memory.push(vec![0; vm.registers[c] as usize]);
		vm.registers[b] = vm.totalMappedSegments as u32;
	}
}

// Unmap Segment
pub fn usg(vm: &mut vm, instruction: u32) {
	let c = unpack1Register(instruction);
	vm.memory[vm.registers[c] as usize].clear();
	vm.unmappedSegments.push(vm.registers[c].try_into().unwrap());
}

// Output
pub fn out(vm: &mut vm, instruction: u32) {
	let c = unpack1Register(instruction);
	print!("{}", vm.registers[c] as u8 as char);
}

// Input
pub fn inp(vm: &mut vm, instruction: u32) {
	let c = unpack1Register(instruction);
	let mut offset: [u8; 1] = [0; 1];
	let number = io::stdin().read(&mut offset);
	let value = match number {
		Ok(0) => u32::MAX,
		Ok(1) => offset[0] as u32,
		_ => panic!("")
	};
	vm.registers[c] = value;
}

// Load Program
pub fn ldp(vm: &mut vm, instruction: u32) {
	let (b, c) = unpack2Registers(instruction);
	if vm.registers[b] != 0 {
		vm.memory[0] = vm.memory[vm.registers[b] as usize].clone();
	}
	vm.programCounter = vm.registers[c];
}

// Load Value
pub fn ldv(vm: &mut vm, instruction: u32) {
	let value = bitpack::bitpack::getu(instruction as u64, 25, 0) as u32;
	let a = bitpack::bitpack::getu(instruction as u64, 3, 25);
	vm.registers[a as usize] = value;
}