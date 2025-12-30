use std::io::stdin;

use dashu::{base::SquareRoot, float::FBig, integer::IBig};


fn factorial(n: i32) -> IBig {
    let mut result = IBig::ONE;
    for i in 1..=n {
        result *= IBig::from(i);
    }
    result
}


fn main() {
    let mut input = String::new();
    println!("input number of digits:");
    stdin().read_line(&mut input).expect("input is not input");
    let digits_of_precision = input.trim().parse::<f64>().unwrap();
    let mut k: i32 = 0;
    let maxk: i32 = ((digits_of_precision / 14.0) as f64).ceil() as i32;
    let bits_of_precision = (((digits_of_precision * 3.32193) as f64).ceil() as usize) + 80;
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
