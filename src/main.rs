/* practice06_rand by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 *
 *
 */

extern crate rand;

use rand::*;
use rand::prelude::*;
use rand::distributions::*;

use std::*;

fn main()
{
	let seed = [ 10 as u8 ; 32];
	let mut rng1 :rand::prelude::StdRng = SeedableRng::from_seed(seed);
	let mut rng2 :StdRng = SeedableRng::from_seed(seed);
	let range = Uniform::new(0.0 as f64, 1.0 as f64);

	for _ in 0..10
	{
		println!("{ } { }", rng1.gen::<f64>(), range.sample(&mut rng2));
	}
	println!(" ");

//乱数生成速度検証１ [0:1]
	let max_cycle = 10u32.pow(9);
	let mut rng1 :rand::prelude::StdRng = SeedableRng::from_seed(seed);
	let mut rng2 :StdRng = SeedableRng::from_seed(seed);
	let range = Uniform::new(0.0 as f64, 1.0 as f64);

	let start_cpu = time::Instant::now();												//[0:1]gen生成
	let start_real = time::SystemTime::now();

	for _ in 0..max_cycle
	{
		let _ :f64 = rng1.gen::<f64>();
	}

	let rng1_cputime = start_cpu.elapsed();
	let rng1_realtime = start_real.elapsed().unwrap();

	let start_cpu = time::Instant::now();												//[0:1]uniform生成
	let start_real = time::SystemTime::now();

	for _ in 0..max_cycle
	{
		let _ :f64 = range.sample(&mut rng2);
	}

	let rng2_cputime = start_cpu.elapsed();
	let rng2_realtime = start_real.elapsed().unwrap();

	println!("generate uniformed random number [0:1] at {:.1e}", max_cycle);
	println!("mehod name       : cpu time[msec]  | real time[msec]");
	println!("gen::<f64>()     : {:3}.{:010} | {:3}.{:010}",
				rng1_cputime.as_millis(), rng1_cputime.as_micros(), rng1_realtime.as_millis(), rng1_realtime.as_micros());
	println!("uniform & sample : {:3}.{:010} | {:3}.{:010}\n",
				rng2_cputime.as_millis(), rng2_cputime.as_micros(), rng2_realtime.as_millis(), rng2_realtime.as_micros());

//乱数生成速度検証２ [0:1]
	let max_cycle = 10u32.pow(9);
	let mut rng1 :rand::prelude::StdRng = SeedableRng::from_seed(seed);
	let mut rng2 :StdRng = SeedableRng::from_seed(seed);
	let range = Uniform::new(0.0 as f64, 1.0 as f64);

	let start_cpu = time::Instant::now();												//[0:1]gen生成
	let start_real = time::SystemTime::now();

	for _ in 0..max_cycle
	{
		let x :f64 = rng1.gen::<f64>();
		let _ = x * x;
	}

	let rng1_cputime = start_cpu.elapsed();
	let rng1_realtime = start_real.elapsed().unwrap();

	let start_cpu = time::Instant::now();												//[0:1]uniform生成
	let start_real = time::SystemTime::now();

	for _ in 0..max_cycle
	{
		let x :f64 = range.sample(&mut rng2);
		let _ = x * x;
	}

	let rng2_cputime = start_cpu.elapsed();
	let rng2_realtime = start_real.elapsed().unwrap();

	println!("generate uniformed random number [0:1] + f64 product at {:.1e}", max_cycle);
	println!("mehod name       : cpu time[msec]  | real time[msec]");
	println!("gen::<f64>()     : {:3}.{:010} | {:3}.{:010}",
				rng1_cputime.as_millis(), rng1_cputime.as_micros(), rng1_realtime.as_millis(), rng1_realtime.as_micros());
	println!("uniform & sample : {:3}.{:010} | {:3}.{:010}\n",
				rng2_cputime.as_millis(), rng2_cputime.as_micros(), rng2_realtime.as_millis(), rng2_realtime.as_micros());

//乱数生成速度検証 [0:n]
	let range_upper :f64 = 1000.0;													// n指定
	let max_cycle = 10u32.pow(9);
	let mut rng1 :rand::prelude::StdRng = SeedableRng::from_seed(seed);
	let mut rng2 :StdRng = SeedableRng::from_seed(seed);
	let range = Uniform::new(0.0 as f64, range_upper as f64);

	let start_cpu = time::Instant::now();												//[0:n]gen生成
	let start_real = time::SystemTime::now();

	for _ in 0..max_cycle
	{
		let _ :f64 = rng1.gen_range(0.0 as f64, &range_upper);
	}

	let rng1_cputime = start_cpu.elapsed();
	let rng1_realtime = start_real.elapsed().unwrap();

	let start_cpu = time::Instant::now();												//[0:n]uniform生成
	let start_real = time::SystemTime::now();

	for _ in 0..max_cycle
	{
		let _ :f64 = range.sample(&mut rng2);
	}

	let rng2_cputime = start_cpu.elapsed();
	let rng2_realtime = start_real.elapsed().unwrap();

	println!("generate uniformed random number [0:{ }] at {:.1e}", range_upper, max_cycle);
	println!("mehod name       : cpu time[msec]  | real time[msec]");
	println!("gen::<f64>()     : {:3}.{:010} | {:3}.{:010}",
				rng1_cputime.as_millis(), rng1_cputime.as_micros(), rng1_realtime.as_millis(), rng1_realtime.as_micros());
	println!("uniform & sample : {:3}.{:010} | {:3}.{:010}\n",
				rng2_cputime.as_millis(), rng2_cputime.as_micros(), rng2_realtime.as_millis(), rng2_realtime.as_micros());
}
