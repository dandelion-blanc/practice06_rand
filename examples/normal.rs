//!
//!
//!

extern crate rand;
extern crate rand_distr;

use rand::Rng;
use rand::RngCore;
use rand_distr::{Normal, Distribution};


fn main()
{
	let mu = 0.0_f64;
	let sigma2 = 100.0_f64;

	let mut rng = rand::thread_rng();
//	let mut rng = rand::prelude::StdRng;
	let normal = Normal::new(mu, sigma2.sqrt()).unwrap();
	let x :f64 = rng.sample(normal);
//	let x :f64 = normal.sample(&mut rng);

    println!("{ }", x);
}