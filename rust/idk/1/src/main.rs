fn compare_ordered_option<T: PartialOrd>(a: Option<T>, b: Option<T>) -> std::cmp::Ordering {
    match (a, b) {
        (None, None) => std::cmp::Ordering::Equal,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(x), Some(y)) => x.partial_cmp(&y).expect("wtf"),
    }
}
fn main() {
    let a = Some(5);
    let b = None;
    println!("{:#?}", compare_ordered_option(a, b));
}
