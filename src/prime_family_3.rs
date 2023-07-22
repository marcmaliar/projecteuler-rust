use itertools::Itertools;
use primes::{PrimeSet, Sieve};
use std::collections::{HashMap, HashSet};

fn zero_part_of_prime(prime: u64, digits_to_zero: &Vec<usize>) -> u64 {
    let mut p = prime;
    for i in digits_to_zero {
        let ppart = ((p / 10_u64.pow((*i).try_into().unwrap())) % 10)
            * 10_u64.pow((*i).try_into().unwrap());
        p = p - ppart;
    }
    p
}

pub fn run() {
    let mut pset = Sieve::new();

    let mut already_seen_smallest: HashSet<(u64, u64)> = HashSet::new();
    let mut powersets: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    let mut factors: HashMap<Vec<usize>, u64> = HashMap::new();

    // let mut current = 29_000_000;

    for i in 11.. {
        if i % 10000 == 0 {
            println!("Prime {}", i);
        }

        //let i = k;//pset.find(current+1).1;
        //current = i;

        let number_of_digits = (i as f64).log10().ceil() as usize;
        if !powersets.contains_key(&number_of_digits) {
            let sets2 = (0..(number_of_digits)).powerset().collect_vec();
            powersets.insert(number_of_digits, sets2);
        }
        let sets = powersets.get(&number_of_digits).unwrap();

        for set in sets {
            let mut families = Vec::new();
            if set.len() == 0 {
                continue;
            }
            let zeroed = zero_part_of_prime(i, set);

            if !factors.contains_key(&*set) {
                let mut factor2 = 0;
                for i in set {
                    factor2 += 10_u64.pow((*i).try_into().unwrap());
                }
                factors.insert((*set).to_vec(), factor2);
            }
            let factor = factors.get(set).unwrap();

            let mut p = zeroed;
            for d in 0..10 {
                if d == 0 {
                    if already_seen_smallest.contains(&(p, *factor)) {
                        break;
                    }
                    already_seen_smallest.insert((p, *factor));
                    if set.contains(&(number_of_digits - 1)) {
                        //println!("Set contains last digit, skipping {}", p);
                        p += factor;
                        continue;
                    }
                }

                if pset.is_prime(p) {
                    families.push(p);
                }
                p += factor;
            }
            //println!("Prime family {}: {:?}", i, families);
            if families.len() > 7 {
                println!("Prime family: {:?}", families);
                return;
            }
        }
    }
}
