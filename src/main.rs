use std::collections::HashMap; 

// 1. point of entry of the rust program
fn main() {

    f05(); 
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

// 4.Largest palindrome product
fn f04() {
    let mut max :u32 = 0;                                           // this var will be use to get the max
    for i in (1..999).rev(){                                        // do a reverse double loop   
        for j in (1..999).rev(){            
            let mut n : String = (i*j).to_string();                 // convert to string the i*j
            let mut n_reversed : String= n.chars().rev().collect(); // assign to n_reversed the rev of n 
            if n == n_reversed && n.parse::<u32>().unwrap()>max{    // if n==n_rev and is above max save it 
               max = n.parse::<u32>().unwrap();                     // save it as u32 
            }
        }
    }

    println!("max = {:?}", max);                                    // print the max value  

}
 
fn f05() {                                                          // find the smallest number could be divided by each num from 1 to 20

    let mut v = Vec::<f64>::new();                                  // create a new vector of floats to get the primes from 2..21 

    for i in 2..21{                                                 // from 2..21 
        if f_is_prime(i) == true{                                   // if is a prime num then 
            v.push(i as f64);                                       // push it on the vector
        }
    }

    let mut v_pow: Vec::<f64> = v                                   // create a new vector to put the powers
            .iter()                                                 // convert to iter 
            .map(|x| (20.0_f64)                                     // map to each variable
            .log(*x as f64)                                         // make the log in base x of 20  
            .floor())                                               // get the floor 
            .collect();                                             // convert to vector

    let mut res = 1.0;                                              // create var of res 
    for it in v.iter().zip(v_pow.iter()){                           // create an iterator zipped wit v and v_pow to  
        let (i,j) = it;                                             // split the tuple 
        res *= i.powf(*j);                                          // res = 2.powf(4)
    }

    println!("res = {:?}", res as i64);                             // display the result 
}


fn f_is_prime(x:i32) -> bool {                                      // function to know if a number is prime. f_is_prine(11)
   for i in (2..x).rev(){                                           // for each number in (2..10).rev 
       if x % i == 0 {                                              // if is divisible 11 % 2 == 0
           return false;                                            // return false
       }
   } 
   true                                                             // if not is divisible then return true, then is prime
}