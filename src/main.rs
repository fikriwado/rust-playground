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
