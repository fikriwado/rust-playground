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

fn print_number (number: i32) {
    println!("number {}", number);
}

fn hi(name: String) {
    println!("name {}", name);
}

#[test]
fn hi_test () {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Fikri");
    hi(name);
    // println!("{}", name); // error ownership
}

fn full_name (first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn full_name_test () {
    let first_name = String::from("Moch Fikri");
    let last_name = String::from("Khoirurrizal");
    let full_name = full_name(first_name, last_name);

    println!("{}", full_name);
    // println!("{}", first_name); // error ownership
    // println!("{}", last_name); // error ownership
}

fn full_name_handle_ownership (first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);
    (first_name, last_name, full_name)
}

#[test]
fn full_name_handle_ownership_test () {
    let first_name = String::from("Moch Fikri");
    let last_name = String::from("Khoirurrizal");
    let (first_name, last_name, full_name) = full_name_handle_ownership(first_name, last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn full_name_reference (first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn full_name_reference_test () {
    let first_name = String::from("Moch Fikri");
    let last_name = String::from("Khoirurrizal");
    let full_name = full_name_reference(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

// ------------------------------------
// ----- CANNOT MUTABLE REFERENCE -----
// ------------------------------------
// fn change_value (value: &String) {
//     value.push_str("Test");
// }

// #[test]
// fn change_value_test () {
//     let mut value = String::from("Fikri");
//     change_value(&value);
//     println!("{}", value);
// }

fn change_value (value: &mut String) {
    value.push_str(" Test");
}

#[test]
fn change_value_test () {
    let mut value = String::from("Fikri");

    let value_borrow = &mut value;
    // let value_borrow_2 = &mut value; // error cannot muttable reference more than once (same value)

    change_value(value_borrow);
    // change_value(value_borrow_2); // error cannot muttable reference more than once (same value)

    let mut value2 = String::from("Wado");
    let value2_borrow = &mut value2;
    change_value(value2_borrow);

    println!("{}", value);
}

// ------------------------------------
// --------- DANGLING POINTER ---------
// ------------------------------------
// fn full_name_dangling_pointer (first_name: &String, last_name: &String) -> &String {
//     format!("{} {}", first_name, last_name)
// }

// #[test]
// fn full_name_dangling_pointer_test () {
//     let first_name = String::from("Moch Fikri");
//     let last_name = String::from("Khoirurrizal");
//     let full_name = full_name_dangling_pointer(&first_name, &last_name);

//     println!("{}", full_name);
//     println!("{}", first_name);
//     println!("{}", last_name);
// }

#[test]
fn slice_reference_test () {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice_test () {
    let name = String::from("Moch Fikri Koirurrizal");

    let first_name: &str = &name[0..10];
    println!("{}", first_name);

    let last_name: &str = &name[11..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8
}

impl Person {
    fn say_hello (&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn struct_person_test () {
    let first_name = String::from("Moch");
    let last_name = String::from("Khoirurrizal");

    let person: Person = Person {
        first_name,
        middle_name: String::from("Fikri"),
        last_name, // shorthand
        age: 24
    };

    // println!("{}", person.first_name);
    // println!("{}", person.middle_name);
    // println!("{}", person.last_name);
    // println!("{}", person.age);

    // println!("{}", first_name); // error ownership
    print_person(&person);

    // let person2: Person = Person { ..person };
    // print_person(&person2);

    // println!("{}", person.first_name); // error ownership

    let person2: Person = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person2);
    println!("{}", person.first_name);
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn tupple_struct_test () {
    let geo_point = GeoPoint(-6.123123, 71.321321);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;

#[test]
fn nothing_test () {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing {};
}

#[test]
fn method_test () {
    let person: Person = Person {
        first_name: String::from("Moch"),
        middle_name: String::from("Fikri"),
        last_name: String::from("Khoirurrizal"),
        age: 24
    };

    person.say_hello("Budi");

    println!("{}", person.first_name);
}

#[test]
fn associated_function_test () {
    let geo_point: GeoPoint = GeoPoint::new(10.0, 10.0);
    // geo_point.new(10.0, 10.0); // error -> new is associated function not method

    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum
}

#[test]
fn enum_test () {
    let _level1: Level = Level::Regular;
    let _level2: Level = Level::Premium;
    let _level3: Level = Level::Platinum;
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String)
}

impl Payment {
    fn pay (&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with ewallet {} {} amount {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn payment_test () {
    let _payment1: Payment = Payment::CreditCard(String::from("12341234"));
    _payment1.pay(50000);

    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("12341234"));
    _payment2.pay(500000);

    let _payment3: Payment = Payment::EWallet(String::from("DANA"), String::from("12341234"));
    _payment3.pay(5000000);
}

#[test]
fn enum_match_test () {
    let level: Level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

#[test]
fn match_value_test () {
    let name = "Fikri";

    match name {
        "Fikri" => {
            println!("Hello Fikri");
        }
        "Wado" => {
            println!("Hello Wado");
        }
        other => {
            println!("Hello {}", other);
        }
    }

    match name {
        "Fikri" | "Wado" => {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn range_patterns_test () {
    let value = 100;

    match value {
        75..100 => { // sudah di dukung -> exclusively
            println!("Great");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        other => {
            println!("Invalid value {}", other);
        }
    }
}

#[test]
fn struct_patterns_test () {
    let point = GeoPoint::new(2.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long : {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long : {}, lat : {}",long, lat);
        }
    }

    let person: Person = Person {
        first_name: String::from("Moch"),
        middle_name: String::from("Fikri"),
        last_name: String::from("Khoirurrizal"),
        age: 24
    };

    match person {
        Person { first_name, last_name, .. } => {
            println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn ignoring_test () {
    let point = GeoPoint::new(2.0, 1.0);

    match point {
        GeoPoint(long, _) => {
            println!("long : {}", long);
        }
    }
}


#[test]
fn ignoring_range_test () {
    let value = 100;

    match value {
        75..=100 => { // sudah di dukung -> exclusively
            println!("Great");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        _ => {
            println!("Invalid value");
        }
    }
}
