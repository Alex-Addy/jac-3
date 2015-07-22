extern crate time;

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{channel, sync_channel};

const UPPER_BOUND: usize = 1 << 16;
const THREAD_LIMIT: usize = 2;

fn main() {
    let start_t = time::SteadyTime::now();

    // start set up {{{
    println!("{} : Calling sieve", time::SteadyTime::now() - start_t);
    let a = sieve_of_eratosthenes(UPPER_BOUND);
    println!("{} : Size of {} element, primes: {}",
             time::SteadyTime::now() - start_t,
             a.len(),
             a.capacity() * std::mem::size_of::<usize>());

    let d: Vec<[u8; 10]> = a.iter().map(|p| digits(*p)).collect();
    println!("{} : Size of {} element, digits: {}",
             time::SteadyTime::now() - start_t,
             d.len(),
             d.capacity() * std::mem::size_of::<[u8; 10]>());
    // end of set up }}}

    // thread set up {{{
    // use sync_channel for producer to keep
    // down channel size
    let (tx_p, rx_c) = sync_channel(THREAD_LIMIT + 1);
    let (tx_c, rx_m) = channel();

    let primes = Arc::new(a);
    let digits = Arc::new(d);

    // producer thread
    // will make the 2-tuples that the threads will use as work quantums
    {
        let primes = primes.clone();
        let digits = digits.clone();
        let start_t = start_t.clone();
        thread::spawn(move || {
            for i in 0..primes.len() {
                for k in i..primes.len() {
                    tx_p.send((i,k));
                }
                println!("{} : Factor {} is completely queued",
                         time::SteadyTime::now() - start_t, primes[i])
            }
        });
        // drop(tx_p); // move takes over tx_p
    }

    let recv = Arc::new(Mutex::new(rx_c));
    for _ in 0..THREAD_LIMIT {
        let recv = recv.clone();
        let tx = tx_c.clone();

        let primes = primes.clone();
        let digits = digits.clone();

        thread::spawn(move || {
            loop {
                let res = {
                    let lock = recv.lock().unwrap();
                    lock.recv()
                }; // drop lock on mutex

                let fin = match res {
                    Ok((i,k)) => find_matching_digits(i,k,&primes,&digits),
                    Err(_) => return, // channel is poisoned
                };

                if let Some(r) = fin {
                    tx.send(r);
                }
            }
        });
    }
    // clean up channels
    // need to drop references in this thread,
    // allowing channels to be closed when threads end
    drop(tx_c);
    drop(recv);
    // end thread set up }}}

    // start wait {{{
    for i in rx_m.iter() {
        println!("Found jac-3 number: {}", i);
    }
    // end wait }}}
    println!("{} : Did not find digit matching under {}",
             time::SteadyTime::now() - start_t, UPPER_BOUND);
}

#[test]
fn matching_test() {
    let primes = sieve_of_eratosthenes(1 << 16);
    let digits = primes.iter().map(|p| digits(*p)).collect();
    
    assert_eq!(Some(1061583), find_matching_digits(1,17, &primes, &digits));
}

fn find_matching_digits(i: usize, k: usize,
    primes: &Vec<usize>, digs: &Vec<[u8; 10]>) -> Option<usize> {
        let prelim_prod = primes[i] * primes[k];
        let prelim_digs = add_u8_10(&digs[i], &digs[k]);

        for m in std::cmp::max(k, i)..primes.len() {
            let prod = primes[m] * prelim_prod;
            let prod_digs = digits(prod);
            let d = add_u8_10(&digits(primes[m]), &prelim_digs);
            if d == prod_digs {
                return Some(prod);
            }
        }

        None
}

#[test]
fn add_test() {
    assert_eq!(add_u8_10(&[0; 10], &[0; 10]), [0; 10]);
    assert_eq!(add_u8_10(&[1; 10], &[3; 10]), [4; 10]);
    assert_eq!(add_u8_10(&[0,1,2,3,4,5,6,7,8,9],
                         &[0; 10]),
               [0,1,2,3,4,5,6,7,8,9]);
    assert_eq!(add_u8_10(&[0,0,0,1,2,3,0,0,0,133],
                         &[1,2,0,0,2,0,0,0,12,22]),
                         [1,2,0,1,4,3,0,0,12,155]);

    // testing both add and digits
    assert_eq!(add_u8_10(
                &add_u8_10(&digits(3), &digits(653)),
                &digits(691)),
            digits(1353669));
}

fn add_u8_10(lhs: &[u8; 10], rhs: &[u8; 10]) -> [u8; 10] {
    let mut res = [0; 10];
    for i in 0..10 {
        res[i] = lhs[i] + rhs[i];
    }

    res
}

#[test]
fn digits_test() {
    assert_eq!(digits(0), [0; 10]);
    assert_eq!(digits(111111), [0,6,0,0,0,0,0,0,0,0]);
    assert_eq!(digits(1234567890), [1,1,1,1,1,1,1,1,1,1]);
}

fn digits(mut num: usize) -> [u8; 10] {
    let mut array = [0; 10];
    while num > 0 {
        let rem = num % 10;
        array[rem] += 1;
        num = num / 10;
    }
    array
}

#[test]
fn sieve_test() {
    assert_eq!(sieve_of_eratosthenes(12), vec![2,3,5,7,11]);
    assert_eq!(sieve_of_eratosthenes(25), vec![2,3,5,7,11,13,17,19,23]);

    assert_eq!(sieve_of_eratosthenes(1000),
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
        59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127,
        131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
        197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269,
        271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349,
        353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431,
        433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
        601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673,
        677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761,
        769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857,
        859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947,
        953, 967, 971, 977, 983, 991, 997]);
}

// creates a sieve from 2 to end(exclusive)
fn sieve_of_eratosthenes(end: usize) -> Vec<usize> {
    let mut a: Vec<bool> = vec![true; end/2]; // remove evens
    a[0] = false; // 1 is not a prime
    println!("Size of {} element bool vec {}", a.len(),
        a.len() * std::mem::size_of::<bool>());

    let mut i = 1; // indexes to 3 in array
    while (i*2)+1 < end {
        if a[i] {
            let stride = (i*2)+1;
            let mut k = i + stride;
            while k < end/2 {
                a[k] = false;
                k = k + stride;
            }
        }
        i += 1;
    }

    let mut res = Vec::with_capacity(a.iter().filter(|b| **b).count()+1);
    res.push(2); // HACK: assumes an end greater than 2
    for i in 1..a.len() {
        if a[i] {
            res.push((i*2)+1);
        }
    }
    res
}

