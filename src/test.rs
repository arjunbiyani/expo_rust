
pub fn run()
{
    let a ="Arjun";
    let b="Biyani";
    println!("This is from run function of test");
    
    //Positional Params
    println!("these are positional args {0}. My Name is {0} {1}",a,b);

    //Named Arguments 
    println!("{name} likes to {activity}",
    name="Arjun",
    activity="code"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}",10,10,10 );

    //Placeholder for debug triaits
    println!("{:?}",("heo"));

    // Basic Math 
    println!("SUM 3+4 {}",3+4);

}