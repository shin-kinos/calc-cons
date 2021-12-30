
use crate::error;

pub fn remove_gap(
	cons_capra07_list    : &mut Vec<f64>,
	target_seq_3_letters : &mut Vec<String>
) {

	if ( *cons_capra07_list ).len() != ( *target_seq_3_letters ).len() { error::error_bomb( "error" ); }

	let mut vec_len : usize = ( *cons_capra07_list ).len();
	let mut i       : usize = 0;

	while i < vec_len {
		//println!( "{}", i );
		if ( *target_seq_3_letters )[ i ] == " - ".to_string() {
			( *cons_capra07_list    ).remove( i );
			( *target_seq_3_letters ).remove( i );
			vec_len -= 1;
		} else {
			i += 1;
		}
	}

	( *cons_capra07_list    ).shrink_to_fit();
	( *target_seq_3_letters ).shrink_to_fit();

	if ( *cons_capra07_list ).len() != ( *target_seq_3_letters ).len() { error::error_bomb( "error" ); }

}