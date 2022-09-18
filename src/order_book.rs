use std::collections::BTreeMap;
use crate::data_types::*;
use crate::matching;

use ordered_float::OrderedFloat;
use std::cmp::Ord;
use std::cmp::Ordering;

pub trait Creator
{
    /// Create a new type
    /// 
    /// # Arguments
    /// 
    /// * `price` - The price to take in consideration
    fn create(price : f32) -> Self;
}

#[derive(PartialEq,Eq, Ord, PartialOrd, Debug)]
pub struct AskKey(OrderedFloat<f32>);

impl Creator for AskKey
{
    fn create(price: f32) -> Self
    {
        Self(OrderedFloat(price))
    }
}

//////////////////////////// BIDKEY /////////////////////////////// 

#[derive(Debug)]
pub struct BidKey(OrderedFloat<f32>);

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
    pub _ask : BTreeMap<AskKey, Limit>,
    pub _trades : Vec<Trade>
}

fn insert_order<T : Ord + Creator>(curr_side : &mut BTreeMap<T, Limit>, order : Order)
{
    let key = T::create(order.price);
    let curr_limit = curr_side.entry(key).or_insert(Limit::new(order.price));
    curr_limit.add_order(order);        
}

impl OrderBook {

    pub fn new(symbol: &str) -> OrderBook
    {
        OrderBook { _symbol : symbol.to_string(), 
                    _bid: BTreeMap::new(), 
                    _ask: BTreeMap::new(),
                    _trades : vec![]}
    }

