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
