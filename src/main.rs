use dashu::{float::FBig, integer::IBig};
fn factorial(n: i32) -> IBig {
    let mut result = IBig::ONE;
    for i in 1..=n {
        result *= IBig::from(i);
    }
    result
}
fn main() {
    let mut k = 0;
    let maxk = 72;
    const BITS_OF_PRECISION: usize = 3322;
    let mut resultnumerator: FBig = FBig::ZERO.with_precision(BITS_OF_PRECISION).unwrap();
    let mut resultdenominator: FBig = FBig::ZERO.with_precision(BITS_OF_PRECISION).unwrap();
    let mut result: FBig = FBig::ZERO.with_precision(BITS_OF_PRECISION).unwrap();
    let negative_one: FBig = FBig::NEG_ONE.with_precision(BITS_OF_PRECISION).unwrap();
    let three_over_two: FBig = FBig::from(3).with_precision(BITS_OF_PRECISION).unwrap() / FBig::from(2).with_precision(BITS_OF_PRECISION).unwrap();
    let thatonebignum: FBig = FBig::from(640320).with_precision(BITS_OF_PRECISION).unwrap();
    let othernum = IBig::from(545140134);
    while k < maxk {
        resultnumerator = negative_one.powi(IBig::from(k));
        resultnumerator *= factorial(6*k);
        resultnumerator *= (&othernum*k) + 13591409;
        resultdenominator = factorial(3*k).into();
        resultdenominator *= factorial(k).pow(3);
        resultdenominator *= thatonebignum.powf(&(FBig::from(3*k).with_precision(BITS_OF_PRECISION).unwrap() + three_over_two.clone()));
        result += resultnumerator/resultdenominator;
        k += 1;
    }
    result *= 12;
    result = 1/result;
    println!("{:?}", result.to_decimal().value().to_string())
}
