use std::{env::args, process::exit};
use primes::{self, PrimeSet};
use rayon::prelude::*;
//use pbr::{self, MultiBar}; //ty https://github.com/a8m/pb/tree/master

fn main(){

  let args: Vec<String> = args().collect();

  let end = usize::from_str_radix(&args[1], 10).unwrap_or(100);

  //create prime array for iteration
  let mut pset = primes::Sieve::new();

  let last_p = pset.get(end);

  let mut part_vec = vec![0u64; last_p as usize+1];//create buffer for threads to write

  eprintln!("Running on {:?} threads", rayon::current_num_threads());

  eprintln!("Testing from 0 to {}", last_p);

  //fill part vector from 0 to the last prime
  part_vec.par_iter_mut().enumerate().for_each(|(idx, x)| {
    *x = (idx * idx - idx) as u64;
  });

  //check if number is caboose number
  pset.iter().for_each(|prime| {

    if prime as usize >= part_vec.len(){
      eprintln!("Done");
      exit(0);
    }
    
    let mut c: u64 = 0;

    while c <= prime{

      if !primes::is_prime(prime + part_vec[c as usize]){
        if c == prime{
          println!("{} is a caboose number", prime);
        }
        break;
      }

      c += 1;
    }

  });

}