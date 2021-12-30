
use std::env;
use std::process;

pub struct Options {

	pub cif_input     : String,
	pub cif_author    : String,
	pub cif_chainid   : String,
	//pub cif_title     : String,
	pub cons_input    : String,
	pub cons_weight   : String,
	pub cons_tolerate : String,
	pub cons_bgdist   : String,

	pub output        : String,
	pub colorize      : String,
	pub quiet         : String,
}

impl Options {
	pub fn new() -> Options {

		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut cif_input     : &String = &String::new();
		let mut cif_author    : &String = &String::from( "yes" );
		let mut cif_chainid   : &String = &String::new();
		//let mut cif_title     : &String = &String::new();
		let mut cons_input    : &String = &String::new();
		let mut cons_weight   : &String = &String::from( "hen" );
		let mut cons_tolerate : &String = &String::from( "yes" );
		let mut cons_bgdist   : &String = &String::from( "blosum62" );

		let mut output        : &String = &String::new();
		let mut colorize      : &String = &String::from( "no" );
		let mut quiet         : &String = &String::from( "no" );

		if argc < 9 { show_usage( &argv[ 0 ] ) };
		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"--cif-input"     => { i += 1; cif_input     = &argv[ i ]; }
				"--cif-author"    => { i += 1; cif_author    = &argv[ i ]; }
				"--cif-chainid"   => { i += 1; cif_chainid   = &argv[ i ]; }
				//"--cif-title"     => { i += 1; cif_title     = &argv[ i ]; }
				"--cons-input"    => { i += 1; cons_input    = &argv[ i ]; }
				"--cons-weight"   => { i += 1; cons_weight   = &argv[ i ]; }
				"--cons-bgdist"   => { i += 1; cons_bgdist   = &argv[ i ]; }
				"--cons-tolerate" => { i += 1; cons_tolerate = &argv[ i ]; }

				"--output"        => { i += 1; output        = &argv[ i ]; }
				"--colorize"      => { i += 1; colorize      = &argv[ i ]; }
				"--quiet"         => { i += 1; quiet         = &argv[ i ]; }
				"--help"          => { show_usage( &argv[ 0 ] );           }
				_                 => { show_usage( &argv[ 0 ] );           }
			}
			i += 1;
		}

		match ( *cif_author ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *cons_weight ).as_str() {
			"hen" | "va" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *cons_bgdist ).as_str() {
			"blosum62"  => (),
			"swissprot" => (),
			"extra"     => (),
			"membrane"  => (),
			"intra"     => (),
			"jtt"       => (),
			"wag"       => (),
			"lg"        => (),
			"equal"     => (),
			_           => show_usage( &argv[ 0 ] ),
		}

		match ( *cons_tolerate ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *colorize ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *quiet ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		Options {
			cif_input     : cif_input.to_string(),
			cif_author    : cif_author.to_string(),
			cif_chainid   : cif_chainid.to_string(),
			//cif_title     : cif_title.to_string(),
			cons_input    : cons_input.to_string(),
			cons_weight   : cons_weight.to_string(),
			cons_tolerate : cons_tolerate.to_string(),
			cons_bgdist   : cons_bgdist.to_string(),

			output        : output.to_string(),
			colorize      : colorize.to_string(),
			quiet         : quiet.to_string(),
		}
	}

	pub fn show_parameter( &self ) {

		println!( "\nParameter set :"                               );
		println!( "===============================================" );
		println!( "--cif-input     : {}", self.cif_input            );
		println!( "--cif-author    : {}", self.cif_author           );
		println!( "--cif-chainid   : {}", self.cif_chainid          );
		//println!( "--cif-title     : {}", self.cif_title            );
		println!( "--cons-input    : {}", self.cons_input           );
		println!( "--cons-weight   : {}", self.cons_weight          );
		println!( "--cons-bgdist   : {}", self.cons_tolerate        );
		println!( "--cons-tolerate : {}", self.cons_bgdist          );
		println!( "--output        : {}", self.output               );
		println!( "--colorize      : {}", self.colorize             );
		println!( "--quiet         : {}", self.quiet                );
		println!( "===============================================" );
	}
}

fn show_usage( arg : &String ) {

	println!( "Usage: {} [Options] \n\nOptions :\n\n", *arg );

	println!( "    --cif-input        Input filename in mmCIF format, REQUIRED. " );
	println!( "    --cons-input       Input filename in Multi-FASTA format, REQUIRED.  " );
	println!( "    --cif-chainid      Chain ID of the input structural information, REQUIRED. " );
	println!( "    --output           Output file name, REQUIRED." );
	println!( "    --cif-author       Use the Author Sequence ID ('yes' or 'no', default 'yes').
                           If 'yes',
                               * auth_comp_id
                               * auth_asym_id
                               * auth_atom_id
                           in 'atom_site' Data Category are used.
                           If 'no',
                               * label_comp_id
                               * label_asym_id
                               * label_atom_id
                           are used instead." );
	//println!( "    --cif-title        The title line of the output FASTA file. " );
	println!( "    --cons-weight      Method of sequence weighting ('hen' or 'va', default 'hen').
                           hen : Position-Based method by Henikoff and Henikoff
                           va  : Distance-Based method by Vingron and Argos" );
	println!( "    --cons-bgdist      Background distribution in the relative entropy (default 'blosum62').
                           blosum62  : BLOSUM62
                           swissprot : Swiss-Prot
                           extra     : AA composition in extracellular proteins
                           membrane  : AA composition in membrane proteins
                           intra     : AA composition in intracellular proteins
                           jtt       : JTT
                           wag       : WAG
                           lg        : LG
                           equal     : No background distribution with equal rate (= 0.05)" );
	println!( "    --cons-tolerate    Tolerate non-standard AA types (such as B, Z and X) in input file ('yes' or 'no', default 'yes')
                           yes : All non-standard AAs are converted to gaps.
                           no  : The program halts if the input file includes non-standard AA types." );
	println!( "    --colorize         Colorize each AA residue displayed on the terminal based on their stereochemical properties ('yes' or 'no', default 'no')." );
	println!( "    --quiet            Show the result data on the terminal ('yes' or 'no', default 'no')" );
	println!( "    --help             Print this help, ignore all other arguments. " );
	println!( "\n" );

	process::exit( 1 );
}
