fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test ()
{
    println!("Hello Test");
}

#[test]
fn variable_test ()
{
    let name = "Moch Fikri Khoirurrizal";
    println!("Hello {}", name);
}

#[test]
fn mut_variable_test ()
{
    let mut name = "Moch Fikri Khoirurrizal";
    println!("Hello {}", name);

    name = "Fikri Wado";
    println!("Hello {}", name);
}

#[test]
fn static_typing_test ()
{
    // let mut name = "Moch Fikri Khoirurrizal";
    let name = "Moch Fikri Khoirurrizal";
    println!("Hello {}", name);

    // name = 15;
    println!("Hello {}", name);
}

#[test]
fn shadowing_test ()
{
    let name = "Moch Fikri Khoirurrizal";
    println!("Hello {}", name);

    let name = 15;
    println!("Hello {}", name);
}

/*
    Komentar ajah bos ini mah
    Komentar ajah bos ini mah
    Komentar ajah bos ini mah
*/
#[test]
fn comment_test ()
{
    //Ini komen juga bos
    println!("Hello");
}

#[test]
fn explicit_test ()
{
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn number_test ()
{
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion_test ()
{
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = a as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    println!("{}", d);

    let e: i8 = d as i8;
    println!("{}", e);

    let f: i32 = 32768;
    println!("{}", f);

    let g: i16 = f as i16;
    println!("{}", g); // result: -32768
    /*
        - i16 => -32768 until 32767
        - It's mean why "g" variable is -32768, because number overflow
    */
}

#[test]
fn numeric_test ()
{
    let a = 10;
    let b = 10;

    let c = a * b;
    println!("{}", c);

    let d = a / b;
    println!("{}", d);

    let e = a + b;
    println!("{}", e);
}

#[test]
fn augmented_test ()
{
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean_test ()
{
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison_test ()
{
    let a = 10;
    let b = 20;
    let result: bool = a > b;

    println!("{}", result);
}

#[test]
fn boolean_operator_test ()
{
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;

    println!("{}", lulus);
}

#[test]
fn char_type_test ()
{
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple_test ()
{
    // let data: (i32, f64, bool) = (10, 10.5, true);

    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a,b,c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    println!("{:?}", data);
}

fn unit()
{
    println!("Hello");
}

#[test]
fn unit_test ()
{
    let result = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array_test ()
{
    // let array: [i32; 5] = [1,2,3,4,5];

    let mut array: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimenstional_test ()
{
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant_test ()
{
    const MINIMUM: i32 = 0;
    println!("{}, {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope_test ()
{
    println!("{}", MAXIMUM);

    let fikri = 1;

    {
        println!("{}", fikri);
        let wado = 2;
        println!("{}", wado);
    }

    // println!("{}", wado); // error
}

#[test]
fn stack_heap_test ()
{
    function_a();
    function_b();
}

fn function_a ()
{
    let a = 10;
    let b = String::from("Fikri");
    println!("{} {}", a, b);
}

fn function_b ()
{
    let a = 10;
    let b = String::from("Wado");
    println!("{} {}", a, b);
}

#[test]
fn string_test () {
    let name = "  Moch Fikri Khoirurrizal  ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type_test () {
    let mut name: String = String::from("Moch Fikri");
    println!("{}", name);

    name.push_str(" Khoirurrizal");
    println!("{}", name);

    let new_name = name.replace("Moch", "M");
    println!("{}", name);
    println!("{}", new_name);
}

#[test]
fn ownership_rules_test () {
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    println!("{}", a);
}

#[test]
fn data_copy_test () {
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement_test () {
    let name1: String = String::from("Moch Fikri Khoirurrizal");
    println!("{}", name1);

    let name2: String = name1; // pindah ownership
    println!("{}", name2);
    // println!("{}", name1); // error
}

#[test]
fn clone_test () {
    let name1: String = String::from("Moch Fikri Khoirurrizal");
    let name2: String = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression_test () {
    // let value = 7;
    // let result: &str;

    // if value >= 8 {
    //     result = "Good";
    // } else if value >= 6 {
    //     result = "Not Bad";
    // } else if value >= 3 {
    //     result = "Bad";
    // } else {
    //     result = "Very Bad";
    // }

    let value = 10;
    let result: &str = if value >= 8 { "Good" }
    else if value >= 6 { "Not Bad" }
    else if value >= 3 { "Bad" }
    else { "Very Bad" };

    println!("{}", result);
}

#[test]
fn loop_expression_test () {
    let mut counter = 0;

    loop {
        counter += 1;

        if  counter > 10 { break; }
        else if counter % 2 == 0 { continue; }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value_test () {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter > 10 { break counter * 2; }
    };

    println!("{}", result);
}

#[test]
fn loop_label_test () {
    let mut number = 1;

    'outer: loop {
        let mut i = 1;

        loop {
            if number > 10 { break 'outer; }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;

            if i > 10 { break; }
        }

        number += 1;
    }
}

#[test]
fn while_loop_test () {
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter)
        }

        counter += 1;
    }
}

#[test]
fn array_iteration_test () {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop_test () {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range_test () {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for i in range {
        println!("{}", array[i])
    }
}

#[test]
fn range_inclusive_test () {
    let range = 0..=4;
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for i in range {
        println!("{}", array[i])
    }
}

fn say_hello () {
    println!("Hello");
}

#[test]
fn say_hello_test () {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye (first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn say_goodbye_test () {
    say_goodbye("Moch Fikri", "Khoirurrizal");
    say_goodbye("Fikri", "Wado");
}

fn factorial_loop (n: i32) -> i32 {
    if n <= 0 { return 0; }

    let mut result = 1;
    for i in 1..=n  {
        result *= i;
    }

    result
}

#[test]
fn factorial_loop_test () {
    let result = factorial_loop(5);
    println!("{}", result);

    let result = factorial_loop(-10);
    println!("{}", result);
}

fn print_text (value: String, times: u32) {
    if times == 0 { return; }
    else { println!("{}", value) }

    print_text(value, times -  1);
}

#[test]
fn print_text_test () {
    print_text(String::from("Fikri"), 10);
}

fn factorial_recursive (n: u32) -> u32 {
    if n <= 1 { return 1; }
    n * factorial_recursive(n - 1)
}

#[test]
fn factorial_recursive_test () {
    let result = factorial_recursive(5);
    println!("{}", result);
}
