
pub fn calc_dist(
	x1 : f64,
	y1 : f64,
	z1 : f64,
	x2 : f64,
	y2 : f64,
	z2 : f64 
) -> f64 {

	let dist : f64 = (
		( ( x1 - x2 ) * ( x1 - x2 ) ) +
		( ( y1 - y2 ) * ( y1 - y2 ) ) +
		( ( z1 - z2 ) * ( z1 - z2 ) )
	).sqrt() as f64;

	dist

}
