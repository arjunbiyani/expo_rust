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
    fn render_fullname(&self) -> String{
        format!("{} {}",self.p_name,self.p_name)
    }
    fn change_product_name(&mut self, pname: &str) {
        self.p_name=pname.to_string(); 
    }
}

//creating function 




pub fn run(){
    let mut p = Product::new(1,"Macbook",222);
   p.change_product_name("Macbook New");
    let mut l = location(1,String::from("Bangalore"),3);
    //p.p_name=String::from("Macbook ");
    println!("{} ",p.render_fullname());
    println!("location {} - {} - {} ",l.0,l.1,l.2 );
}
