use std::env::args;
use primes;

fn main() {

  let args: Vec<String> = args().collect();

  let start = u64::from_str_radix(args.get(1).unwrap(), 10).unwrap();
  let end = u64::from_str_radix(args.get(2).unwrap(), 10).unwrap();

  let mut part_vec: Vec<u64> = Vec::new();

  let mut num: u64 = 0;

  while num < start{
    part_vec.push(num * num - num);
    num += 1;
  }
  //num is start
  //part_vec.get(x).is_ok() | x < start

  while num < end {//from start to end
    part_vec.push(num * num - num);//add current number pair
    
    let mut c = 0;
    //check if All x < num | x is prime
    while primes::is_prime(num + part_vec.get(c).unwrap()) {
      c += 1;
    }
    
    if c < num as usize{//num is no caboose number
      num += 1;
      continue;
    }
    
    println!("{} is a Caboose number", num);

    num += 1;
  }

}
