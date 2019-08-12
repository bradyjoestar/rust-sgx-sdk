#![deny(warnings, missing_debug_implementations)]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate byteorder;
extern crate bytes;
extern crate slab;
extern crate mio;
extern crate httparse;

extern crate log;
extern crate rand;
extern crate sha1;

use rand::distributions::{Distribution, Uniform};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub fn libfn(){
    println!("{}","swb");

    {
        let range = Uniform::new(-1.0f64, 1.0);
        let mut rng = rand::thread_rng();

        let total = 1_000_000;
        let mut in_circle = 0;

        for _ in 0..total {
            let a = range.sample(&mut rng);
            let b = range.sample(&mut rng);
            if a*a + b*b <= 1.0 {
                in_circle += 1;
            }
        }

        // prints something close to 3.14159...
        println!("π is approximately {}", 4. * (in_circle as f64) / (total as f64));
    }


}