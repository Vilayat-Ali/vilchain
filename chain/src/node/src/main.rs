use structure::{txn::TxnBuilder, FloatValue};

fn main() {
    let t = TxnBuilder::new()
        .set_from("asdsdasd")
        .set_to("asdasdasd")
        .set_value(FloatValue::new(12, 23, 43))
        .build();

    println!("{:#?}", serde_json::to_string_pretty(&t.unwrap()).unwrap());
}
