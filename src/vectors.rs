

pub fn run()
{
   
    let mut numbers:Vec<i32> = vec![1,2,3];
    numbers.push(11);
    println!("{}",numbers[0]);

    //looping through vectors
    for x in numbers.iter(){
        println!("{}",x);
    }

    //mutatio

    for x in numbers.iter_mut(){
        *x *=2;
        
    }
    println!("{:?}",numbers);
}