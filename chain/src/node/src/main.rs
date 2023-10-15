use structure::{txn, FloatValue};

fn main() {
    let txn = txn::TxnBuilder::new()
        .set_from("0xadasda432332j43j24j324h324")
        .set_to("0xj12321j3j123123j123")
        .set_value(FloatValue::default())
        .build();

    let resp: String = serde_json::to_string_pretty(&txn).unwrap();

    println!("{:#?}", resp);
}
