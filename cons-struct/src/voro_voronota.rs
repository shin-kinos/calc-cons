
use std::fs::read_to_string;
use std::process::Command;

use crate::error;

pub struct Voronota {
	pub ball1_chainid : Vec<String>,
	pub ball1_resseq  : Vec<String>,
	//pub ball1_icode   : Vec<String>,
	//pub ball1_serial  : Vec<String>,
	//pub ball1_altloc  : Vec<String>,
	pub ball1_resname : Vec<String>,
	pub ball1_name    : Vec<String>,
	pub ball2_chainid : Vec<String>,
	//pub ball2_resseq  : Vec<String>,
	//pub ball2_icode   : Vec<String>,
	//pub ball2_serial  : Vec<String>,
	//pub ball2_altloc  : Vec<String>,
	//pub ball2_resname : Vec<String>,
	//pub ball2_name    : Vec<String>,
	pub area          : Vec<String>,
	//pub dist          : Vec<String>,
	//pub query         : Vec<String>,
	//pub draw          : Vec<String>,
}

impl Voronota {
	pub fn new() -> Voronota
	{
		let ball1_chainid : Vec<String> = Vec::new();
		let ball1_resseq  : Vec<String> = Vec::new();
		//let ball1_icode   : Vec<String> = Vec::new();
		//let ball1_serial  : Vec<String> = Vec::new();
		//let ball1_altloc  : Vec<String> = Vec::new();
		let ball1_resname : Vec<String> = Vec::new();
		let ball1_name    : Vec<String> = Vec::new();
		let ball2_chainid : Vec<String> = Vec::new();
		//let ball2_resseq  : Vec<String> = Vec::new();
		//let ball2_icode   : Vec<String> = Vec::new();
		//let ball2_serial  : Vec<String> = Vec::new();
		//let ball2_altloc  : Vec<String> = Vec::new();
		//let ball2_resname : Vec<String> = Vec::new();
		//let ball2_name    : Vec<String> = Vec::new();
		let area          : Vec<String> = Vec::new();
		//let dist          : Vec<String> = Vec::new();
		//let query         : Vec<String> = Vec::new();
		//let draw          : Vec<String> = Vec::new();

		Voronota {
			ball1_chainid : ball1_chainid,
			ball1_resseq  : ball1_resseq,
			//ball1_icode   : ball1_icode,
			//ball1_serial  : ball1_serial,
			//ball1_altloc  : ball1_altloc,
			ball1_resname : ball1_resname,
			ball1_name    : ball1_name,
			ball2_chainid : ball2_chainid,
			//ball2_resseq  : ball2_resseq,
			//ball2_icode   : ball2_icode,
			//ball2_serial  : ball2_serial,
			//ball2_altloc  : ball2_altloc,
			//ball2_resname : ball2_resname,
			//ball2_name    : ball2_name,
			area          : area,
			//dist          : dist,
			//query         : query,
			//draw          : draw,
		}
	}

	pub fn read_voronota_info( &mut self, arg_o : &str )
	{
		let mut fin_name : Vec<&str> = Vec::new();
		fin_name.push( arg_o      );
		fin_name.push( ".contacts" );
		fin_name.push( ".expanded" );
		//println!( "{}", fin_name.concat() );

		let fin = read_to_string( fin_name.concat() /* .as_str() */ ).expect( "FAILED to open input file" );

		for line in fin.lines() {
			let compornents : Vec<&str> = line.split_whitespace().collect();

			self.ball1_chainid.push( compornents[  0 ].to_string() );
			self.ball1_resseq .push( compornents[  1 ].to_string() );
			//self.ball1_icode  .push( compornents[  2 ].to_string() );
			//self.ball1_serial .push( compornents[  3 ].to_string() );
			//self.ball1_altloc .push( compornents[  4 ].to_string() );
			self.ball1_resname.push( compornents[  5 ].to_string() );
			self.ball1_name   .push( compornents[  6 ].to_string() );
			self.ball2_chainid.push( compornents[  7 ].to_string() );
			//self.ball2_resseq .push( compornents[  8 ].to_string() );
			//self.ball2_icode  .push( compornents[  9 ].to_string() );
			//self.ball2_serial .push( compornents[ 10 ].to_string() );
			//self.ball2_altloc .push( compornents[ 11 ].to_string() );
			//self.ball2_resname.push( compornents[ 12 ].to_string() );
			//self.ball2_name   .push( compornents[ 13 ].to_string() );
			self.area         .push( compornents[ 14 ].to_string() );
			//self.dist         .push( compornents[ 15 ].to_string() );
			//self.query        .push( compornents[ 16 ].to_string() );
			//self.draw         .push( compornents[ 17 ].to_string() );
		}
	}

	pub fn check_voronota_data( &self ) {
		if self.ball1_resseq .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball1_icode  .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball1_serial .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball1_altloc .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		if self.ball1_resname.len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		if self.ball1_name   .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		if self.ball2_chainid.len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball2_resseq .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball2_icode  .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball2_serial .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball2_altloc .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball2_resname.len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.ball2_name   .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		if self.area         .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.dist         .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.query        .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
		//if self.draw         .len() != self.ball1_chainid.len() { error::error_bomb( "data_len_not_same" ); }
	}

