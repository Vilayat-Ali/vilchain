use structure::{
    txn::{non_publishable_txn::NonPublishableTransaction, TxnBuilder},
    FloatValue,
};

fn main() {
    let mut t = TxnBuilder::new()
        .set_from("asdsdasd")
        .set_to("asdasdasd")
        .set_value(FloatValue::new(12, 23, 43))
        .build()
        .unwrap();

    let d = t.publish();

    println!("{:#?}", serde_json::to_string_pretty(&d).unwrap());
    let hash = d.clone().hash.unwrap();
    println!("{:#?}", hash);
    println!("{:#?}", d.verify_hash(hash));
}
