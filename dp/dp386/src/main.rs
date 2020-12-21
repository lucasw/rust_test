/*
https://www.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/

There are 7 ways to partition the number 5 into the sum of positive integers:

5 = 1 + 4 = 1 + 1 + 3 = 2 + 3 = 1 + 2 + 2 = 1 + 1 + 1 + 2 = 1 + 1 + 1 + 1 + 1
Let's express this as p(5) = 7. If you write down the number of ways to partition each number starting at 0 you get:

p(n) = 1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, ...
By convention, p(0) = 1.

Challenge
Compute p(666). You must run your program all the way through to completion to meet the challenge. To check your answer, p(666) is a 26-digit number and the sum of the digits is 127. Also, p(66) = 2323520.

You can do this using the definition of p(n) above, although you'll need to be more clever than listing all possible partitions of 666 and counting them. Alternatively, you can use the formula for p(n) given in the next section.

*/

use std::collections::HashMap;

fn update_pmap(pmap : &mut HashMap<u64, HashMap<Vec<u64>, bool>>, n : u64) {
    match pmap.get(&n) {
        Some(_vvec) => (),
        None => {
            let mut vvec = HashMap::new();
            vvec.insert(vec![n], true);
            // TODO(lucasw) this triple for loop gets super slow in the 30s
            let mut count = 0;
            for m in 1..(n / 2 + 1) {
                // println!("    n {}, m {}, n-m {}", n, m, n - m);
                update_pmap(pmap, m);
                update_pmap(pmap, n - m);
                let v1 = pmap.get(&m).unwrap();
                let v2 = pmap.get(&(n - m)).unwrap();

                for va in v1.keys() {
                    for vb in v2.keys() {
                        // println!("va {:?}, vb {:?}", va, vb);
                        // let vcombo = va.clone().append(&mut vb.clone());
                        let mut vcombo = va.clone();
                        for v in vb {
                            vcombo.push(*v);
                        }
                        vcombo.sort_unstable();
                        // println!("combo {:?}", vcombo);
                        vvec.insert(vcombo, true);
                        count += 1;
                    }
                }
            }
            println!("vvec processed {}, unique {}", count, vvec.keys().len());
            pmap.insert(n, vvec);
        }
    }
}

fn main() {
    let mut pvec = HashMap::new();
    pvec.insert(vec![1], true);
    let mut pmap = HashMap::new();
    pmap.insert(1, pvec);

    for n in 1..30 {
        update_pmap(&mut pmap, n);
        match pmap.get(&n) {
            // Some(vecs) => println!("{} num {},  {:?}", n, vecs.keys().len(), vecs.keys()),
            Some(vecs) => println!("{} num {}", n, vecs.keys().len()),
            None => ()
        }
        // println!("---");
    }
}
