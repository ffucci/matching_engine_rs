mod data_types;
mod order_book;
mod matching;

fn main() {
    let order = data_types::Order{id:1, side: data_types::Side::Buy, price:12.2f32, qty:100};
    println!("{:?}", order);
}
