// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut shopping_list = Vec::new();
    add_to_list(&mut shopping_list, "milk");
}

fn add_to_list<T>(item_list: &mut Vec<T>, item: T) {
    item_list.push(item);
}