mod periodic_table;

fn main() {
    // List first 10 elements
    for element in periodic_table::ELEMENTS.iter().take(10) {
        println!("{:?}", element);
    }
}
