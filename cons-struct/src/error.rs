
use std::process;
use colored::*;

pub fn error_bomb( arg : &str )
{
	println!( "{}", "\n!!! ERROR !!!\n".red() );

	match arg {
		"seq_title_not_same"                      => println!( "Inadequate format in Multi-FASTA file."                     ),
		"seq_len_not_same"                        => println!( "The length of all the sequences must be same."              ),
		"site_ent_len_not_same"                   => println!( "Length of ( *site_list ) != Length of ( *cons_re_list )"    ),
		"non_standard_residue"                    => println!( "Non-standard residue was observed in the input file."       ),
		"unexpected_symbol"                       => println!( "Unexpected symbol was observed in the input file."          ),
		"no_such_title_line"                      => println!( "No such a title line in the input Multi-FASTA file."        ),
		"len_label_atom_id_group_pdb_not_same"    => println!( "Lengh of ( label_atom_id ) != Length of ( group_pdb )."     ),
		"len_label_alt_id_group_pdb_not_same"     => println!( "Lengh of ( label_alt_id ) != Length of ( group_pdb )."      ),
		"len_label_comp_id_group_pdb_not_same"    => println!( "Lengh of ( label_comp_id ) != Length of ( group_pdb )."     ),
		"len_label_asym_id_group_pdb_not_same"    => println!( "Lengh of ( label_asym_id ) != Length of ( group_pdb )."     ),
		"len_label_seq_id_group_pdb_not_same"     => println!( "Lengh of ( label_seq_id ) != Length of ( group_pdb )."      ),
		"len_auth_comp_id_group_pdb_not_same"     => println!( "Lengh of ( auth_comp_id ) != Length of ( group_pdb )."      ),
		"len_auth_asym_id_group_pdb_not_same"     => println!( "Lengh of ( auth_asym_id ) != Length of ( group_pdb )."      ),
		"len_auth_atom_id_group_pdb_not_same"     => println!( "Lengh of ( auth_atom_id ) != Length of ( group_pdb )."      ),
		"len_auth_seq_id_group_pdb_not_same"      => println!( "Lengh of ( auth_seq_id ) != Length of ( group_pdb )."       ),
		"len_cartn_x_group_pdb_not_same"          => println!( "Lengh of ( cartn_x ) != Length of ( group_pdb )."           ),
		"len_cartn_y_group_pdb_not_same"          => println!( "Lengh of ( cartn_y ) != Length of ( group_pdb )."           ),
		"len_cartn_z_group_pdb_not_same"          => println!( "Lengh of ( cartn_z ) != Length of ( group_pdb )."           ),
		"len_res_num_list_res_name_list_not_same" => println!( "Length of ( res_num_list ) != Length of ( res_name_list )." ),
		"len_coord_x_list_res_name_list_not_same" => println!( "Length of ( coord_x_list ) != Length of ( res_name_list )." ),
		"len_coord_y_list_res_name_list_not_same" => println!( "Length of ( coord_y_list ) != Length of ( res_name_list )." ),
		"len_coord_z_list_res_name_list_not_same" => println!( "Length of ( coord_z_list ) != Length of ( res_name_list )." ),
		"len_resseq_list_chainid_list_not_same"   => println!( "Length of ( resseq_list ) != Length of ( chainid_list )."   ),
		"len_resname_list_chainid_list_not_same"  => println!( "Length of ( resname_list ) != Length of ( chainid_list )."  ),
		"len_asa_list_chainid_list_not_same"      => println!( "Length of ( asa_list ) != Length of ( chainid_list )."      ),
		"no_such_seq_in_msa"                      => println!( "There was not the same sequence in the input MSA."          ),
		"data_len_not_same"                       => println!( "Inadequate structural data. Check the input mmCIF file."    ),
		_                                         => (),
	}

	println!( "{}", "\n!!! Program halted !!!\n".red() );

	process::exit( 1 );
}
