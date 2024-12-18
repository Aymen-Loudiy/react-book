fn data_types() {
    let _a: u32 = 50;
    let _b: i32 = -5;
    let _c: f64 = 1.03;
    let _d: bool = true;
    let _e: char = 'e';
    let tup: (i32, &str, f64) = (1, "a", 5363.2);
    let _first_tup: i32 = tup.0;
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _first_arr: i32 = _arr[0];
}
