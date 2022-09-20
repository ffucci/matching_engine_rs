#![crate_name = "doc"]
use std::cmp;
use ordered_float::OrderedFloat;

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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Trade
{
    pub aggressive_id : u32,
    pub passive_id : u32,
    pub price : f32,
    pub qty : u32,
}

impl Trade
{
    pub fn new(aggressive_id : u32, passive_id : u32, price : f32, qty : u32) -> Trade
    {
        Trade { aggressive_id: aggressive_id, 
                passive_id: passive_id, price: price, qty: qty }
    }
}

#[derive(Debug)]
pub struct Limit
{
    pub price : f32,
    pub qty : u32,
    pub orders: Vec<Order>,
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
            None => Err("cannot remove order from limit"),
        }
    }

    pub fn make_trades(&mut self, aggressive_order : &mut Order) -> Vec<Trade>
    {
        // Cannot make any trade if the price do not match or is empty
        let mut trades = Vec::new();

        loop
        {
            if aggressive_order.qty == 0 || self.orders.is_empty()
            {
                break;
            }

            let mut need_to_remove = false;
            if let Some(passive_order) = self.orders.first_mut()
            {
                let traded_quantity = cmp::min(aggressive_order.qty, passive_order.qty);
                aggressive_order.qty -= traded_quantity;
                passive_order.qty -= traded_quantity;
                self.qty -= traded_quantity;
                if passive_order.qty == 0
                {
                    need_to_remove = true;
                }
                println!("passive_order {:?}", passive_order);
                println!("aggressive_order {:?}", aggressive_order);
                trades.push(Trade{aggressive_id : aggressive_order.id, 
                    passive_id : passive_order.id, 
                    price : passive_order.price, 
                    qty : traded_quantity})
            }

            if need_to_remove
            {
                self.orders.remove(0);
            }
        }
        
        return trades;
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
        let mut limit = Limit::new(12.2f32);
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


    #[test]
    fn can_make_trade()
    {
        let mut limit = Limit::new(12.2f32);
        let order = Order{id:1, side: Side::Buy, price:12.2f32, qty:100};
        let order2 = Order{id:2, side: Side::Buy, price:12.2f32, qty:22};
        let order3 = Order{id:3, side: Side::Buy, price:12.2f32, qty:44};

        limit.add_order(order);
        limit.add_order(order2);
        limit.add_order(order3);

        assert_eq!(limit.qty, 166);

        let mut order_to_match = Order{id : 4, side : Side::Sell, price:12.2f32, qty : 90};
        let trades = limit.make_trades(&mut order_to_match);
        assert_eq!(trades.len(), 1);
        assert_eq!(limit.num_orders(), 3);
        assert_eq!(limit.qty, 166 - 90);

        println!("trades = {:?}", trades);
    }

    #[test]
    fn removing_empty_order_gives_error()
    {
        let mut limit = Limit::new(12.2f32);
        let val = limit.remove_order(0);
        assert_eq!(val.is_err(), true);
        assert_eq!(val, Err("cannot remove order from limit"));
    }
}