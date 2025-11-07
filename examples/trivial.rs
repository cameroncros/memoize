use memoize::memoize;

#[memoize]
fn hello1(arg: String, arg2: usize) -> bool {
    println!("{} => {}", arg, arg2);
    arg.len() % 2 == arg2
}

#[memoize]
fn hello2(arg: String, arg2: usize) -> Result<bool, ()> {
    println!("{} => {}", arg, arg2);
    Ok(arg.len() % 2 == arg2)
}

fn main() {
    // `hello` is only called once here.
    assert!(hello1("World1".to_string(), 0));
    assert!(hello1("World1".to_string(), 0));
    // Sometimes one might need the original function.
    assert!(memoized_original_hello1("World1".to_string(), 0));
    memoized_flush_hello1();

    // `hello` is only called once here.
    assert!(hello2("World2".to_string(), 0).unwrap());
    assert!(hello2("World2".to_string(), 0).unwrap());
    // Sometimes one might need the original function.
    assert!(memoized_original_hello2("World2".to_string(), 0).unwrap());
    memoized_flush_hello2();
}
