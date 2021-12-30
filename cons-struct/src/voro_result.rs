
use std::fs::File;
use std::io::Write;

use colored::*;

pub fn show_result(
	chainid_list : &Vec<&str>,
	resseq_list  : &Vec<usize>,
	resname_list : &Vec<&str>,
	asa_list     : &Vec<f64>,
	rsa_list     : &Vec<f64>,
	state_list   : &Vec<&str>,
	arg_c        : &String
) {

	println!( "\nResult : \n" );

	if ( *arg_c ) == "yes".to_string() {
		print!( "Number\t"  );
		print!( "ChainID\t" );
		print!( "ResSeq\t"  );
		print!( "ResName\t" );
		print!( "ResASA\t"  );
		print!( "ResRSA\t"  );
		print!( "State"     );
		print!( "\n"        );

		for i in 0 .. ( *rsa_list ).len() {
			print!( "{}\t", i + 1                         );
			print!( "{}\t", ( *chainid_list )[ i ]        );
			print!( "{}\t", ( *resseq_list  )[ i ]        );
			colorize_res( resname_list[ i ]               );
			print!( "{:.3}\t", ( *asa_list )[ i ]         );
			print!( "{:.3}\t", ( *rsa_list )[ i ] * 100.0 );
			colorize_state( ( *state_list )[ i ]          );
			print!( "\n"                                  );
		}
	} else {
		print!( "Number\t"  );
		print!( "ChainID\t" );
		print!( "ResSeq\t"  );
		print!( "ResName\t" );
		print!( "ResASA\t"  );
		print!( "ResRSA\t"  );
		print!( "State"     );
		print!( "\n"        );

		for i in 0 .. ( *rsa_list ).len() {
			print!( "{}\t"   , i + 1                          );
			print!( "{}\t"   , ( *chainid_list )[ i ]         );
			print!( "{}\t"   , ( *resseq_list  )[ i ]         );
			print!( "{}\t"   , ( *resname_list )[ i ]         );
			print!( "{:.3}\t", ( *asa_list     )[ i ]         );
			print!( "{:.3}\t", ( *rsa_list     )[ i ] * 100.0 );
			print!( "{}"     , ( *state_list   )[ i ]         );
			print!( "\n"                                      );
		}
	}

}

pub fn save_result(
	chainid_list : &Vec<&str>,
	resseq_list  : &Vec<usize>,
	resname_list : &Vec<&str>,
	asa_list     : &Vec<f64>,
	rsa_list     : &Vec<f64>,
	state_list   : &Vec<&str>,
	arg_o        : &String
) {

	let fout_name : String = arg_o.to_owned() + &".rsa".to_string();
	let mut fout = File::create( fout_name.as_str() ).expect( "FAILED to open output file" );

	//writeln!( fout, "{}", "num\tcons\tsite" ).expect( "FAILED to write" );
	write!( fout, "{}", "Number\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "ChainID\t" ).expect( "FAILED to write" );
	write!( fout, "{}", "ResSeq\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "ResName\t" ).expect( "FAILED to write" );
	write!( fout, "{}", "ResASA\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "ResRSA\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "State"     ).expect( "FAILED to write" );
	write!( fout, "{}", "\n"        ).expect( "FAILED to write" );

	for i in 0 .. ( *rsa_list ).len() {
		write!( fout, "{}\t"   , i + 1                          ).expect( "FAILED to write" );
		write!( fout, "{}\t"   , ( *chainid_list )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{}\t"   , ( *resseq_list  )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{}\t"   , ( *resname_list )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{:.3}\t", ( *asa_list     )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{:.3}\t", ( *rsa_list     )[ i ] * 100.0 ).expect( "FAILED to write" );
		write!( fout, "{}"     , ( *state_list   )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "\n"                                      ).expect( "FAILED to write" );
	}

	println!( "\nThe Voronota output file was correctly written.\n" );
}

fn colorize_state( state : &str ) {

	match state {
		"buried"       => print!( "{}", state.red()   ),
		"intermediate" => print!( "{}", state.green() ),
		"exposed"      => print!( "{}", state.cyan()  ),
		_              => print!( "{}", state         ),
	}

}

fn colorize_res( residue : &str ) {

	//let sequence : Vec<char> = ( *arg ).chars().collect();
	//println!("{:?}", sequence);

	match residue {
		"ALA" | "VAL" | "LEU" | "ILE" | "MET" | "CYS" => print!( "{}\t", residue.on_yellow().black()  ),
		"PHE" | "TRP" | "TYR" | "HIS"                 => print!( "{}\t", residue.on_cyan().black()    ),
		"SER" | "THR" | "ASN" | "GLN"                 => print!( "{}\t", residue.on_green().black()   ),
		"LYS" | "ARG"                                 => print!( "{}\t", residue.on_blue().black()    ),
		"ASP" | "GLU"                                 => print!( "{}\t", residue.on_red().black()     ),
		"GLY" | "PRO"                                 => print!( "{}\t", residue.on_magenta().black() ),
		_                                             => print!( "{}\t", residue                      ),
	}

}
