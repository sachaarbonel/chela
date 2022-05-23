use chela::Column;
use chela::ToEntity;
use chela::Entity;

#[derive(ToEntity)]
struct Point {
    name: String,
    x: i32,
    y: i32,
}

fn main() {
    let point = Point {
        name: "origin".to_string(),
        x: 2,
        y: 3,
    };
    let schema = point.to_entity();
    println!("{:#?}", schema);
}
