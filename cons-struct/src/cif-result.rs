
use std::fs::File;
use std::io::Write;

use crate::neighboring;

pub fn show_save_result(
	res_name_list : &Vec<String>,
	res_num_list  : &Vec<isize>,
	coord_x_list  : &Vec<f64>,
	coord_y_list  : &Vec<f64>,
	coord_z_list  : &Vec<f64>,
	arg_o         : &String
) {

	println!( "\nResult :\n" );

	let mut fout = File::create( ( *arg_o ).as_str() ).expect( "FAILED to open output file" );

	let vec_len : usize = res_name_list.len();

	println!( "CtrRsdName\tCtrRsdNum\tCtrCodX\tCtrCodY\tCtrCodZ\tNbrRsdName\tNbrRsdNum\tNbrCodX\tNbrCodY\tNbrCodZ\tDist" );
	writeln!( fout, "CtrRsdName\tCtrRsdNum\tCtrCodX\tCtrCodY\tCtrCodZ\tNbrRsdName\tNbrRsdNum\tNbrCodX\tNbrCodY\tNbrCodZ\tDist" ).expect( "FAILED to open output file" );
	for i in 0 .. vec_len {
		for j in 0 .. vec_len {
			if i != j {
				let dist : f64 = neighboring::calc_dist(
					( *coord_x_list )[ i ],
					( *coord_y_list )[ i ],
					( *coord_z_list )[ i ],
					( *coord_x_list )[ j ],
					( *coord_y_list )[ j ],
					( *coord_z_list )[ j ]
				);
				if dist < 6.5 {
					print!( "{}\t",    ( *res_name_list )[ i ] );
					print!( "{}\t",    ( *res_num_list  )[ i ] );
					print!( "{:.3}\t", ( *coord_x_list  )[ i ] );
					print!( "{:.3}\t", ( *coord_y_list  )[ i ] );
					print!( "{:.3}\t", ( *coord_z_list  )[ i ] );
					print!( "{}\t",    ( *res_name_list )[ j ] );
					print!( "{}\t",    ( *res_num_list  )[ j ] );
					print!( "{:.3}\t", ( *coord_x_list  )[ j ] );
					print!( "{:.3}\t", ( *coord_y_list  )[ j ] );
					print!( "{:.3}\t", ( *coord_z_list  )[ j ] );
					print!( "{:.3}\t", dist                    );
					print!( "\n"                               );

					write!( fout, "{}\t",    ( *res_name_list )[ i ] ).expect( "FAILED to write" );
					write!( fout, "{}\t",    ( *res_num_list  )[ i ] ).expect( "FAILED to write" );
					write!( fout, "{:.3}\t", ( *coord_x_list  )[ i ] ).expect( "FAILED to write" );
					write!( fout, "{:.3}\t", ( *coord_y_list  )[ i ] ).expect( "FAILED to write" );
					write!( fout, "{:.3}\t", ( *coord_z_list  )[ i ] ).expect( "FAILED to write" );
					write!( fout, "{}\t",    ( *res_name_list )[ j ] ).expect( "FAILED to write" );
					write!( fout, "{}\t",    ( *res_num_list  )[ j ] ).expect( "FAILED to write" );
					write!( fout, "{:.3}\t", ( *coord_x_list  )[ j ] ).expect( "FAILED to write" );
					write!( fout, "{:.3}\t", ( *coord_y_list  )[ j ] ).expect( "FAILED to write" );
					write!( fout, "{:.3}\t", ( *coord_z_list  )[ j ] ).expect( "FAILED to write" );
					write!( fout, "{:.3}\t", dist                    ).expect( "FAILED to write" );
					write!( fout, "\n"                               ).expect( "FAILED to write" );
				}
			}
		}
	}

	println!( "\nThe output file was correctly written.\n" );

}
