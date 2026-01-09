use heck::ToSnakeCase;
fn main() {
    println!("type_ -> {}", "type_".to_snake_case());
    println!("type -> {}", "type".to_snake_case());
}