    pub fn insert_order_at_level(&mut self, order: &mut Order)
    {
        match &order.side
        {
            Side::Buy => 
            {
                // If I find something at a lower or equal price respect to what I want to buy
                let match_bid_strategy = |best_availiable_price, current_offered_price| 
                {
                    OrderedFloat(best_availiable_price) <= OrderedFloat(current_offered_price)
                };

                let mut bid_trades = matching::match_order(&mut self._ask, &match_bid_strategy, order);
                if !bid_trades.is_empty()
                {
                    self._trades.append(&mut bid_trades);
                }

                if order.qty == 0
                {
                    return
                }
                
                insert_order(&mut self._bid, *order);
            },
            Side::Sell => 
            {
                // If I find something at a higher or equal price respect to what I want to buy
                let match_ask_strategy = |best_availiable_price, current_offered_price| 
                {
                    OrderedFloat(best_availiable_price) >= OrderedFloat(current_offered_price)
                };
                let mut ask_trades = matching::match_order(&mut self._bid, &match_ask_strategy, order);
                if !ask_trades.is_empty()
                {
                    self._trades.append(&mut ask_trades);
                }
                if order.qty == 0
                {
                    return
                }

                insert_order(&mut self._ask, *order)
            },
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
        let order_book = OrderBook::new("AAPL");
        println!("{:?}", order_book);
        assert_eq!(order_book._symbol, "AAPL");
    }

    #[test]
    fn insert_order_at_level_is_correct()
    {
        let mut order_book = OrderBook::new("AAPL");
        let mut order = Order{id:1, side:Side::Sell, price:12.2f32, qty:100};
        order_book.insert_order_at_level(&mut order);
        let mut order2 = Order{id:1, side:Side::Sell, price:12.5f32, qty:100};
        order_book.insert_order_at_level(&mut order2);
        let best_price = order_book._ask.iter().next();
        println!("{:?}", order_book);
        println!("Best Price = {:?}", best_price);
        assert_eq!((*best_price.unwrap().1).price, 12.2f32);
    }

    #[test]
    fn insert_multiple_orders()
    {
        let mut order_book = OrderBook::new("AAPL");
        let mut order = Order{id:1, side:Side::Buy, price:12.2f32, qty:100};
        order_book.insert_order_at_level(&mut order);
        let mut order2 = Order{id:2, side:Side::Sell, price:12.5f32, qty:25};
        order_book.insert_order_at_level(&mut order2);
        println!("{:?}", order_book);
        assert_eq!(order_book._bid.len(), 1);
        assert_eq!(order_book._ask.len(), 1);
    }

    #[test]
    fn insert_multiple_orders_at_same_level()
    {
        let mut order_book = OrderBook::new("AAPL");
        let mut order = Order{id:1, side:Side::Sell, price:12.2f32, qty:100};
        order_book.insert_order_at_level(&mut order);
        let mut order2 = Order{id:2, side:Side::Sell, price:12.2f32, qty:25};
        order_book.insert_order_at_level(&mut order2);
        println!("{:?}", order_book);
        assert_eq!(order_book._ask.len(), 1);
        assert_eq!(order_book.best_ask().unwrap().num_orders(), 2);
        assert_eq!(order_book.best_ask().unwrap().qty, 125);
    }

    #[test]
    fn insert_multiple_orders_at_different_level()
    {
        let mut order_book = OrderBook::new("AAPL");
        let mut order = Order{id:1, side:Side::Sell, price:12.2f32, qty:100};
        let mut order2 = Order{id:2, side:Side::Sell, price:12.2f32, qty:25};
        let mut order3 = Order{id:3, side:Side::Sell, price:12.5f32, qty:25};
        order_book.insert_order_at_level(&mut order);
        order_book.insert_order_at_level(&mut order2);
        order_book.insert_order_at_level(&mut order3);

        // Add buy orders
        let mut order4 = Order{id:4, side:Side::Buy, price:12.1f32, qty:100};
        let mut order5 = Order{id:5, side:Side::Buy, price:12.1f32, qty:25};
        let mut order6 = Order{id:6, side:Side::Buy, price:12.15f32, qty:25};
        order_book.insert_order_at_level(&mut order4);
        order_book.insert_order_at_level(&mut order5);
        order_book.insert_order_at_level(&mut order6);

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

    #[test]
    fn cannot_insert_empty_order()
    {
        let mut order_book = OrderBook::new("AAPL");
        let mut _id : u32 = 1;
        let mut order = Order{id:_id, side:Side::Buy, price:12.2f32, qty:0};
        order_book.insert_order_at_level(&mut order);
        assert_eq!(order_book.best_bid().is_none(), true);
        assert_eq!(order_book.best_ask().is_none(), true);
        let mut empty_sell_order = Order{id:_id, side:Side::Sell, price:12.2f32, qty:0};
        order_book.insert_order_at_level(&mut empty_sell_order);
    }

    #[test]
    fn match_orders_multiple()
    {
        let mut order_book = OrderBook::new("AAPL");
        let mut _id : u32 = 1;
        let mut order = Order{id:_id, side:Side::Buy, price:12.2f32, qty:100};
        _id += 1;

        let mut order2 = Order{id:_id, side:Side::Buy, price:12.2f32, qty:25};
        _id += 1;

        let mut order3 = Order{id:_id, side:Side::Buy, price:12.5f32, qty:25};
        _id += 1;

        let mut order4 = Order{id:_id, side:Side::Buy, price:12.7f32, qty:25};
        _id += 1;

        order_book.insert_order_at_level(&mut order);
        order_book.insert_order_at_level(&mut order2);
        order_book.insert_order_at_level(&mut order3);
        order_book.insert_order_at_level(&mut order4);

        // Add sell orders
        let mut order5 = Order{id:_id, side:Side::Sell, price:12.2f32, qty:100};
        _id += 1;

        order_book.insert_order_at_level(&mut order5);
        println!("trades = {:?}", order_book._trades);
        let t1 = Trade::new(5, 4, 12.7f32, 25);
        let t2 = Trade::new(5, 3, 12.5f32, 25);
        let t3 = Trade::new(5, 1, 12.2f32, 50);
        let mut expected_trades = vec![t1,t2,t3];
        assert_eq!(order_book._trades, expected_trades);
        assert_eq!(order_book._trades.len(), 3);
        assert_eq!(order_book.best_bid().is_some(), true);
        assert_eq!(order_book.best_bid().unwrap().qty, 75);

        assert_eq!(order_book.best_bid().unwrap().num_orders(), 2);
        let exp_order1 = Order{id:1, side:Side::Buy, price:12.2f32, qty:50};
        let exp_order2 = Order{id:2, side:Side::Buy, price:12.2f32, qty:25};
        let expected_orders_at_level = vec![exp_order1, exp_order2];
        assert_eq!(order_book.best_bid().unwrap().orders, expected_orders_at_level);
    
        let mut order6 = Order{id:_id, side:Side::Sell, price:12.1f32, qty:25};
        _id += 1;

        order_book.insert_order_at_level(&mut order6);
        assert_eq!(order_book._trades.len(), 4);
        expected_trades.push(Trade::new(6, 1, 12.2, 25));
        assert_eq!(order_book._trades, expected_trades);

        let mut order7 = Order{id:_id, side:Side::Sell, price:12.01f32, qty:50};
        _id += 1;

        expected_trades.push(Trade::new(7, 1, 12.2, 25));
        expected_trades.push(Trade::new(7, 2, 12.2, 25));

        // INSERT LAST ORDER IN THE ORDER BOOK
        order_book.insert_order_at_level(&mut order7);

        assert_eq!(order_book._trades.len(), 6);
        assert_eq!(order_book._trades, expected_trades);
        assert_eq!(order_book.best_bid().is_none(), true);
        assert_eq!(order_book.best_ask().is_none(), true);
    }


}
