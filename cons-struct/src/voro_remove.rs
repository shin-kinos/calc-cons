
use crate::error;

pub fn remove_other_chains(
	chainid_list : &mut Vec<&str>,
	resseq_list  : &mut Vec<usize>,
	resname_list : &mut Vec<&str>,
	asa_list     : &mut Vec<f64>,
	chainid      : &String
) {

	if ( *resseq_list ).len() != ( *chainid_list ).len() { error::error_bomb( "len_resseq_list_chainid_list_not_same"  ); }
	if ( *resname_list).len() != ( *chainid_list ).len() { error::error_bomb( "len_resname_list_chainid_list_not_same" ); }
	if ( *asa_list    ).len() != ( *chainid_list ).len() { error::error_bomb( "len_asa_list_chainid_list_not_same"     ); }

	let mut vec_len : usize = ( *resseq_list ).len();
	let mut i       : usize = 0;

	while i < vec_len {
		//println!( "{}", i );
		if ( *chainid_list )[ i ] != chainid.to_string() {
			( *chainid_list ).remove( i );
			( *resseq_list  ).remove( i );
			( *resname_list ).remove( i );
			( *asa_list     ).remove( i );
			vec_len -= 1;
		} else {
			i += 1;
		}
	}

	( *chainid_list ).shrink_to_fit();
	( *resseq_list  ).shrink_to_fit();
	( *resname_list ).shrink_to_fit();
	( *asa_list     ).shrink_to_fit();

	if ( *resseq_list ).len() != ( *chainid_list ).len() { error::error_bomb( "len_resseq_list_chainid_list_not_same"  ); }
	if ( *resname_list).len() != ( *chainid_list ).len() { error::error_bomb( "len_resname_list_chainid_list_not_same" ); }
	if ( *asa_list    ).len() != ( *chainid_list ).len() { error::error_bomb( "len_asa_list_chainid_list_not_same"     ); }

}
