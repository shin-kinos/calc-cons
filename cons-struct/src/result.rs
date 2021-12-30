
use colored::*;

pub fn show_result(
	/* Conservation result */
	cons_list     : &Vec<f64>,
	targ_res_list : & Vec<String>,
	/* Voronota result */
	chainid_list  : &Vec<&str>,
	resseq_list   : &Vec<usize>,
	resname_list  : &Vec<&str>,
	asa_list      : &Vec<f64>,
	rsa_list      : &Vec<f64>,
	state_list    : &Vec<&str>,
	/* colorize argument */
	arg_c         : &String
) {

	println!( "\nResult : \n" );

	if ( *arg_c ) == "yes".to_string() {
		print!( "Number\t"  );
		print!( "Cons\t"    );
		print!( "ConsRes\t" );
		print!( "ChainID\t" );
		print!( "ResSeq\t"  );
		print!( "ResName\t" );
		print!( "ResASA\t"  );
		print!( "ResRSA\t"  );
		print!( "State"     );
		print!( "\n"        );

		for i in 0 .. ( *rsa_list ).len() {
			print!( "{}\t", i + 1                          );
			print!( "{:.3}\t", ( *cons_list )[ i ]         );
			colorize_res( ( *targ_res_list )[ i ].as_str() );
			print!( "{}\t", ( *chainid_list )[ i ]         );
			print!( "{}\t", ( *resseq_list  )[ i ]         );
			colorize_res( resname_list[ i ]                );
			print!( "{:.3}\t", ( *asa_list )[ i ]          );
			print!( "{:.3}\t", ( *rsa_list )[ i ] * 100.0  );
			colorize_state( ( *state_list )[ i ]           );
			print!( "\n"                                   );
		}
	} else {
		print!( "Number\t"  );
		print!( "ChainID\t" );
		print!( "Cons\t"    );
		print!( "ConsRes\t" );
		print!( "ResSeq\t"  );
		print!( "ResName\t" );
		print!( "ResASA\t"  );
		print!( "ResRSA\t"  );
		print!( "State"     );
		print!( "\n"        );

		for i in 0 .. ( *rsa_list ).len() {
			print!( "{}\t"   , i + 1                          );
			print!( "{:.3}\t", ( *cons_list )[ i ]            );
			print!( "{}\t", ( *targ_res_list )[ i ]             );
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
