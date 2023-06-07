use ordered_float::OrderedFloat;

fn main() {
    let a0 = 0.0f64;
    let a1 = OrderedFloat(a0);
    let b0 = -0.0f64;
    let b1 = OrderedFloat(b0);
    dbg!(
        a0,
        a1,
        b0,
        b1,
        a0 == b0,
        a1 == b1,
        a0.partial_cmp(&b0),
        b0.partial_cmp(&b1),
        serde_json::to_string(&a0).unwrap(),
        serde_json::to_string(&a1).unwrap(),
        serde_json::to_string(&b0).unwrap(),
        serde_json::to_string(&b1).unwrap(),
    );
}
