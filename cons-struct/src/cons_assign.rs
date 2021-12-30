
use std::collections::HashMap;

//use crate::error;

pub fn detect_assigned_sequence(
	//title_list : &Vec<String>,
	seq_list        : &Vec<String>,
	taget_seq_index : usize
) -> Vec<String> {

	/*
	for title in title_list {
		println!( "{}", title );
	}
	*/

	/*
	let vec_len : usize = title_list.len();

	let mut title_line_vec : Vec<String> = vec![ ">".to_string() ];

	if !( *arg_a ).starts_with( ">" ) {
		//println!( "*arg_a does not starts with '>'." );
		title_line_vec.push( ( *arg_a ).to_string() );
	}

	let title_line : &String = &String::from( title_line_vec.concat() );

	if !( *title_list ).contains( title_line ) { error::error_bomb( "no_such_title_line" ); }

	let mut _title_line_index : usize = 0;
	for i in 0 .. vec_len {
		if *title_line == ( *title_list )[ i ] {
			_title_line_index = i;
		}
	}

	*/

	let target_seq : &String = &( *seq_list )[ taget_seq_index ];
	//println!( "Target sequence assigned : {:?}", target_seq );

	let target_seq_3_letters : Vec<String> = convert_letters( target_seq );
	/*
	for i in 0 .. seq_3_letters.len() {
		println!( "{}", seq_3_letters[ i ] );
	}
	*/

	target_seq_3_letters

}

fn convert_letters( target_seq : &String ) -> Vec<String> {

	let target_seq_char_vec : Vec<char> = ( *target_seq ).chars().collect();

	/* Set residue ( and gap ) convert HashMap index. */
	let mut res_convert_index : HashMap<char, String> = HashMap::new();
	res_convert_index.insert( 'A', String::from( "ALA" ) );
	res_convert_index.insert( 'R', String::from( "ARG" ) );
	res_convert_index.insert( 'N', String::from( "ASN" ) );
	res_convert_index.insert( 'D', String::from( "ASP" ) );
	res_convert_index.insert( 'C', String::from( "CYS" ) );
	res_convert_index.insert( 'Q', String::from( "GLN" ) );
	res_convert_index.insert( 'E', String::from( "GLU" ) );
	res_convert_index.insert( 'G', String::from( "GLY" ) );
	res_convert_index.insert( 'H', String::from( "HIS" ) );
	res_convert_index.insert( 'I', String::from( "ILE" ) );
	res_convert_index.insert( 'L', String::from( "LEU" ) );
	res_convert_index.insert( 'K', String::from( "LYS" ) );
	res_convert_index.insert( 'M', String::from( "MET" ) );
	res_convert_index.insert( 'F', String::from( "PHE" ) );
	res_convert_index.insert( 'P', String::from( "PRO" ) );
	res_convert_index.insert( 'S', String::from( "SER" ) );
	res_convert_index.insert( 'T', String::from( "THR" ) );
	res_convert_index.insert( 'W', String::from( "TRP" ) );
	res_convert_index.insert( 'Y', String::from( "TYR" ) );
	res_convert_index.insert( 'V', String::from( "VAL" ) );
	res_convert_index.insert( '-', String::from( " - " ) );

	/* Convert residues from 1 to 3 letters. */
	let mut residue_3_lettars : Vec<String> = Vec::new();

	for residue in target_seq_char_vec.iter() {
		//let test : &String = &res_convert_index[ residue ];
		residue_3_lettars.push( res_convert_index[ residue ].to_string() );
	}

	residue_3_lettars

}