	/*
	pub fn show_solvent_info( &self )
	{
		println!( "Solvent data : \n" );

		print!( "ChainID\t"  );
		print!( "ResSeq\t"   );
		print!( "ResName\t"  );
		print!( "BallName\t" );
		print!( "ASA\n"      );

		for i in 0 .. ( self.area ).len() {
			print!( "{}\t", ( self.ball1_chainid )[ i ] );
			print!( "{}\t", ( self.ball1_resseq  )[ i ] );
			print!( "{}\t", ( self.ball1_resname )[ i ] );
			print!( "{}\t", ( self.ball1_name    )[ i ] );
			print!( "{}\t", ( self.area          )[ i ] );
			print!( "\n"                                );
		}
	}
	*/

	pub fn make_asa_list( &self ) -> (
		/* chainid_list : */ Vec<&str>,
		/* resseq_list  : */ Vec<usize>,
		/* resname_list : */ Vec<&str>,
		/* asa_list     : */ Vec<f64>
	) {

		let mut chainid_list : Vec<&str>  = Vec::new();
		let mut resseq_list  : Vec<usize> = Vec::new();
		let mut resname_list : Vec<&str>  = Vec::new();
		let mut asa_list     : Vec<f64>   = Vec::new();

		let mut chainid : &String  = &( self.ball1_chainid )[ 0 ];
		let mut resseq  : usize    =  ( self.ball1_resseq  )[ 0 ].parse().unwrap();
		let mut resname : &String  = &( self.ball1_resname )[ 0 ];
		let mut solvent : &String  = &( self.ball2_chainid )[ 0 ];

		let mut asa : f64 = if solvent == "solvent" {
			( self.area )[ 0 ].parse().unwrap()
		} else {
			0.0
		};

		chainid_list.push( chainid );
		resseq_list .push( resseq  );
		resname_list.push( resname );
		asa_list    .push( asa     );

		/*
		* resseq  = Residue number in sequence
		* asa     = ASA
		* chainid = ChainID
		* resname = Residue name
		*/
		for i in 1 .. ( self.ball1_resseq ).len() {
			solvent = &( self.ball2_chainid )[ i ];
			resseq  =  ( self.ball1_resseq )[ i ].parse().unwrap();
			asa     =  ( self.area )[ i ].parse().unwrap();
			if resseq != resseq_list[ resseq_list.len() - 1 ] && solvent == "solvent" {
				chainid = &( self.ball1_chainid )[ i ];
				resname = &( self.ball1_resname )[ i ];
				chainid_list.push( chainid );
				resseq_list .push( resseq  );
				resname_list.push( resname );
				asa_list    .push( asa     );
			} else if resseq != resseq_list[ resseq_list.len() - 1 ] && solvent != "solvent" {
				chainid = &( self.ball1_chainid )[ i ];
				resname = &( self.ball1_resname )[ i ];
				chainid_list.push( chainid );
				resseq_list .push( resseq  );
				resname_list.push( resname );
				asa_list    .push( 0.0     );
			} else if resseq == resseq_list[ resseq_list.len() - 1 ] && solvent == "solvent" {
				let last_element : usize = asa_list.len() - 1;
				asa_list[ last_element ] += asa;
			}
		}

		(
			chainid_list,
			resseq_list,
			resname_list,
			asa_list
		)

	}

}

