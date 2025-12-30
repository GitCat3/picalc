use dashu::{base::SquareRoot, float::FBig, integer::IBig};
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
    const BITS_OF_PRECISION: usize = 3322 + 80;
    let mut resultnumerator: FBig = FBig::ZERO.with_precision(BITS_OF_PRECISION).unwrap();
    let mut resultdenominator: FBig = FBig::ZERO.with_precision(BITS_OF_PRECISION).unwrap();
    let mut result: FBig = FBig::ZERO.with_precision(BITS_OF_PRECISION).unwrap();
    let negative_one: FBig = FBig::NEG_ONE.with_precision(BITS_OF_PRECISION).unwrap();
    let othernum = IBig::from(545140134);
    while k < maxk {
        resultnumerator = negative_one.powi(IBig::from(k));
        resultnumerator *= factorial(6*k);
        resultnumerator *= (&othernum*k) + 13591409;
        resultdenominator = FBig::from(factorial(3*k)).with_precision(BITS_OF_PRECISION).unwrap();
        resultdenominator *= factorial(k).pow(3);
        resultdenominator *=  IBig::from(640320).pow((3*k).try_into().unwrap());
        result += resultnumerator/resultdenominator;
        k += 1;
    }
    result = (426880 * FBig::sqrt(&FBig::from(10005).with_precision(BITS_OF_PRECISION).unwrap())) / result;
    println!("{:.1000}", result.to_decimal().value())
}
