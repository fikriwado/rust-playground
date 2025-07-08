fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test()
{
    println!("Hello Test");
}

#[test]
fn variable_test()
{
    let name = "Moch Fikri Khoirurrizal";
    println!("Hello {}", name);
}

#[test]
fn mut_variable_test()
{
    let mut name = "Moch Fikri Khoirurrizal";
    println!("Hello {}", name);

    name = "Fikri Wado";
    println!("Hello {}", name);
}

#[test]
fn static_typing_test()
{
    // let mut name = "Moch Fikri Khoirurrizal";
    let name = "Moch Fikri Khoirurrizal";
    println!("Hello {}", name);

    // name = 15;
    println!("Hello {}", name);
}

#[test]
fn shadowing_test()
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
fn comment_test()
{
    //Ini komen juga bos
    println!("Hello");
}

#[test]
fn explicit_test()
{
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn number_test()
{
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion_test()
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
fn numeric_test()
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
fn augmented_test()
{
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean_test()
{
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison_test()
{
    let a = 10;
    let b = 20;
    let result: bool = a > b;

    println!("{}", result);
}

#[test]
fn boolean_operator_test()
{
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;

    println!("{}", lulus);
}
