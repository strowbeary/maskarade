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
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use pbr::MultiBar;
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

    let new_file = File::create(&Path::new("file.mask")).unwrap();
    let stream = BufWriter::new(new_file);
    let arc_stream = Arc::new(Mutex::new(stream));
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
    println!("Generating key");
    seed = x0.clone();

    let mut handles = vec![];
    let mut mb = MultiBar::new();
    mb.println("Generating key file");
    let thread_number = 6;
    for _offset in 0..thread_number {
        let mut next_occurrence_offset = if _offset > 0 {1} else {0};
        let start: u64 = length / thread_number * (_offset);
        let end: u64 = (length / thread_number) * (_offset + 1) - 1;
        println!("offset {}, from {} to {}", _offset, &start, &end);
        let mut thread_seed = seed.clone();
        let thread_m = m.clone();
        let thread_x0 = seed.clone();
        let thread_stream = Arc::clone(&arc_stream);
        let mut pb = mb.create_bar(end - start);
        pb.format("║█░░║");
        let handle = thread::spawn( move || {
            for _i in start..end {
                thread_seed = thread_seed.square().divmod(&thread_m).1;
                if thread_seed.eq(&thread_x0) {
                    println!("Period is {}", _i);
                }
                let mod_seed = thread_seed.divmod(&Int::from(256)).1;
                let bit = u8::from(&mod_seed);

                thread_stream.lock().unwrap().write(&[bit]).unwrap();
                pb.inc();
            }
            pb.finish();
        });
        handles.push(handle);
    }
    mb.listen();
    for handle in handles {
        handle.join().unwrap();
    }

}
