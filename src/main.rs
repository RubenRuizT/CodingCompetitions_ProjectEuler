// 1. point of entry of the rust program
fn main() {
    f01();
}

// --- problems --- 
//1. Multiples of 3 and 5
fn f01() {

    let mut a = 3;                         // first multiple
    let mut b = 5;                         // second multiple
    let mut bound = 999;                   // bound of <1000 

    let a_sum = arit_progrs(a,bound);      // arit_progrs(3,999) 
    let b_sum = arit_progrs(b,bound);      // arit_progrs(5,999) 
    let a_b_sum = arit_progrs(a*b,bound);  // arit_progrs(15,999) 

    println!("ans = {:?}", a_sum+b_sum-a_b_sum); // 233168

}

// arithmetic progression  sum(n)= n*(n_last+d)/2
fn arit_progrs(d:u32, bound:u32) -> u32 {  // example 5,999 
    let mut n = bound/d as u32;            // 199 = 999/5 as u32
    let mut n_last = n*d;                  // 995 = 199*5           
    n*(n_last+d)/2                         // 199(995+5)/2
}