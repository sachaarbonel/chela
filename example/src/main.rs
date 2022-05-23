use chela::Column;
use chela::Entity;
use chela::Schema;
use chela::ToEntity;

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
    let schema = Schema::new(vec![Box::new(point)]);
    println!("{}", schema.run());
}