pub fn exe_voronota( arg_i : &str, arg_o : &str /*, arg_f : &str */ )
{

	////////////////////////////////////////
	println!( "\nVonorota execution :\n" );
	////////////////////////////////////////

	/* Vonorota execution : Make balls data. */
	let mut voronota_cmd_vec : Vec<&str> = Vec::new();
	voronota_cmd_vec.push( "voronota "                  /* .to_string() */ );
	voronota_cmd_vec.push( "get-balls-from-atoms-file " /* .to_string() */ );
	voronota_cmd_vec.push( "--annotated "               /* .to_string() */ );
	voronota_cmd_vec.push( "--input-format "            /* .to_string() */ );
	voronota_cmd_vec.push( "mmcif"                      /* .to_string() */ );
	voronota_cmd_vec.push( " < "                        /* .to_string() */ );
	voronota_cmd_vec.push( arg_i                        /* .to_string() */ );
	voronota_cmd_vec.push( " > "                        /* .to_string() */ );
	voronota_cmd_vec.push( arg_o                        /* .to_string() */ );
	voronota_cmd_vec.push( ".balls"                     /* .to_string() */ );

	let mut voronota_cmd : String = voronota_cmd_vec.concat();
	println!( "\nvoronota get-balls-from-atoms-file command :\n% {}", voronota_cmd );

	let voronota_get_balls = Command::new( "sh" )
		.arg( "-c" )
		.arg( voronota_cmd )
		.status()
		.expect( "failed to execute process" );

	println!( "voronota_get_balls : Process finished ( {} )\n", voronota_get_balls );
	assert!( voronota_get_balls.success() );

	voronota_cmd_vec.clear();

	/* Vonorota execution : Make contacts and solvent data. */
	voronota_cmd_vec.push( "voronota "           /* .to_string() */ );
	voronota_cmd_vec.push( "calculate-contacts " /* .to_string() */ );
	voronota_cmd_vec.push( "--annotated "        /* .to_string() */ );
	voronota_cmd_vec.push( "< "                  /* .to_string() */ );
	voronota_cmd_vec.push( arg_o                 /* .to_string() */ );
	voronota_cmd_vec.push( ".balls "             /* .to_string() */ );
	voronota_cmd_vec.push( "> "                  /* .to_string() */ );
	voronota_cmd_vec.push( arg_o                 /* .to_string() */ );
	voronota_cmd_vec.push( ".contacts"           /* .to_string() */ );

	voronota_cmd = voronota_cmd_vec.concat();
	println!( "\nvoronota calculate-contacts command :\n% {}", voronota_cmd );

	let voronota_contacts = Command::new( "sh" )
		.arg( "-c" )
		.arg( voronota_cmd )
		.status()
		.expect( "failed to execute process" );

	println!( "voronota_contacts : Process finished ( {} )\n", voronota_contacts );
	assert!( voronota_contacts.success() );

	voronota_cmd_vec.clear();

	/* Vonorota execution : Make expanded contacts and solvent data. */
	voronota_cmd_vec.push( "cat "                /* .to_string() */ );
	voronota_cmd_vec.push( arg_o                 /* .to_string() */ );
	voronota_cmd_vec.push( ".contacts "          /* .to_string() */ );
	voronota_cmd_vec.push( "| "                  /* .to_string() */ );
	voronota_cmd_vec.push( "voronota "           /* .to_string() */ );
	voronota_cmd_vec.push( "expand-descriptors " /* .to_string() */ );
	voronota_cmd_vec.push( "| "                  /* .to_string() */ );
	voronota_cmd_vec.push( "column -t "          /* .to_string() */ );
	voronota_cmd_vec.push( "> "                  /* .to_string() */ );
	voronota_cmd_vec.push( arg_o                 /* .to_string() */ );
	voronota_cmd_vec.push( ".contacts.expanded"  /* .to_string() */ );

	voronota_cmd = voronota_cmd_vec.concat();
	println!( "\nvoronota expand-descriptors command :\n% {}", voronota_cmd );

	let voronota_descriptor = Command::new( "sh" )
		.arg( "-c" )
		.arg( voronota_cmd )
		.status()
		.expect( "failed to execute process" );

	println!( "voronota_descriptor : Process finished ( {} )\n", voronota_descriptor );
	assert!( voronota_descriptor.success() );

	/*

	voronota_cmd_vec.clear();

	/* Vonorota execution : Make solvent data. */
	voronota_cmd_vec.push( "cat "                /* .to_string() */ );
	voronota_cmd_vec.push( arg_o                 /* .to_string() */ );
	voronota_cmd_vec.push( ".contacts.expanded " /* .to_string() */ );
	voronota_cmd_vec.push( "| "                  /* .to_string() */ );
	voronota_cmd_vec.push( "grep "               /* .to_string() */ );
	voronota_cmd_vec.push( "\"solvent\" "        /* .to_string() */ );
	voronota_cmd_vec.push( "> "                  /* .to_string() */ );
	voronota_cmd_vec.push( arg_o                 /* .to_string() */ );
	voronota_cmd_vec.push( ".solvent"            /* .to_string() */ );

	voronota_cmd = voronota_cmd_vec.concat();
	println!( "\nPurifying solvent data command :\n% {}", voronota_cmd );

	let voronota_solvent = Command::new( "sh" )
		.arg( "-c" )
		.arg( voronota_cmd )
		.status()
		.expect( "failed to execute process" );

	println!( "voronota_solvent : Process finished ( {} )\n", voronota_solvent );
	assert!( voronota_solvent.success() );

	voronota_cmd_vec.clear();

	/* Remove '$output.balls'. */
	voronota_cmd_vec.push( "rm ".to_string() );
	voronota_cmd_vec.push( arg_o.to_string() );
	voronota_cmd_vec.push( ".balls".to_string() );
	voronota_cmd = voronota_cmd_vec.concat();

	Command::new( "sh" )
		.arg( "-c" )
		.arg( voronota_cmd )
		.status()
		.expect( "failed to execute process" );

	voronota_cmd_vec.clear();

	/* Remove '$output.contacts'. */
	voronota_cmd_vec.push( "rm ".to_string() );
	voronota_cmd_vec.push( arg_o.to_string() );
	voronota_cmd_vec.push( ".contacts".to_string() );
	voronota_cmd = voronota_cmd_vec.concat();

	Command::new( "sh" )
		.arg( "-c" )
		.arg( voronota_cmd )
		.status()
		.expect( "failed to execute process" );

	*/

}
