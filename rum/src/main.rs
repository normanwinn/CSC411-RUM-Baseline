#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::env::args;
use std::fs::File;
use std::io::Read;
use rum::rum::vm;

fn main() {
    // Handle inputs
    let arguments: Vec<String> = args().collect();
    let filename = &arguments[1];
    let mut groupedBytes: Vec<[u8; 4]> = vec![];
    
    // Collect all instructions
    let infile = File::open(filename).unwrap();
	let all_bytes = infile.bytes().map(|byte| byte.unwrap());
	for curChunk in all_bytes.collect::<Vec<u8>>().chunks(4) {
		groupedBytes.push([
			curChunk[0],
			curChunk[1],
			curChunk[2],
			curChunk[3],
		]);
	}

    // Put instructions into memory
	let words: Vec<u32> = groupedBytes.into_iter()
			.map(|byteGroup| u32::from_be_bytes(byteGroup))
			.collect();
    
    // Create a "virtual machine"
    let mut RUM = vm {
        running: true,
    	registers: vec![0_u32; 8],
    	memory: vec![words],
    	unmappedSegments: vec![],
    	totalMappedSegments: 0,
    	programCounter: 0
    };

    // Run the virtual machine
    let mut curInstruction: u32;
    while RUM.running {
    	curInstruction = RUM.getInstruction();
    	RUM.run(curInstruction);
    }
}