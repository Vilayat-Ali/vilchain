use structure::BigNum;

fn main() {
        let mut b1 = BigNum::from_string("22.0000").unwrap();
        let b2 = BigNum::from_string("11.0000").unwrap();
      let mut b1_c = b1.clone();
        b1_c.add(&b2);
        println!("{:#?} - {:#?} = {:?}", b1, b2, b1_c);
}