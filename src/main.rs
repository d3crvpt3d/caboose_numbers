use std::env::args;
use primes;
use rayon::prelude::*;
//use pbr::{self, MultiBar}; //ty https://github.com/a8m/pb/tree/master

fn main(){

  let args: Vec<String> = args().collect();

  let end = usize::from_str_radix(&args[1], 10).unwrap_or(100);

  //let mb = MultiBar::new();
  //mb.println(&format!("Caboose from {} to {}", start,end));

  //let mut b1 = mb.create_bar(start);
  //let mut b2 = mb.create_bar(end-start);

  let mut part_vec = vec![0usize; end];//create buffer for threads to write

  eprintln!("Running on {:?} threads", rayon::current_num_threads());


  //fill test buffer
  part_vec.par_iter_mut().enumerate().for_each(|(idx, x)| {
    *x = idx * idx - idx;
  });


  //check if number is caboose number
  part_vec.par_iter().enumerate().for_each(|(idx, _)| {
    
    let mut c = 0;

    while c <= idx {

      if !primes::is_prime((idx + part_vec[c]) as u64){
        if c == idx{
          println!("{} is a caboose number", idx);
        }
        break;
      }

      c += 1;
    }
    
  });

}