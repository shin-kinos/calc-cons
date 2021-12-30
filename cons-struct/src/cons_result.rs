
use std::fs::File;
use std::io::Write;
use colored::*;

use crate::error;

pub fn show_result(
	//site_list         : &Vec<String>,
	cons_capra07_list : &Vec<f64>,
	seq_3_letters     : &Vec<String>,
	arg_c             : &String
) {

	println!( "\nResult : \n" );

	if ( *seq_3_letters ).len() != ( *cons_capra07_list ).len() { error::error_bomb( "site_ent_len_not_same" ); }

	if ( *arg_c ).as_str() == "yes" {
		println!( "Colorize :" );
		println!( "{}", "Aliphatic (ALA, VAL, LEU, ILE, MET, CYS)".on_yellow().black() );
		println!( "{}",        "Aromatic (PHE, TRP, TYR, HIS)".on_cyan().black()       );
		println!( "{}",           "Polar (SER, THR, ASN, GLN)".on_green().black()      );
		println!( "{}",              "Positive (LYS, ARG)".on_blue().black()           );
		println!( "{}",              "Negative (ASP, GLU)".on_red().black()            );
		println!( "{}", "Special conformations (GLY, PRO)".on_magenta().black()        );
		println!( "" );

		println!( "ConsNum\tCons\tConsRes" );
		for i in 0 .. ( *cons_capra07_list ).len() {
			print!( "{}\t", i + 1 );
			print!( "{:.3}\t", ( *cons_capra07_list )[ i ] );
			colorize( &( ( *seq_3_letters )[ i ] ) );
		}

	} else if ( *arg_c ).as_str() == "no" {
		/**/
		println!( "ConsNum\tCons\tConsRes" );
		for i in 0 .. ( *cons_capra07_list ).len() {
			println!( "{}\t{:.3}\t{}", i + 1, ( *cons_capra07_list )[ i ], ( *seq_3_letters )[ i ] );
		}
	}

}

pub fn save_result(
	//site_list         : &Vec<String>,
	cons_capra07_list : &Vec<f64>,
	seq_3_letters     : &Vec<String>,
	arg_o             : &String
) {

	let mut fout = File::create( ( *arg_o ).as_str() ).expect( "FAILED to open output file" );

	writeln!( fout, "{}", "ConsNum\tCons\tConsRes" ).expect( "FAILED to write" );

	for i in 0 .. ( *cons_capra07_list ).len() {
		writeln!( fout, "{}\t{:.3}\t{}", i + 1, ( *cons_capra07_list )[ i ], ( *seq_3_letters )[ i ] ).expect( "FAILED to write" );
	}

	println!( "\nThe conservation output file was correctly written.\n" );
}

fn colorize( residue : &String ) {

	//let sequence : Vec<char> = ( *arg ).chars().collect();
	//println!("{:?}", sequence);

	match ( *residue ).as_str() {
		"ALA" | "VAL" | "LEU" | "ILE" | "MET" | "CYS" => println!( "{}", ( *residue ).to_string().on_yellow().black()  ),
		"PHE" | "TRP" | "TYR" | "HIS"                 => println!( "{}", ( *residue ).to_string().on_cyan().black()    ),
		"SER" | "THR" | "ASN" | "GLN"                 => println!( "{}", ( *residue ).to_string().on_green().black()   ),
		"LYS" | "ARG"                                 => println!( "{}", ( *residue ).to_string().on_blue().black()    ),
		"ASP" | "GLU"                                 => println!( "{}", ( *residue ).to_string().on_red().black()     ),
		"GLY" | "PRO"                                 => println!( "{}", ( *residue ).to_string().on_magenta().black() ),
		_                                             => println!( "{}", ( *residue ).to_string()                      ),
	}

	/*
	for symbol in sequence.iter() {
		match *symbol {
			'A' | 'V' | 'L' | 'I' | 'M' | 'C' => print!( "{}", ( *symbol ).to_string().on_yellow().black()  ),
			'F' | 'W' | 'Y' | 'H'             => print!( "{}", ( *symbol ).to_string().on_cyan().black()    ),
			'S' | 'T' | 'N' | 'Q'             => print!( "{}", ( *symbol ).to_string().on_green().black()   ),
			'K' | 'R'                         => print!( "{}", ( *symbol ).to_string().on_blue().black()    ),
			'D' | 'E'                         => print!( "{}", ( *symbol ).to_string().on_red().black()     ),
			'G' | 'P'                         => print!( "{}", ( *symbol ).to_string().on_magenta().black() ),
			'B' | 'Z' | 'J' | 'O'             => print!( "{}", ( *symbol ).to_string().yellow()             ),
			'X'                               => print!( "{}", ( *symbol ).to_string().red()                ),
			_                                 => print!( "{}", ( *symbol ).to_string()                      ),
		}
	}
	*/

}
