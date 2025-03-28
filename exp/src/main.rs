use inherits::new_type;

#[new_type]
struct NewType(String);

fn main() {
    let n = NewType(String::from("Hello World"));

    assert_eq!(11, n.len());
}
