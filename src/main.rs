extern crate ramp;
extern crate rand;
extern crate pbr;
extern crate core;

use std::process::Command;
use ramp::Int;
use std::str::FromStr;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;
use pbr::ProgressBar;

fn generate_prime() -> Int {
    let output = Command::new("openssl")
        .arg("prime")
        .arg("-generate")
        .arg("-bits")
        .arg("1024")
        .output()
        .expect("failed to execute process");
    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout.pop();
    return Int::from_str(&stdout).unwrap();
}

fn main()  {
    let length = 500000;
    let mut pb = ProgressBar::new(length);
    pb.format("║█░░║");
    let new_file = File::create(&Path::new("file.mask")).unwrap();
    let mut stream = BufWriter::new(new_file);
    print!("Generating two big primes");
    let m = generate_prime() * generate_prime();
    println!(" [done]");
    print!("Finding a seed");

    let mut seed= Int::zero();
    while (seed == 0) || (seed == 1) && (seed.gcd(&m) != 1) {
        seed = Int::from(rand::random::<u8>());
    }
    let x0 = seed.clone();
    println!(" [done]");

    seed = x0.clone();
    for _i in 0..length {
        seed = seed.square().divmod(&m).1;
        if seed.eq(&x0) {
            println!("Period is {}", _i);
        }
        let mod_seed = seed.divmod(&Int::from(256)).1;
        let bit = u8::from(&mod_seed);

        stream.write(&[bit]).unwrap();
        pb.inc();
    }
    pb.finish_print("done");

}
