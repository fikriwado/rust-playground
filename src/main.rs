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
