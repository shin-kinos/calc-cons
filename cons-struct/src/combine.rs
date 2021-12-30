
use std::process;
use std::process::Command;
use std::path::Path;

use rand::Rng;
use rand::distributions::Alphanumeric; // For ASCII randomization.
use colored::*;

pub fn check_cons_voro_res( target_seq_3_letters : &Vec<String>, voro_resname_list : &Vec<&str> ) {

	if ( *target_seq_3_letters ) != ( *voro_resname_list ) {
		println!( "{}", "\nWARNING !!!\n".yellow() );
		println!( "The assigned residues in scoring conservation and the Voronota output residues did NOT match. " );
		println!( "{}", "Conservation data and Voronota data were not combined in the output file.".yellow() );

		println!( "{}", "\nProgram completed !!!\n".green() );
		process::exit( 1 );
	}

}

pub fn combine_cons_voro( output_name : &String ) {

	/* Get tempfile name */
	let tempfile_name : String = get_tempfile_name( "tempfile" );

	/* Vonorota execution : Make balls data. */
	let mut combine_cmd_vec : Vec<&str> = Vec::new();

	combine_cmd_vec.push( "paste "                  );
	combine_cmd_vec.push( ( *output_name ).as_str() );
	combine_cmd_vec.push( " "                       );
	combine_cmd_vec.push( ( *output_name ).as_str() );
	combine_cmd_vec.push( ".rsa"                    );
	combine_cmd_vec.push( " > "                     );
	combine_cmd_vec.push( tempfile_name.as_str()    );

	//println!( "\ncombine_cons_voro command :\n% {}", combine_cmd_vec.concat() );

	let combine_cons_voro = Command::new( "sh" )
		.arg( "-c" )
		.arg( combine_cmd_vec.concat() )
		.status()
		.expect( "failed to execute process" );

	//println!( "combine_cons_voro : Process finished ( {} )\n", combine_cons_voro );
	assert!( combine_cons_voro.success() );

	/* Overwrite tempfile. */
	combine_cmd_vec.clear();

	combine_cmd_vec.push( "cat "                    );
	combine_cmd_vec.push( tempfile_name.as_str()    );
	combine_cmd_vec.push( " > "                     );
	combine_cmd_vec.push( ( *output_name ).as_str() );

	let overwrite = Command::new( "sh" )
	.arg( "-c" )
	.arg( combine_cmd_vec.concat() )
	.status()
	.expect( "failed to execute process" );

	assert!( overwrite.success() );

	/* Remove tempfile. */
	combine_cmd_vec.clear();

	combine_cmd_vec.push( "rm "                  );
	combine_cmd_vec.push( tempfile_name.as_str() );

	let rm_tempfile = Command::new( "sh" )
	.arg( "-c" )
	.arg( combine_cmd_vec.concat() )
	.status()
	.expect( "failed to execute process" );

	assert!( rm_tempfile.success() );

	println!( "\nThe conservation data and Voronota output data were correctly merged.\n" );

}

fn get_tempfile_name( prefix : &str ) -> String {

	let mut filename_vec : Vec<String> = vec![ prefix.to_string() ];

	filename_vec.push(
		rand::thread_rng()
		.sample_iter( &Alphanumeric )
		.take( 10 )
		.map( char::from )
		.collect()
	);

	while Path::new( filename_vec.concat().as_str() ).exists() {
		filename_vec.clear();
		filename_vec.push( prefix.to_string() );

		filename_vec.push(
			rand::thread_rng()
			.sample_iter( &Alphanumeric )
			.take( 10 )
			.map( char::from )
			.collect()
		);

		filename_vec.shrink_to_fit();
	}

	//println!( "{}", filename_vec.concat() );

	filename_vec.concat()

}
