fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
fn statement_vs_expression() {
    let statement = 5;
    let expression = {
        let x = 5;
        x + 1
    };
}
