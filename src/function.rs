fn _main() {
    println!("Hello, world!");

    _another_function();
}

fn _another_function() {
    println!("Another function.");
}
fn _statement_vs_expression() {
    let _statement = 5;
    let _expression = {
        let x = 5;
        x + 1
    };
}
