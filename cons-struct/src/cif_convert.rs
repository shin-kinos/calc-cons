
use std::collections::HashMap;

use crate::error;

pub fn convert_residue( res_list_3 : &Vec<String> ) -> Vec<char> {

	let mut res_convert_index : HashMap<String, char> = HashMap::new();

	/* Set residue convert HashMap index. */
	res_convert_index.insert( String::from( "ALA" ), 'A' );
	res_convert_index.insert( String::from( "ARG" ), 'R' );
	res_convert_index.insert( String::from( "ASN" ), 'N' );
	res_convert_index.insert( String::from( "ASP" ), 'D' );
	res_convert_index.insert( String::from( "CYS" ), 'C' );
	res_convert_index.insert( String::from( "GLN" ), 'Q' );
	res_convert_index.insert( String::from( "GLU" ), 'E' );
	res_convert_index.insert( String::from( "GLY" ), 'G' );
	res_convert_index.insert( String::from( "HIS" ), 'H' );
	res_convert_index.insert( String::from( "ILE" ), 'I' );
	res_convert_index.insert( String::from( "LEU" ), 'L' );
	res_convert_index.insert( String::from( "LYS" ), 'K' );
	res_convert_index.insert( String::from( "MET" ), 'M' );
	res_convert_index.insert( String::from( "PHE" ), 'F' );
	res_convert_index.insert( String::from( "PRO" ), 'P' );
	res_convert_index.insert( String::from( "SER" ), 'S' );
	res_convert_index.insert( String::from( "THR" ), 'T' );
	res_convert_index.insert( String::from( "TRP" ), 'W' );
	res_convert_index.insert( String::from( "TYR" ), 'Y' );
	res_convert_index.insert( String::from( "VAL" ), 'V' );

	/* Convert AA list into one letter String. */
	let mut res_list_1 : Vec<char> = Vec::new();

	for residue in ( *res_list_3 ).iter() {
		res_list_1.push( res_convert_index[ residue ] );
	}

	/* Check whether ( length of res_list_3 ) == ( length of res_list_1 ). */
	assert_eq!( res_list_3.len(), res_list_1.len() );

	res_list_1
}

pub fn targetseq_exist_check( seq_list : &Vec<String>, target_sequence : &Vec<char> ) -> usize {

	let num_seq : usize = ( *seq_list ).len();
	let mut check : bool = false;

	let mut _target_seq_index = 0;

	for i in 0 .. num_seq {
		let seq_with_gap    : Vec<char> = ( seq_list[ i ].to_string() ).chars().collect();
		let seq_without_gap : Vec<char> = remove_gap( &seq_with_gap );
		//println!( "{:?}", seq_without_gap );
		if seq_without_gap == ( *target_sequence ) {
			//println!( "There is !" );
			check = true;
			_target_seq_index = i;
			break;
		}
	}

	if check == false { error::error_bomb( "no_such_seq_in_msa" ); }

	_target_seq_index

}

fn remove_gap( seq_with_gap : &Vec<char> ) -> Vec<char> {

	let seq_len : usize = ( *seq_with_gap ).len();

	let mut seq_without_gap : Vec<char> = Vec::new();

	for i in 0 .. seq_len {
		if ( *seq_with_gap )[ i ] != '-' {
			seq_without_gap.push( ( *seq_with_gap )[ i ] );
		}
	}

	seq_without_gap
}
