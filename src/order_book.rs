use std::collections::BTreeMap;
use crate::data_types::*;

use ordered_float::OrderedFloat;
use std::cmp::Ord;
use std::cmp::Ordering;


trait Creator
{
    /// Adds a new order to the vector of orders, the orders are added in FIFO fashion
    /// 
    /// # Arguments
    /// 
    /// * `price` - The price to take in consideration
    fn create(price : f32) -> Self;
}

#[derive(PartialEq,Eq, Ord, PartialOrd, Debug)]
struct AskKey(OrderedFloat<f32>);

impl Creator for AskKey
{
    fn create(price: f32) -> Self
    {
        Self(OrderedFloat(price))
    }
}

//////////////////////////// BIDKEY /////////////////////////////// 

#[derive(Debug)]
struct BidKey(OrderedFloat<f32>);

impl Creator for BidKey
{
    fn create(price: f32) -> Self
    {
        Self(OrderedFloat(price))
    }
}

impl PartialEq for BidKey
{
    fn eq(&self, other : &Self) -> bool
    {
        return self.0 == other.0;
    }
}

impl Eq for BidKey
{}

impl Ord for BidKey
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        if self.0 > other.0
        {
            Ordering::Less
        } else if self.0 < other.0
        {
            Ordering::Greater
        } else
        {
            Ordering::Equal
        }
    }
}

impl PartialOrd for BidKey
{
    fn partial_cmp(&self, other : &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

//////////////////////////// ORDERBOOK /////////////////////////////// 


#[derive(Debug)]
struct OrderBook
{
    _symbol: String,
    pub _bid : BTreeMap<BidKey, Limit>,
    pub _ask : BTreeMap<AskKey, Limit>
}

fn insert_order<T : Ord + Creator>(curr_side : &mut BTreeMap<T, Limit>, order : Order)
{
    let key = T::create(order.price);
    // Here we need to match first and then we can do the rest
    let curr_limit = curr_side.entry(key).or_insert(Limit::new(order.price));
    curr_limit.add_order(order);        
}


impl OrderBook {

    pub fn new(symbol: &str) -> OrderBook
    {
        OrderBook { _symbol : String::from(symbol), 
                    _bid: BTreeMap::new(), 
                    _ask: BTreeMap::new()}
    }

    pub fn insert_order_at_level(&mut self, order: Order)
    {
        match &order.side
        {
            Side::Buy => insert_order(&mut self._bid, order),
            Side::Sell => insert_order(&mut self._ask, order),
        }
    }

    pub fn best_bid(&self) -> Option<&Limit>
    {
        let val = self._bid.iter().next();
        match &val
        {
            Some((_, l)) => Some(l),
            None => None,
        }
    }

    pub fn best_ask(&self) -> Option<&Limit>
    {
        let val = self._ask.iter().next();
        match &val
        {
            Some((_, l)) => Some(l),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::order_book::OrderBook;
    use crate::data_types::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn create_order_book()
    {
        let mut order_book = OrderBook::new("AAPL");
        println!("{:?}", order_book);
    }

    #[test]
    fn insert_order_at_level_is_correct()
    {
        let mut order_book = OrderBook::new("AAPL");
        let order = Order{id:1, side:Side::Sell, price:12.2f32, qty:100};
        order_book.insert_order_at_level(order);
        let order2 = Order{id:1, side:Side::Sell, price:12.5f32, qty:100};
        order_book.insert_order_at_level(order2);
        let best_price = order_book._ask.iter().next();
        println!("{:?}", order_book);
        println!("Best Price = {:?}", best_price);
    }

    #[test]
    fn insert_multiple_orders()
    {
        let mut order_book = OrderBook::new("AAPL");
        let order = Order{id:1, side:Side::Buy, price:12.2f32, qty:100};
        order_book.insert_order_at_level(order);
        let order2 = Order{id:2, side:Side::Sell, price:12.5f32, qty:25};
        order_book.insert_order_at_level(order2);
        println!("{:?}", order_book);
        assert_eq!(order_book._bid.len(), 1);
        assert_eq!(order_book._ask.len(), 1);
    }

    #[test]
    fn insert_multiple_orders_at_same_level()
    {
        let mut order_book = OrderBook::new("AAPL");
        let order = Order{id:1, side:Side::Sell, price:12.2f32, qty:100};
        order_book.insert_order_at_level(order);
        let order2 = Order{id:2, side:Side::Sell, price:12.2f32, qty:25};
        order_book.insert_order_at_level(order2);
        println!("{:?}", order_book);
        assert_eq!(order_book._ask.len(), 1);
        assert_eq!(order_book.best_ask().unwrap().num_orders(), 2);
        assert_eq!(order_book.best_ask().unwrap().qty, 125);
    }

    #[test]
    fn insert_multiple_orders_at_different_level()
    {
        let mut order_book = OrderBook::new("AAPL");
        let order = Order{id:1, side:Side::Sell, price:12.2f32, qty:100};
        let order2 = Order{id:2, side:Side::Sell, price:12.2f32, qty:25};
        let order3 = Order{id:3, side:Side::Sell, price:12.5f32, qty:25};
        order_book.insert_order_at_level(order);
        order_book.insert_order_at_level(order2);
        order_book.insert_order_at_level(order3);

        // Add buy orders
        let order4 = Order{id:4, side:Side::Buy, price:12.1f32, qty:100};
        let order5 = Order{id:5, side:Side::Buy, price:12.1f32, qty:25};
        let order6 = Order{id:6, side:Side::Buy, price:12.15f32, qty:25};
        order_book.insert_order_at_level(order4);
        order_book.insert_order_at_level(order5);
        order_book.insert_order_at_level(order6);

        println!("{:?}", order_book);
        assert_eq!(order_book._ask.len(), 2);
        let best_ask_price = order_book.best_ask().unwrap();
        assert_eq!(best_ask_price.num_orders(), 2);
        assert_eq!(best_ask_price.qty, 125);
        assert_eq!(OrderedFloat(best_ask_price.price), OrderedFloat(12.2f32));

        assert_eq!(order_book._bid.len(), 2);
        let best_bid_price = order_book.best_bid().unwrap();
        assert_eq!(best_bid_price.num_orders(), 1);
        assert_eq!(best_bid_price.qty, 25);
        assert_eq!(OrderedFloat(best_bid_price.price), OrderedFloat(12.15f32));
    }

}
