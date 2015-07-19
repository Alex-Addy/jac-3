fn main() {
    let a = sieve_of_eratosthenes(2usize.pow(31));
    println!("Size of {} element res: {}", a.len(), std::mem::size_of_val(&a));
}
//I'm looking for numbers of the form p*q*r where p q and r are prime, and in base ten the product has the same digit frequencies as the factors
//so I'm basically looping through a bunch of prime triplets, multiplying them out, formatting to base 10, and comparing against the digit counts of the factors
//I either want a counterexample to the conjecture that all such numbers have 3 as one p q or r, or just a bunch of examples that will maybe lead to insight
//the 'slow' version is currently the baseline, where I sieve out a few hundred million primes, and then loop over the triplets, multiply them out, then format everything and compare
//the 'fast' version adds an extra precomputation step, where it preformats and makes a histogram of the digit counts. 10 bytes per prime, adds about 2 gigs to memory requirement
//but it also means every prime is formatted once, and not 200million squared times or whatever :p
//the sieving process is already threaded, but the actual triplet checking is not. That's the next step probably, is to thread that

#[test]
fn digits_test() {
    assert_eq!(digits(0), [0; 10]);
    assert_eq!(digits(111111), [0,6,0,0,0,0,0,0,0,0]);
    assert_eq!(digits(1234567890), [1,1,1,1,1,1,1,1,1,1]);
}

fn digits(mut num: usize) -> [usize; 10] {
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
    let mut A: Vec<bool> = vec![true; end/2]; // remove evens
    A[0] = false; // 1 is not a prime
    println!("Size of {} element bool vec {}", A.len(),
        A.len() * std::mem::size_of::<bool>());

    let mut i = 1; // indexes to 3 in array
    while (i*2)+1 < end {
        if A[i] {
            let stride = (i*2)+1;
            let mut k = i + stride;
            while k < end/2 {
                A[k] = false;
                k = k + stride;
            }
        }
        i += 1;
    }

    let mut res = Vec::with_capacity(A.iter().filter(|b| **b).count()+1);
    res.push(2); // HACK: assumes an end greater than 2
    for i in 1..A.len() {
        if A[i] {
            res.push((i*2)+1);
        }
    }
//    println!("{:?}", res);
    res
//    A.iter().enumerate()
//        .skip(2) // 0 and 1 are not primes
//        .filter_map(|(i, b)| if *b { Some(i) } else { None })
//        .collect()
}

