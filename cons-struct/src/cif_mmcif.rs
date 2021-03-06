
use std::fs::read_to_string;

use crate::error;

pub struct MmCif {
	pub group_pdb          : Vec<String>,
	//pub id                 : Vec<String>,
	//pub type_symbol        : Vec<String>,
	pub label_atom_id      : Vec<String>,
	pub label_alt_id       : Vec<String>,
	pub label_comp_id      : Vec<String>,
	pub label_asym_id      : Vec<String>,
	//pub label_entity_id    : Vec<String>,
	pub label_seq_id       : Vec<String>,
	//pub pdbx_pdb_ins_code  : Vec<String>,
	pub cartn_x            : Vec<String>,
	pub cartn_y            : Vec<String>,
	pub cartn_z            : Vec<String>,
	//pub occupancy          : Vec<String>,
	//pub b_iso_or_equiv     : Vec<String>,
	//pub pdbx_formal_charge : Vec<String>,
	pub auth_seq_id        : Vec<String>,
	pub auth_comp_id       : Vec<String>,
	pub auth_asym_id       : Vec<String>,
	pub auth_atom_id       : Vec<String>,
	//pub pdbx_pdb_model_num : Vec<String>,

	pub atom_site_data     : Vec<String>,
	pub category_list      : Vec<String>,
	pub category_order     : Vec<usize>,
}

impl MmCif {
	pub fn new() -> MmCif {
		let group_pdb          : Vec<String> = Vec::new();
		//let id                 : Vec<String> = Vec::new();
		//let type_symbol        : Vec<String> = Vec::new();
		let label_atom_id      : Vec<String> = Vec::new();
		let label_alt_id       : Vec<String> = Vec::new();
		let label_comp_id      : Vec<String> = Vec::new();
		let label_asym_id      : Vec<String> = Vec::new();
		//let label_entity_id    : Vec<String> = Vec::new();
		let label_seq_id       : Vec<String> = Vec::new();
		//let pdbx_pdb_ins_code  : Vec<String> = Vec::new();
		let cartn_x            : Vec<String> = Vec::new();
		let cartn_y            : Vec<String> = Vec::new();
		let cartn_z            : Vec<String> = Vec::new();
		//let occupancy          : Vec<String> = Vec::new();
		//let b_iso_or_equiv     : Vec<String> = Vec::new();
		//let pdbx_formal_charge : Vec<String> = Vec::new();
		let auth_seq_id        : Vec<String> = Vec::new();
		let auth_comp_id       : Vec<String> = Vec::new();
		let auth_asym_id       : Vec<String> = Vec::new();
		let auth_atom_id       : Vec<String> = Vec::new();
		//let pdbx_pdb_model_num : Vec<String> = Vec::new();

		let atom_site_data     : Vec<String> = Vec::new();
		let category_list      : Vec<String> = Vec::new();
		let category_order     : Vec<usize>  = Vec::new();

		MmCif {
			group_pdb          : group_pdb,
			//id                 : id,
			//type_symbol        : type_symbol,
			label_atom_id      : label_atom_id,
			label_alt_id       : label_alt_id,
			label_comp_id      : label_comp_id,
			label_asym_id      : label_asym_id,
			//label_entity_id    : label_entity_id,
			label_seq_id       : label_seq_id,
			//pdbx_pdb_ins_code  : pdbx_pdb_ins_code,
			cartn_x            : cartn_x,
			cartn_y            : cartn_y,
			cartn_z            : cartn_z,
			//occupancy          : occupancy,
			//b_iso_or_equiv     : b_iso_or_equiv,
			//pdbx_formal_charge : pdbx_formal_charge,
			auth_seq_id        : auth_seq_id,
			auth_comp_id       : auth_comp_id,
			auth_asym_id       : auth_asym_id,
			auth_atom_id       : auth_atom_id,
			//pdbx_pdb_model_num : pdbx_pdb_model_num,

			atom_site_data     : atom_site_data,
			category_list      : category_list,
			category_order     : category_order,
		}
	}

	pub fn read_cif_info( &mut self, arg_i : &String ) {

		let fin = read_to_string( ( *arg_i ).as_str() ).expect( "FAILED to open input file" );

		let mut _category_start : bool = false;

		for line in fin.lines() {
			if line.starts_with( "_atom_site." ) {
				_category_start = true;
				( self.category_list ).push( line.to_string() );
			}
			if _category_start == true {
				if !line.starts_with( "_atom_site." ) && !line.starts_with( "#" ) {
					( self.atom_site_data ).push( line.to_string() );
				} else if line.starts_with( "#" ) {
					break;
				}
			}
		}

	}

