fn main() 
{
    let a = 5;
    let b = "twenty seven";
    let c = 10.25f32;

    // a = 27;

    println!("The value of 'a' is {}", a);
    println!("The value of 'b' is {}", b);
    println!("The value of 'c' is {}", c);

    // Full type list here: https://doc.rust-lang.org/nightly/std/
    let _d:u32;
    let _e:i64 = 5150;
    let _f:String = String::from("Hello There!");

    ifconditional();
    whileloop();
    forloop();
    stringslice();
}

fn ifconditional()
{
    let a = 1;
    let b = 2;
    
    if a > b
    {
        println!("1 > 2");
    }
    else
    {
        println!("1 < 2");
    }
}

fn whileloop()
{
    let mut done = false;
    
    while !done
    {
        println!("Printing only once");
        done = true;
    }
}

fn forloop()
{
    for index in 0..5
    {
        println!("print loop {}", index);
    }    
}

fn stringslice()
{
    let s = String::from("Here Is An Example String");
    let first = s[0..4];
    let second = &s[5..7];
    let third = &s[8..10];
    let fourth = &s[11..18];
    let fifth = &s[19..25];

    println!("{}-{}-{}-{}-{}", first, second, third, fourth, fifth);
}
