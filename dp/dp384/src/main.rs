/*
https://stackoverflow.com/questions/59272670/how-do-i-shift-the-elements-inside-a-rust-vector-to-the-right-and-put-the-out-of

For the purpose of this challenge, a k-ary necklace of length n is a sequence of n letters chosen from k options, e.g. ABBEACEEA is a 5-ary necklace of length 9. Note that not every letter needs to appear in the necklace. Two necklaces are equal if you can move some letters from the beginning to the end to make the other one, otherwise maintaining the order. For instance, ABCDE is equal to DEABC. For more detail, see challenge #383 Easy: Necklace matching.

Today's challenge is, given k and n, find the number of distinct k-ary necklaces of length n. That is, the size of the largest set of k-ary necklaces of length n such that no two of them are equal to each other. You do not need to actually generate the necklaces, just count them.

For example, there are 24 distinct 3-ary necklaces of length 4, so necklaces(3, 4) is 24. Here they are:

AAAA  BBBB  CCCC
AAAB  BBBC  CCCA
AAAC  BBBA  CCCB
AABB  BBCC  CCAA
ABAB  BCBC  CACA
AABC  BBCA  CCAB
AACB  BBAC  CCBA
ABAC  BCBA  CACB
You only need to handle inputs such that kn < 10,000.

necklaces(2, 12) => 352
necklaces(3, 7) => 315
necklaces(9, 4) => 1665
necklaces(21, 3) => 3101
necklaces(99, 2) => 4950


Try generating the combinations, de-duplicate by rotating every vector of 'letters' (just u8s here)
so that the cumulative sum of the shortest set of initial letters is lowest out of all possibilities?
*/

use std::collections::HashMap;

fn seq_value(seq: &Vec<u32>, k : u32) -> u32 {
    let mut val = 0;
    for ind in 0..seq.len() {
       val += seq[ind] * k.pow(seq.len() as u32 - 1 - ind as u32);
    }
    val
}

fn shift_to_minimum(seq : &Vec<u32>, k : u32) -> Vec<u32> {
    let mut min_ind = 0;
    let mut min_val = 0;
    let mut seq_tmp = seq.clone();
    for ind in 0..seq.len() {
        let val = seq_value(&seq_tmp, k);
        if ind == 0 {
            min_val = val;
            min_ind = ind;
        } else if val < min_val {
            min_val = val;
            min_ind = ind;
        }
        seq_tmp.rotate_right(1);
    }
    seq_tmp.rotate_right(min_ind);
    seq_tmp
}

fn seq_incr(mut seq : &mut Vec<u32>, ind : usize, k : u32) {
    seq[ind] += 1;
    if seq[ind] >= k {
        if ind == 0 {
            return;
        }
        seq_incr(&mut seq, ind - 1, k);
        seq[ind] -= k;
    }
}

fn necklaces(k : u32, n : usize) {
    let mut seq = vec![0; n];
    let count = k.pow(n as u32);
    let mut necklaces = HashMap::new();

    for _ in 0..count {
        let shifted = shift_to_minimum(&mut seq, k);
        necklaces.insert(shifted, true);
        // println!("{:?} {:?}", seq, shifted);
        let ind = seq.len() - 1;
        seq_incr(&mut seq, ind, k);
    }

    println!("necklaces({}, {}) => {}", k, n, necklaces.keys().len());
    /*
    for seq in necklaces.keys() {
        println!("{:?}", seq);
    }
    println!("");
    */
}

fn main() {
    necklaces(3, 4);
    necklaces(2, 12);
    necklaces(3, 7);
    necklaces(9, 4);
    necklaces(21, 3);
    necklaces(99, 2);
}
