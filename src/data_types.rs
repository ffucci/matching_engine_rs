
#[derive(Debug)]
pub enum Side
{
    Buy,
    Sell
}

#[derive(Debug)]
pub struct Order
{
    pub id : u32,
    pub side : Side,
    pub price : f32,
    pub qty : u32,
}

pub struct Limit
{
    pub price : f32,
    qty : u32,
    orders: Vec<Order>,
}

impl Limit
{
    pub fn new(price:f32) -> Self
    {
        Self{
            price,
            qty: 0,
            orders : vec![],
        }
    }

    pub fn add_order(&mut self, order : Order)
    {
        self.qty = self.qty + order.qty;
        self.orders.push(order);
    }

}

#[cfg(test)]
mod tests
{
    use crate::data_types::Order;
    use crate::data_types::Limit;
    use crate::data_types::Side;

    #[test]
    fn try_order()
    {
        let order = Order{id:1, side: Side::Buy, price:12.2f32, qty:100};
        println!("{:?}", order);
    }

    #[test]
    fn can_add_order()
    {
        let mut limit = Limit::new(12.12);
        let order = Order{id:1, side:Side::Sell, price:12.2f32, qty:100};
        limit.add_order(order);
        assert_eq!(limit.qty, 100);
        assert_eq!(limit.orders.len(), 1);
    }

    #[test]
    fn can_add_multiple_order()
    {
        let mut limit = Limit::new(12.12);
        let order = Order{id:1, side: Side::Buy, price:12.2f32, qty:100};
        let order2 = Order{id:2, side: Side::Buy, price:12.2f32, qty:22};
        limit.add_order(order);
        limit.add_order(order2);
        assert_eq!(limit.qty, 122);
        assert_eq!(limit.orders.len(), 2);
    }
}