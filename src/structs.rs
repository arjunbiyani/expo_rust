// related to classes 
struct Product {
    id: i32,
    p_name: String,
    price: i32,
    
}
// Tuple Struct
struct location(i32,String,i32);

impl Product {
    // Constructor 
    fn new(id:i32,p_name: &str,price:i32) -> Product{
        Product{
            id: id,
            p_name: p_name.to_string(),
            price: price
        }
    }
}


pub fn run(){
    let mut p = Product::new(1,"Macbook",222);

    let mut l = location(1,String::from("Bangalore"),3);
    //p.p_name=String::from("Macbook");
    println!("{} - {} - {} ",p.id,p.p_name,p.price );
    println!("location {} - {} - {} ",l.0,l.1,l.2 );
}