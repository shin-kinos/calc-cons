
use std::collections::HashMap;

pub fn calc_rsa( resname_list : &Vec<&str>, asa_list : &Vec<f64> ) -> Vec<f64>
{
	let asa_trip : HashMap<String, f64> = define_asa_trip();

	let mut rsa_list : Vec<f64> = Vec::new();

	/*
	 * Calculate RSA in each residue.
	 * asa_list[]     = ASA list of all the residues
	 * asa_trip[]     = ASA list of Ala-X-Ala tripeptide ( denominator of RSA )
	 * resname_list[] = Residue name list ( index ) in 3 letters
	 * rsa            = RSA
	 * rsa_list[]     = RSA list
	 */
	for i in 0 .. asa_list.len() {
		let rsa : f64 = ( *asa_list )[ i ] / asa_trip[ resname_list[ i ] ];
		rsa_list.push( rsa )
	}

	rsa_list
}

pub fn detect_state( rsa_list : &Vec<f64> ) -> Vec<&str>
{
	let length : usize = ( *rsa_list ).len();

	let mut state_list : Vec<&str> = Vec::new();

	/*
	 * Divide the residues into 3 type based on their RSA. 
	 * Beried                       : RSA < 9%
	 * Intermediate ( semi-beried ) : 9%  ≦ RSA ≦ 36%
	 * Exposed                      : RSA > 36%
	 */
	for i in 0 .. length {
		if rsa_list[ i ] < 0.09 {
			state_list.push( "buried" );
		} else if rsa_list[ i ] >= 0.09 && rsa_list[ i ] <= 0.36 {
			state_list.push( "intermediate" );
		} else if rsa_list[ i ] > 0.36 {
			state_list.push( "exposed" );
		}
	}

	state_list
}

fn define_asa_trip() -> HashMap<String, f64>
{
	/* Define the ASAs of Ala-X-Ala tripeptide ( denominator of RSA ). */
	let aa_list : Vec<String> = vec![
		"ALA".to_string(),
		"ARG".to_string(),
		"ASN".to_string(),
		"ASP".to_string(),
		"CYS".to_string(),
		"GLN".to_string(),
		"GLU".to_string(),
		"GLY".to_string(),
		"HIS".to_string(),
		"ILE".to_string(),
		"LEU".to_string(),
		"LYS".to_string(),
		"MET".to_string(),
		"PHE".to_string(),
		"PRO".to_string(),
		"SER".to_string(),
		"THR".to_string(),
		"TRP".to_string(),
		"TYR".to_string(),
		"VAL".to_string() 
	]; 

	let asa_aa : Vec<f64>
	= vec![
		110.2, // ALA
		229.0, // ARG
		146.4, // ASN
		144.1, // ASP
		140.4, // CYS
		178.6, // GLN
		174.7, // GLU
		78.7 , // GLY
		181.9, // HIS
		185.0, // ILE
		183.1, // LEU
		205.7, // LYS
		200.1, // MET
		200.7, // PHE
		141.9, // PRO
		117.2, // SER
		138.7, // THR
		240.5, // TRP
		213.7, // TYR
		153.7  // VAL
	];

	let mut asa_trip : HashMap<String, f64> = HashMap::new();

	for i in 0 .. 20 {
		let aa : &String = &( aa_list[ i ] );
		asa_trip.insert( aa.to_string(), asa_aa[ i ] );
	}

	asa_trip
}
