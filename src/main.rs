use std::{i128, io::{self, Write}};

use dashu::{dbig, float::DBig, integer::IBig};

fn factorial(number : i128) -> IBig{
    let mut factorial : IBig = IBig::ONE;
    for i in 1..(number+1) { factorial*=i; }
    return factorial
}

fn main() {
    let bignum = IBig::from(545140134);
    let smallnum = IBig::from(13591409);
    let bottomnum = DBig::from(640320);
    let mut pi = dbig!(0).with_precision(1000).unwrap();
    let mut iterations = String::new();
    print!("input number of iterations to run: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut iterations).expect("failed to read line");
    for k in 0..iterations.trim_end().parse::<i128>().expect("invalid input") {
        let mut numerator = IBig::from((-1_i128).pow(k.try_into().unwrap()));
        numerator*=factorial(6*k);
        numerator*=(&bignum*k)+&smallnum;
        let mut denominator = DBig::from(factorial(3*k));
        denominator *= factorial(k).pow(3);
        denominator *= &bottomnum.powf(&(DBig::from(3*k).with_precision(1000).unwrap() + dbig!(1.5).with_precision(1000).unwrap()));
        pi += numerator/denominator;
    }
    pi = 1/(12*pi);
    println!("{}", pi.to_string());
    println!("should be accurate to roughly {} digits", 14*iterations.trim_end().parse::<i128>().unwrap());
}
