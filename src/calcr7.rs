extern crate u15;

use u15::U15;
use std::collections::HashMap;

//NOTE: Not the fastest in the world but should on a reasonable machine
//with optimizations turned on take less than 10 min to find the result.
//Without release compile this overflows the stack!
//r7 = 25734
fn main() {
    let a = U15::from(4);
    let b = U15::from(1);
    //The variable we want to tune exactly to the value that 
    //our ackerman returns 6 as result
    let mut r7 = U15::from(1);
    let mut memory = HashMap::new();
    let mut res = ack(a, b, r7, &mut memory); 
    //Play with the register until we have the right input
    while res != U15::from(6) {
        r7 = r7 + 1;
        memory.clear();
        res = ack(a, b, r7, &mut memory);
    }
    println!("Found correct answer as r7={} with result of: {}", r7, res);
}


//This function is rewritten to be better readable from the reverse engineered
//Ackerman function found in the dissasembly (see function.cpp)
fn ack(a: U15, b: U15, r7: U15, memory: &mut HashMap<(U15, U15), U15>) -> U15 {
    //Here is where our Memoization kicks in to speed things up
    if let Some(mem) = memory.get(&(a, b)) {
        return *mem;
    }

    //Standard ackerman
    if a == U15::from(0) {
        return b + 1;
    }
    if b == U15::from(0) {
        let res = ack(a + 32_767, r7, r7, memory);
        memory.insert((a + 32_767, r7), res);
        return res;
    }
    let res1 = ack(a, b + 32_767, r7, memory);
    memory.insert((a, b + 32_767), res1);
    let res2 = ack(a + 32_767, res1, r7, memory);
    memory.insert((a + 32_767, res1), res2);

    res2
}