pub fn run()
{

    let mut hello= String::from("this is string");
    // Get Length 
    println!("Length : {}",hello.len());
    hello.push_str("World with push ");
    println!("{}",hello);
}