use std::convert::identity;
fn check_pass(mut pass:i64) -> bool {
    let mut last_dig = 11;
    let mut double = false;
    let mut rep_count = 1;
    while pass > 0 {
        let dig = pass % 10;
        if dig > last_dig {
            return false;
        }
        if dig == last_dig {
            rep_count += 1;
        } else {
            if rep_count == 2 {
                double = true;
            }
            rep_count = 1;
        }
        pass /= 10;
        last_dig = dig;
    }
    double || (rep_count == 2)
}
fn main() {
    let sum = (256310..732737).map(|x| check_pass(x)).filter(|x| *x).count();

    println!("{}", sum);
}
