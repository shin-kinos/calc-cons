
mod mmcif;
mod neighboring;
mod options;
mod result;

fn main() {

	println!( "\nDefine neighboring residues by calculating distances between the alpha carbons in each residue.\n" );

	/* Set options. */
	let opts = options::Options::new();
	opts.show_parameter();

	/* Read mmCIF file and get information. */
	let mut data = mmcif::MmCif::new();
	data.read_cif_info( &( opts.input ) );
	//println!( "{:?}", data.category_list );

	/*
	for i in 0 .. ( data.atom_site_data ).len() {
		println!( "{:?}", ( data.atom_site_data )[ i ] );
	}
	*/

	/* Read "_atom_site" category and set it's order. */
	data.define_category_order();
	//println!( "{:?}", data.category_order );

	/* Get ATOM and HETATM record line. */
	data.set_data_vector();

	/* Remove "label_alt_id". */
	data.remove_alt_id();

	/* Remove HETATM record line. */
	data.remove_hetatm();

	/*
	print!( "group_PDB\t"     );
	print!( "label_atom_id\t" );
	print!( "label_alt_id\t"  );
	print!( "label_comp_id\t" );
	print!( "label_asym_id\t" );
	print!( "auth_comp_id\t"  );
	print!( "auth_asym_id\t"  );
	print!( "auth_atom_id\t"  );
	print!( "cartn_x\t"       );
	print!( "cartn_y\t"       );
	print!( "cartn_z"         );
	print!( "\n" );
	for i in 0 .. ( data.group_pdb ).len() {
		print!( "{}\t", data.group_pdb    [ i ] );
		print!( "{}\t", data.label_atom_id[ i ] );
		print!( "{}\t", data.label_alt_id [ i ] );
		print!( "{}\t", data.label_comp_id[ i ] );
		print!( "{}\t", data.label_asym_id[ i ] );
		print!( "{}\t", data.auth_comp_id [ i ] );
		print!( "{}\t", data.auth_asym_id [ i ] );
		print!( "{}\t", data.auth_atom_id [ i ] );
		print!( "{}\t", data.cartn_x      [ i ] );
		print!( "{}\t", data.cartn_y      [ i ] );
		print!( "{}"  , data.cartn_z      [ i ] );
		print!( "\n" );
	}
	*/

	/* Get coordinate information of CA in each residue. */
	let (
		res_name_list, // : Vec<String>;
		res_num_list,  // : Vec<isize>;
		coord_x_list,  // : Vec<f64>;
		coord_y_list,  // : Vec<f64>;
		coord_z_list   // : Vec<f64>;
	) = data.get_coord_info( &( opts.author ), &( opts.chainid ) );

	/*
	for i in 0 .. res_name_list.len() {
		print!( "{}{}\t{:.3}\t{:.3}\t{:.3}",
			res_name_list[ i ],
			res_num_list[ i ],
			coord_x_list[ i ],
			coord_y_list[ i ],
			coord_z_list[ i ]
		);
		print!( "\n" );
	}
	*/

	/* Show and save result. */
	result::show_save_result(
		&res_name_list,
		&res_num_list,
		&coord_x_list,
		&coord_y_list,
		&coord_z_list,
		&( opts.output )
	);

	println!( "\nProgram completed !!!\n" );

}
