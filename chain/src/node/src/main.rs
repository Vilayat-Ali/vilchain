use structure::BigNum;

fn main() {
    let mut b1 = BigNum::from_string("15.112").unwrap();
    let b2 = BigNum::from_string("5.002").unwrap();

    b1.add(&b2);
    println!("{:?}", b1);
//     println!("{:?}", b2);
}