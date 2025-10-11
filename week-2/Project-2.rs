fn main(){
	let t:f64 = 450000.00 * 2.0;
	let m:f64 = 1500000.00 * 1.0;
	let h:f64 = 750000.00 * 3.0;
	let d:f64 = 2850000.00 * 3.0;
	let a:f64 = 250000.00 * 1.0;

	// sum
	let s = t + m + h + d + a;
	println!("sum is {}", s);

	// average
	let a = s/10.0;
	println!("average is {}", a);

}