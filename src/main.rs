// #[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    inStock: bool,
}

enum Category {
    Electronics,
    Furniture,
    Learning,
}
impl Product {
    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    fn buy_one(&self) {
        println!("{} was bought for: {}", self.name, self.price);
    }
    fn new(name: String, price: f32) -> Product {
        Product {
            name,
            price,
            inStock: true,
        }
    }
}

fn main() {
    let mut Laptop = Product::new("Lenovo Ideapad".to_string(), 230.9);
    let sales_tax = Laptop.calculate_sales_tax();
    Laptop.set_price(2000.99);
    Laptop.buy_one();
    println!("sales tax for {} is: {}", Laptop.name, sales_tax);
}
