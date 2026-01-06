use std::{io::stdin, time::Instant};

use dashu::{base::SquareRoot, float::{FBig, round::mode::HalfEven}, integer::IBig};


fn factorial(n: i32) -> IBig {
    let mut result = IBig::ONE;
    for i in 1..=n {
        result *= IBig::from(i);
    }
    result
}

fn original_chudnovsky(digits_of_precision: f64, bits_of_precision: usize, maxk: i32) {
    let mut k = 0;
    let mut resultnumerator: FBig;
    let mut resultdenominator: FBig;
    let mut result: FBig = FBig::ZERO.with_precision(bits_of_precision).unwrap();
    let othernum = IBig::from(545140134);

    while k < maxk {
        resultnumerator = if k % 2 == 0 {FBig::ONE.with_precision(bits_of_precision).unwrap()} else {FBig::NEG_ONE.with_precision(bits_of_precision).unwrap()};
        resultnumerator *= factorial(6*k);
        resultnumerator *= (&othernum*k) + 13591409;
        resultdenominator = FBig::from(factorial(3*k)).with_precision(bits_of_precision).unwrap();
        resultdenominator *= factorial(k).pow(3);
        resultdenominator *=  IBig::from(640320).pow((3*k).try_into().unwrap());
        result += resultnumerator/resultdenominator;
        k += 1;
    }

    result = (426880 * FBig::sqrt(&FBig::from(10005).with_precision(bits_of_precision).unwrap())) / result;
    let s = result.to_decimal().value().to_string();
    let truncated = &s[..digits_of_precision as usize + 2];
    println!("{}", truncated);
}

fn optimized_chudnovsky(bits: usize, digits_of_precision: f64) {

    
    let c: i64 = 640320;
    let c3_over_24: i128 = (c as i128).pow(3) / 24;

    
    let mut k: i64 = 0;
    let mut a_k: FBig<HalfEven>  = FBig::ONE.with_precision(bits + 32).unwrap();
    let mut a_sum = a_k.clone();
    let mut b_sum = FBig::ZERO.with_precision(bits + 32).unwrap();

    loop {
        k += 1;

        let num = -(6*k - 5) * (2*k - 1) * (6*k - 1);
        let den = (k*k*k) as i128 * c3_over_24;

        a_k *= FBig::from(num).with_precision(bits + 32).unwrap();
        a_k /= FBig::from(den).with_precision(bits + 32).unwrap();

        a_sum += &a_k;
        b_sum += FBig::from(k).with_precision(bits + 32).unwrap() * &a_k;

        let threshold: FBig<HalfEven> = FBig::ONE.with_precision(bits + 32).unwrap() >> bits.try_into().unwrap();
        if &a_k * &a_k < &threshold * &threshold {
            break;
        }
    }



    
    let total =
        FBig::from(13591409i64).with_precision(bits + 32).unwrap() * a_sum +
        FBig::from(545140134i64).with_precision(bits + 32).unwrap() * b_sum;

    
    let sqrt_10005 = FBig::from(10005u32).with_precision(bits + 32).unwrap().sqrt();
    let pi = FBig::from(426880u32).with_precision(bits + 32).unwrap() * sqrt_10005 / total;
    let s = pi.to_decimal().value().to_string();
    let truncated = &s[..digits_of_precision as usize + 2];
    println!("{}", truncated);

}


fn main() {
    let mut input = String::new();
    println!("input number of digits:");
    stdin().read_line(&mut input).expect("input is not input");
    let digits_of_precision = input.trim().parse::<f64>().unwrap();
    let maxk: i32 = ((digits_of_precision / 14.0) as f64).ceil() as i32;
    let bits_of_precision = (((digits_of_precision * 3.32193) as f64).ceil() as usize) + 80;


    let starttime = Instant::now();

    original_chudnovsky(digits_of_precision, bits_of_precision, maxk);

    println!("base algorithm ran in {:#?}", starttime.elapsed());


    let optimizedstarttime = Instant::now();

    optimized_chudnovsky(bits_of_precision, digits_of_precision);

    println!("Optimized algorithm ran in {:#?}", optimizedstarttime.elapsed());
}
