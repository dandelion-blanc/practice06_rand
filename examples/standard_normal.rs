//!
//!
//!

extern crate rand;
extern crate rand_distr;

use rand::prelude::*;
use rand_distr::StandardNormal;


fn main()
{
//	let mut rng = rand::thread_rng();
//	let val: f64 = rng.sample(StandardNormal);
	let val :f64 = thread_rng().sample(StandardNormal);

	println!("{}", val);
}
