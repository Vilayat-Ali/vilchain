use common::{txn, FloatValue};

fn main() {
    let t1: common::txn::Txn = common::txn::builder::TxnBuilder::init()
        .set_sender_address(String::from("Asd"))
        .set_recepient_address(String::from("ASD"))
        .set_gas(FloatValue::new(0, 10, 1))
        .set_value(FloatValue::new(1, 5, 142))
        .build()
        .unwrap();
}