	pub fn define_category_order( &mut self ) {

		self.category_order = vec![ 0; 13 ];

		for i in 0 .. ( self.category_list ).len() {
			if      ( self.category_list )[ i ].starts_with( "_atom_site.group_PDB"     ) { ( self.category_order )[  0 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.label_atom_id" ) { ( self.category_order )[  1 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.label_alt_id"  ) { ( self.category_order )[  2 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.label_comp_id" ) { ( self.category_order )[  3 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.label_asym_id" ) { ( self.category_order )[  4 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.label_seq_id"  ) { ( self.category_order )[  5 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.auth_comp_id"  ) { ( self.category_order )[  6 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.auth_asym_id"  ) { ( self.category_order )[  7 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.auth_atom_id"  ) { ( self.category_order )[  8 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.auth_seq_id"   ) { ( self.category_order )[  9 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.Cartn_x"       ) { ( self.category_order )[ 10 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.Cartn_y"       ) { ( self.category_order )[ 11 ] = i; }
			else if ( self.category_list )[ i ].starts_with( "_atom_site.Cartn_z"       ) { ( self.category_order )[ 12 ] = i; }
		}

	}

	pub fn set_data_vector( &mut self ) {
		//let data_len : usize = ( self.atom_site_data ).len();

		for line in ( self.atom_site_data ).iter() {
			let compornents : Vec<&str> = line.split_whitespace().collect();
			( self.group_pdb     ).push( compornents[ ( self.category_order )[  0 ] ].to_string() );
			( self.label_atom_id ).push( compornents[ ( self.category_order )[  1 ] ].to_string() );
			( self.label_alt_id  ).push( compornents[ ( self.category_order )[  2 ] ].to_string() );
			( self.label_comp_id ).push( compornents[ ( self.category_order )[  3 ] ].to_string() );
			( self.label_asym_id ).push( compornents[ ( self.category_order )[  4 ] ].to_string() );
			( self.label_seq_id  ).push( compornents[ ( self.category_order )[  5 ] ].to_string() );
			( self.auth_comp_id  ).push( compornents[ ( self.category_order )[  6 ] ].to_string() );
			( self.auth_asym_id  ).push( compornents[ ( self.category_order )[  7 ] ].to_string() );
			( self.auth_atom_id  ).push( compornents[ ( self.category_order )[  8 ] ].to_string() );
			( self.auth_seq_id   ).push( compornents[ ( self.category_order )[  9 ] ].to_string() );
			( self.cartn_x       ).push( compornents[ ( self.category_order )[ 10 ] ].to_string() );
			( self.cartn_y       ).push( compornents[ ( self.category_order )[ 11 ] ].to_string() );
			( self.cartn_z       ).push( compornents[ ( self.category_order )[ 12 ] ].to_string() );
		}
	}

	pub fn remove_alt_id( &mut self ) {

		let mut vec_len : usize = ( self.group_pdb ).len();
		let mut i       : usize = 0;

		while i < vec_len {
			if ( self.label_alt_id )[ i ] != "." && ( self.label_alt_id )[ i ] != "A" {
				( self.group_pdb     ).remove( i );
				( self.label_atom_id ).remove( i );
				( self.label_alt_id  ).remove( i );
				( self.label_comp_id ).remove( i );
				( self.label_asym_id ).remove( i );
				( self.label_seq_id  ).remove( i );
				( self.auth_comp_id  ).remove( i );
				( self.auth_asym_id  ).remove( i );
				( self.auth_atom_id  ).remove( i );
				( self.auth_seq_id   ).remove( i );
				( self.cartn_x       ).remove( i );
				( self.cartn_y       ).remove( i );
				( self.cartn_z       ).remove( i );
				vec_len -= 1;
			} else {
				i += 1;
			}
		}

		( self.group_pdb     ).shrink_to_fit();
		( self.label_atom_id ).shrink_to_fit();
		( self.label_alt_id  ).shrink_to_fit();
		( self.label_comp_id ).shrink_to_fit();
		( self.label_asym_id ).shrink_to_fit();
		( self.label_seq_id  ).shrink_to_fit();
		( self.auth_comp_id  ).shrink_to_fit();
		( self.auth_asym_id  ).shrink_to_fit();
		( self.auth_atom_id  ).shrink_to_fit();
		( self.auth_seq_id   ).shrink_to_fit();
		( self.cartn_x       ).shrink_to_fit();
		( self.cartn_y       ).shrink_to_fit();
		( self.cartn_z       ).shrink_to_fit();

		if ( self.label_atom_id ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_atom_id_group_pdb_not_same" ); }
		if ( self.label_alt_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_alt_id_group_pdb_not_same"  ); }
		if ( self.label_comp_id ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_comp_id_group_pdb_not_same" ); }
		if ( self.label_asym_id ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_asym_id_group_pdb_not_same" ); }
		if ( self.label_seq_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_seq_id_group_pdb_not_same"  ); }
		if ( self.auth_comp_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_comp_id_group_pdb_not_same"  ); }
		if ( self.auth_asym_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_asym_id_group_pdb_not_same"  ); }
		if ( self.auth_atom_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_atom_id_group_pdb_not_same"  ); }
		if ( self.auth_seq_id   ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_seq_id_group_pdb_not_same"   ); }
		if ( self.cartn_x       ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_cartn_x_group_pdb_not_same"       ); }
		if ( self.cartn_y       ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_cartn_y_group_pdb_not_same"       ); }
		if ( self.cartn_z       ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_cartn_z_group_pdb_not_same"       ); }

		/*
		assert_eq!( ( self.label_atom_id ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_alt_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_comp_id ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_asym_id ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_seq_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_comp_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_asym_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_atom_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_seq_id   ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.cartn_x       ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.cartn_y       ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.cartn_z       ).len(), ( self.group_pdb ).len() );
		*/

	}

	pub fn remove_hetatm( &mut self ) {

		let mut vec_len : usize = ( self.group_pdb ).len();
		let mut i       : usize = 0;

		while i < vec_len {
			if ( self.group_pdb )[ i ] == "HETATM" {
				( self.group_pdb     ).remove( i );
				( self.label_atom_id ).remove( i );
				( self.label_alt_id  ).remove( i );
				( self.label_comp_id ).remove( i );
				( self.label_asym_id ).remove( i );
				( self.label_seq_id  ).remove( i );
				( self.auth_comp_id  ).remove( i );
				( self.auth_asym_id  ).remove( i );
				( self.auth_atom_id  ).remove( i );
				( self.auth_seq_id   ).remove( i );
				( self.cartn_x       ).remove( i );
				( self.cartn_y       ).remove( i );
				( self.cartn_z       ).remove( i );
				vec_len -= 1;
			} else {
				i += 1;
			}
		}

		( self.group_pdb     ).shrink_to_fit();
		( self.label_atom_id ).shrink_to_fit();
		( self.label_alt_id  ).shrink_to_fit();
		( self.label_comp_id ).shrink_to_fit();
		( self.label_asym_id ).shrink_to_fit();
		( self.label_seq_id  ).shrink_to_fit();
		( self.auth_comp_id  ).shrink_to_fit();
		( self.auth_asym_id  ).shrink_to_fit();
		( self.auth_atom_id  ).shrink_to_fit();
		( self.auth_seq_id   ).shrink_to_fit();
		( self.cartn_x       ).shrink_to_fit();
		( self.cartn_y       ).shrink_to_fit();
		( self.cartn_z       ).shrink_to_fit();

		if ( self.label_atom_id ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_atom_id_group_pdb_not_same" ); }
		if ( self.label_alt_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_alt_id_group_pdb_not_same"  ); }
		if ( self.label_comp_id ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_comp_id_group_pdb_not_same" ); }
		if ( self.label_asym_id ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_asym_id_group_pdb_not_same" ); }
		if ( self.label_seq_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_label_seq_id_group_pdb_not_same"  ); }
		if ( self.auth_comp_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_comp_id_group_pdb_not_same"  ); }
		if ( self.auth_asym_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_asym_id_group_pdb_not_same"  ); }
		if ( self.auth_atom_id  ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_atom_id_group_pdb_not_same"  ); }
		if ( self.auth_seq_id   ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_auth_seq_id_group_pdb_not_same"   ); }
		if ( self.cartn_x       ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_cartn_x_group_pdb_not_same"       ); }
		if ( self.cartn_y       ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_cartn_y_group_pdb_not_same"       ); }
		if ( self.cartn_z       ).len() != ( self.group_pdb ).len() { error::error_bomb( "len_cartn_z_group_pdb_not_same"       ); }

		/*
		assert_eq!( ( self.label_atom_id ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_alt_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_comp_id ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_asym_id ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.label_seq_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_comp_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_asym_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_atom_id  ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.auth_seq_id   ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.cartn_x       ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.cartn_y       ).len(), ( self.group_pdb ).len() );
		assert_eq!( ( self.cartn_z       ).len(), ( self.group_pdb ).len() );
		*/

	}

	pub fn get_res_coord_info( &self, arg_a : &String, arg_c : &String ) -> (
		Vec<String>,
		Vec<isize>,
		Vec<f64>,
		Vec<f64>,
		Vec<f64>
	) {
		let (
			res_name_list,
			res_num_list,
			coord_x_list,
			coord_y_list,
			coord_z_list
		) = if arg_a == "no" {
			get_coord_ca(
				&( self.label_atom_id ),
				&( self.label_comp_id ),
				&( self.label_asym_id ),
				&( self.label_seq_id  ),
				&( self.cartn_x       ),
				&( self.cartn_y       ),
				&( self.cartn_z       ),
				arg_c
			)
		} else {
			get_coord_ca(
				&( self.auth_atom_id ),
				&( self.auth_comp_id ),
				&( self.auth_asym_id ),
				&( self.auth_seq_id  ),
				&( self.cartn_x      ),
				&( self.cartn_y      ),
				&( self.cartn_z      ),
				arg_c
			)
		};

		if res_num_list.len() != res_name_list.len() { error::error_bomb( "len_res_num_list_res_name_list_not_same" ); }
		if coord_x_list.len() != res_name_list.len() { error::error_bomb( "len_coord_x_list_res_name_list_not_same" ); }
		if coord_y_list.len() != res_name_list.len() { error::error_bomb( "len_coord_y_list_res_name_list_not_same" ); }
		if coord_z_list.len() != res_name_list.len() { error::error_bomb( "len_coord_z_list_res_name_list_not_same" ); }

		/*
		assert_eq!( res_num_list.len(), res_name_list.len() );
		assert_eq!( coord_x_list.len(), res_name_list.len() );
		assert_eq!( coord_y_list.len(), res_name_list.len() );
		assert_eq!( coord_z_list.len(), res_name_list.len() );
		*/

	(
		res_name_list,
		res_num_list,
		coord_x_list,
		coord_y_list,
		coord_z_list
	)

	}

}

fn get_coord_ca(
	atom_id_vec     : &Vec<String>,
	residue_id_vec  : &Vec<String>,
	chain_id_vec    : &Vec<String>,
	residue_seq_vec : &Vec<String>,
	cartn_x_vec     : &Vec<String>,
	cartn_y_vec     : &Vec<String>,
	cartn_z_vec     : &Vec<String>,
	arg_c           : &String
) -> (
	Vec<String>,
	Vec<isize>,
	Vec<f64>,
	Vec<f64>,
	Vec<f64>
) {

	assert_eq!( true, ( *chain_id_vec ).contains( arg_c ), "No such residue chain ID in the input mmCIF file : {}", arg_c );

	let mut res_name_list : Vec<String> = Vec::new();
	let mut res_num_list  : Vec<isize>  = Vec::new();
	let mut coord_x_list  : Vec<f64>    = Vec::new();
	let mut coord_y_list  : Vec<f64>    = Vec::new();
	let mut coord_z_list  : Vec<f64>    = Vec::new();

	let vec_len : usize = ( *atom_id_vec ).len();

	for i in 0 .. vec_len {
		if ( *arg_c ) == ( *chain_id_vec )[ i ] && ( *atom_id_vec )[ i ] == "CA" {
			res_name_list.push( ( ( *residue_id_vec  )[ i ] ).to_string()               );
			res_num_list .push( ( ( *residue_seq_vec )[ i ] ).parse::<isize>().unwrap() );
			coord_x_list .push( ( ( *cartn_x_vec     )[ i ] ).parse::<f64>().unwrap()   );
			coord_y_list .push( ( ( *cartn_y_vec     )[ i ] ).parse::<f64>().unwrap()   );
			coord_z_list .push( ( ( *cartn_z_vec     )[ i ] ).parse::<f64>().unwrap()   );
		}
	}

	(
		res_name_list,
		res_num_list,
		coord_x_list,
		coord_y_list,
		coord_z_list
	)
}
