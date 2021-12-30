
use std::time::Instant;
use colored::*;

mod cif_convert;
mod cif_mmcif;
mod cons_assign;
mod cons_bgdist;
mod cons_entropy;
mod cons_fasta;
mod cons_gap;
mod cons_remove;
mod cons_result;
mod cons_weighting;
mod voro_access;
mod voro_voronota;
mod voro_remove;
mod voro_result;

mod combine;
mod error;
mod options;
mod result;

fn main() {

	/* Elapsed time : Start */
	let start = Instant::now();

	/* Set options. */
	let opts = options::Options::new();

	/* Show parameter set. */
	opts.show_parameter();


	////////////////////////////////////////////////////////////////////////////////////
	//                         STRUCTURAL INFORMATION PARSING                         //
	////////////////////////////////////////////////////////////////////////////////////


	/* Read mmCIF file and get information. */
	let mut cif_data = cif_mmcif::MmCif::new();
	cif_data.read_cif_info( &( opts.cif_input ) );
	//println!( "{:?}", cif_data.category_list );

	/*
	for i in 0 .. ( cif_data.atom_site_data ).len() {
		println!( "{:?}", ( cif_data.atom_site_data )[ i ] );
	}
	*/

	/* Read "_atom_site" category and set it's order. */
	cif_data.define_category_order();
	//println!( "{:?}", data.category_order );

	/* Get ATOM and HETATM record line. */
	cif_data.set_data_vector();

	/* Remove "label_alt_id". */
	cif_data.remove_alt_id();

	/* Remove HETATM record line. */
	cif_data.remove_hetatm();

	/*
	print!( "group_PDB\t"     );
	print!( "label_atom_id\t" );
	print!( "label_alt_id\t"  );
	print!( "label_comp_id\t" );
	print!( "label_asym_id\t" );
	print!( "auth_comp_id\t"  );
	print!( "auth_asym_id\t"  );
	print!( "auth_atom_id\t"  );
	print!( "Cartn_x\t"       );
	print!( "Cartn_y\t"       );
	print!( "Cartn_z"         );
	print!( "\n" );
	for i in 0 .. ( cif_data.group_pdb ).len() {
		print!( "{}\t", ( cif_data.group_pdb )[ i ]     );
		print!( "{}\t", ( cif_data.label_atom_id )[ i ] );
		print!( "{}\t", ( cif_data.label_alt_id )[ i ]  );
		print!( "{}\t", ( cif_data.label_comp_id )[ i ] );
		print!( "{}\t", ( cif_data.label_asym_id )[ i ] );
		print!( "{}\t", ( cif_data.auth_comp_id )[ i ]  );
		print!( "{}\t", ( cif_data.auth_asym_id )[ i ]  );
		print!( "{}\t", ( cif_data.auth_atom_id )[ i ]  );
		print!( "{}\t", ( cif_data.cartn_x )[ i ]       );
		print!( "{}\t", ( cif_data.cartn_y )[ i ]       );
		print!( "{}"  , ( cif_data.cartn_z )[ i ]       );
		print!( "\n" );
	}
	*/

	/* Get sequence and coordinate information of CA in each residue. */
	let (
		res_name_list, // : Vec<String>;
		_res_num_list,  // : Vec<isize>;
		_coord_x_list,  // : Vec<f64>;
		_coord_y_list,  // : Vec<f64>;
		_coord_z_list   // : Vec<f64>;
	) = cif_data.get_res_coord_info( &( opts.cif_author ), &( opts.cif_chainid ) );

	/*
	for i in 0 .. res_name_list.len() {
		print!( "{}{}\t{:.3}\t{:.3}\t{:.3}",
			res_name_list[ i ],
			res_num_list[ i ],
			_coord_x_list[ i ],
			_coord_y_list[ i ],
			_coord_z_list[ i ]
		);
		print!( "\n" );
	}
	*/

	/* Convert the residues from 3 letter into 1 letter to get the "target sequence". */
	let mut target_sequence : Vec<char> = cif_convert::convert_residue( &res_name_list );
	//println!( "Target sequence : \n{:?}", target_sequence );

	////////////////////////////////////////////////////////////////////////////////////
	//                              SCORING CONSERVATION                              //
	////////////////////////////////////////////////////////////////////////////////////

	/* Read an input file and get FASTA information. */
	let mut cons_data = cons_fasta::Fasta::new();
	cons_data.read_fasta_info( &( opts.cons_input ) );

	/* Check whether the input file is correct FASTA format. */
	cons_data.check_fasta_info( &( opts.cons_tolerate ) );

	/* Get site information as Vec<String>. */
	cons_data.get_site_list();

	/* Show sequence and site number information. */
	if opts.quiet == "no".to_string() {
		println!( "\nNumber of the sequences : {}", ( cons_data.seq_list ).len() );
		println!( "Number of the sites     : {}", ( cons_data.site_list ).len()  );
	}

	/*
	println!( "\nInputfile content :\n" );
	for i in 0 .. ( cons_data.seq_list ).len() {
		println!( "Title    {} : {}", i + 1, ( cons_data.title_list )[ i ] );
		println!( "Sequence {} : {}", i + 1, ( cons_data.seq_list )[ i ]   );
	}
	*/

	/*
	println!( "\nSite content :\n" );
	for i in 0 .. ( cons_data.site_list ).len() {
		println!( "Site {} : {}", i + 1, ( cons_data.site_list )[ i ] );
	}
	*/

	/*
	 * Check whether the input MSA contains the target assigned sequence.
	 * If checked, the index of the assigned sequence in the MSA is memorized.
	 * If not checked, program halts with an error massage.
	 */
	let target_seq_index : usize = cif_convert::targetseq_exist_check( &( cons_data.seq_list ), &target_sequence );
	target_sequence.clear(); target_sequence.shrink_to_fit();

	if opts.quiet == "no".to_string() {
		println!( "\nThe order of the assigned sequence in the input MSA : {}", target_seq_index + 1 );
	}

	let mut cons_weight_list : Vec<f64> = cons_weighting::seq_weight(
		&( cons_data.seq_list ),
		&( cons_data.site_list ),
		&( opts.cons_weight )
	);

	/*
	println!( "\nSequence weighting :\n" );
	for i in 0 .. cons_weight_list.len() {
		println!( "Weight of Sequence {} : {:.3}", i + 1, cons_weight_list[ i ] );
	}
	*/

	if opts.quiet == "no".to_string() {
		let sum_weight : f64 = ( cons_weight_list ).iter().sum();
		println!( "\nSum of the weighting factors : {:.3}", sum_weight );
	}

	/* Calculate gap penalties taking acconut of sequence weighting. */
	let mut cons_gap_pen_list : Vec<f64> = cons_gap::weight_gap_penalty( &( cons_data.site_list ), &cons_weight_list );

	/*
	for i in 0 .. cons_gap_pen_list.len() {
		println!( "Gap penalty of site {} : {:.4}", i + 1, cons_gap_pen_list[ i ] );
	}
	*/

	let mut cons_capra07_list : Vec<f64> = cons_entropy::js_divergence(
		&( cons_data.site_list ),
		&cons_weight_list,
		&cons_gap_pen_list,
		&( opts.cons_bgdist )
	);
	cons_weight_list.clear(); cons_weight_list.shrink_to_fit();
	cons_gap_pen_list.clear(); cons_gap_pen_list.shrink_to_fit();

	/*
	for i in 0 .. cons_capra07_list.len() {
		println!( "Jensen-Shannon divergence site {} : {:.3}", i + 1, cons_capra07_list[ i ] );
	}
	*/

	/* Assign conservation scores to the target sequence's residues. */
	let mut target_seq_3_letters : Vec<String> = cons_assign::detect_assigned_sequence(
		//&( cons_data.title_list ),
		&( cons_data.seq_list ),
		target_seq_index
	);

	/*
	for i in 0 .. target_seq_3_letters.len() {
		println!( "{}\t{}", i + 1, target_seq_3_letters[ i ] );
	}
	*/

	/* Remove conservation scores assigned to gap. */
	cons_remove::remove_gap( &mut cons_capra07_list, &mut target_seq_3_letters );

	/*
	for i in 0 .. target_seq_3_letters.len() {
		println!( "{}\t{:.3}\t{}", i + 1, cons_capra07_list[ i ], target_seq_3_letters[ i ] );
	}
	*/

	/* Show conservation result. */
	if opts.quiet == "no".to_string() {
		cons_result::show_result(
			&cons_capra07_list,
			&target_seq_3_letters,
			&( opts.colorize )
		);
	}

	/* Save conservation result. */
	cons_result::save_result(
		&cons_capra07_list,
		&target_seq_3_letters,
		&( opts.output )
	);

	////////////////////////////////////////////////////////////////////////////////////
	//                               VORONOTA EXECUTION                               //
	////////////////////////////////////////////////////////////////////////////////////

	/* Set Vectors for Voronota. */
	let mut voro_data = voro_voronota::Voronota::new();

	/* Execute Voronota. */
	voro_voronota::exe_voronota(
		&( opts.cif_input  ).as_str(),
		&( opts.output ).as_str()
		//&( opts.format ).as_str()
	);

	/* Check Voronota data. */
	voro_data.check_voronota_data();

	/* Read Voronota output and Get infomation. */
	voro_data.read_voronota_info( &( opts.output ).as_str() );

	/* Make ASA list a residue. */
	let (
		mut voro_chainid_list, // : Vec<&str>
		mut voro_resseq_list , // : Vec<usize>
		mut voro_resname_list, // : Vec<&str>
		mut voro_asa_list      // : Vec<f64>
	)
	= voro_data.make_asa_list();

	/* Remove the rest chain ID data. */
	voro_remove::remove_other_chains(
		&mut voro_chainid_list,    // : Vec<&str>
		&mut voro_resseq_list ,    // : Vec<usize>
		&mut voro_resname_list,    // : Vec<&str>
		&mut voro_asa_list,        // : Vec<f64>
		&( opts.cif_chainid ) // : String
	);

	/*
	for i in 0 .. voro_chainid_list.len() {
		print!( "{}\t", voro_chainid_list[ i ] );
		print!( "{}\t", voro_resseq_list[ i ]  );
		print!( "{}\t", voro_resname_list[ i ] );
		print!( "{}\t", voro_asa_list[ i ]     );
		print!( "\n"                           );
	}
	*/

	/* Calculate RSA. */
	let voro_rsa_list : Vec<f64> = voro_access::calc_rsa( &voro_resname_list, &voro_asa_list );

	let voro_state_list : Vec<&str> = voro_access::detect_state( &voro_rsa_list );

	/*
	for i in 0 .. voro_rsa_list.len() {
		print!( "{}\t"   , voro_chainid_list[ i ]     );
		print!( "{}\t"   , voro_resseq_list[ i ]      );
		print!( "{}\t"   , voro_resname_list[ i ]     );
		print!( "{:.3}\t", voro_asa_list[ i ]         );
		print!( "{:.3}\t", voro_rsa_list[ i ] * 100.0 );
		print!( "{}\t"   , voro_state_list[ i ]       );
		print!( "\n"                                  );
	}
	*/

	/* Show RSA result. */
	if opts.quiet == "no".to_string() {
		voro_result::show_result(
			&voro_chainid_list,
			&voro_resseq_list,
			&voro_resname_list,
			&voro_asa_list,
			&voro_rsa_list,
			&voro_state_list,
			&( opts.colorize )
		);
	}

	/* Save RSA result. */
	voro_result::save_result(
		&voro_chainid_list,
		&voro_resseq_list,
		&voro_resname_list,
		&voro_asa_list,
		&voro_rsa_list,
		&voro_state_list,
		&( opts.output )
	);

	////////////////////////////////////////////////////////////////////////////////////
	//                  CONSERVATION-VORONOTA RESULT DATA COMBINATION                 //
	////////////////////////////////////////////////////////////////////////////////////

	/* Check whether or not the composition of the assigned sequence is same as the Voronota output's. */
	combine::check_cons_voro_res( &target_seq_3_letters, &voro_resname_list );

	/* Show the whole result. */
	if opts.quiet == "no".to_string() {
		result::show_result(
			&cons_capra07_list,
			&target_seq_3_letters,
			&voro_chainid_list,
			&voro_resseq_list,
			&voro_resname_list,
			&voro_asa_list,
			&voro_rsa_list,
			&voro_state_list,
			&( opts.colorize )
		);
	}

	combine::combine_cons_voro( &( opts.output ) );

	println!( "{}", "\nProgram completed !!!\n".green() );

	/* Elapsed time : End */
	let end = start.elapsed();
	println!( "Total elapsed time : {:?}", end );

}
