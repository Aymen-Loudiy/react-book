pub fn _main() {
    println!("Hello, world!");

    _another_function();
}

pub fn _another_function() {
    println!("Another function.");
}
pub fn _statement_vs_expression() {
    let _statement = 5;
    let _expression = {
        let x = 5;
        x + 1
    };
}
