// 1. point of entry of the rust program
fn main() {
    f03();
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


// 2. Even Fibonacci numbers
fn f02() {
    let mut a = 1;                          // start the var a with 1 
    let mut b = 2;                          // start the var b with 2 
    let mut c = 0;                          // start the var c with value 0 

    let mut s = 2;                          // var which hold the sum

    while b < 4000000 {                     // while the value of b is under 4.000.000
       c = a + b;                           // 3 = 1 + 2  

       if c%2 == 0 {                        // if is even add c to s
            s += c; 
       }
       a = b;                               // a = 2  
       b = c;                               // b = 3  
        
    }
    println!("s = {:?}", s);

}


// 3. Large prime factor 
fn f03() {
    let mut num : u64= 600851475143;        // num from the problem  
    let mut factor = 2;                     // to start with a factorial 

    while num > 1 {                         // while num is 1 divide it by is factorial
        if num % factor == 0{               
            num /= factor;                  // 600851475143 = 600851475143 / 3
        } 
        else {
            factor +=1;                     // 2 += 1 
        }
    }

   println!("factor = {:?}", factor);       // print the last factor to get the largest
}