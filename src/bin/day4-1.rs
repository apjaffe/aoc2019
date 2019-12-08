use std::convert::identity;
fn check_pass(mut pass:i64) -> bool {
    let mut last_dig = 11;
    let mut repeat = false;
    while pass > 0 {
        let dig = pass % 10;
        if dig > last_dig {
            return false;
        }
        if dig == last_dig {
            repeat = true;
        }
        pass /= 10;
        last_dig = dig;
    }
    repeat
}
fn main() {
    let sum = (256310..732737).map(|x| check_pass(x)).filter(|x| *x).count();

    println!("{}", sum);
}
