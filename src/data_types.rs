#![crate_name = "doc"]

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side
{
    Buy,
    Sell
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Order
{
    pub id : u32,
    pub side : Side,
    pub price : f32,
    pub qty : u32,
}

#[derive(Debug)]
pub struct Limit
{
    pub price : f32,
    pub qty : u32,
    orders: Vec<Order>,
}

impl Limit
{
    /// Creates a new limit object which represents a limit with a vector of orders
    /// 
    /// # Arguments
    /// 
    /// * `price` - a float value representing the new limit on which the limit is constructed
    pub fn new(price:f32) -> Self
    {
        Self{
            price,
            qty: 0,
            orders : vec![],
        }
    }

    /// Adds a new order to the vector of orders, the orders are added in FIFO fashion
    /// 
    /// # Arguments
    /// 
    /// * `order` - The order to be added at that specific limit
    pub fn add_order(&mut self, order : Order)
    {
        self.qty = self.qty + order.qty;
        self.orders.push(order);
    }

    pub fn remove_order(&mut self, order_id: u32) -> Result<Order, &str>
    {
        let pos = self.orders.iter().position(|&ord| ord.id == order_id);
        match pos
        {
            Some(upos) => 
            {
                self.qty -= self.orders[upos].qty;
                Ok(self.orders.remove(upos))
            },
            None => Err("cannot remove"),
        }
    }

    pub fn num_orders(&self) -> usize
    {
        self.orders.len()
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

    #[test]
    fn can_remove_order_at_limit()
    {
        let mut limit = Limit::new(12.12);
        let order = Order{id:1, side: Side::Buy, price:12.2f32, qty:100};
        let order2 = Order{id:2, side: Side::Buy, price:12.2f32, qty:22};
        limit.add_order(order);
        limit.add_order(order2);
        assert_eq!(limit.qty, 122);
        let res = limit.remove_order(1);

        println!("{:?}",res.unwrap());
        assert_eq!(res.unwrap(), order);
        assert_eq!(limit.orders.len(), 1);
        assert_eq!(limit.qty, 22);
    }
}