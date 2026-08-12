fn get_value(opt: Option<i32>) -> i32 {
    opt.unwrap_or_else(|| 0)
}

fn main() {
    let value = get_value(Some(10));
    println!("Value: {}", value);

    let default_value = get_value(None);
    println!("Default value: {}", default_value);
}
