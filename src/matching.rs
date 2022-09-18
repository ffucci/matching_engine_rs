use std::collections::BTreeMap;
use crate::data_types::*;

pub fn match_order<T : Ord >(curr_side : &mut BTreeMap<T, Limit>, 
                         can_trade : &dyn Fn(f32, f32) -> bool, 
                         order : &mut Order) -> Vec<Trade>
{
    let mut trades = Vec::new();
    // Here we need to match first and then we can do the rest
    loop 
    {
        // If there are no order anymore at a certain level let's clean the level
        curr_side.retain(|k, level| level.qty != 0 && level.num_orders() != 0);

        if order.qty == 0 || curr_side.is_empty()
        {
            break;
        }

        let price_level = curr_side.iter_mut().next();
        if price_level.is_none()
        {
            break;
        }

        let (_, limit) = price_level.unwrap();
        if !can_trade(limit.price, order.price)
        {
            break;
        }

        println!("Can trade @price {0} ", limit.price);
        // Make trades up until we can and reduce the qty accordingly
        let mut trades_at_price = limit.make_trades(order);
        trades.append(&mut trades_at_price);
    }

    return trades;
}

#[cfg(test)]
mod tests
{
    use ordered_float::OrderedFloat;
    use crate::data_types::{Order, Limit, Side, Trade};
    use std::collections::BTreeMap;
    use crate::order_book::*;

    // Module under test
    use super::match_order;

    #[test]
    fn can_match_with_bid()
    {
        let mut m = BTreeMap::new();
        let mut limit = Limit::new(12.2f32);
        let order = Order{id:1, side: Side::Buy, price:12.2f32, qty:100};
        let order2 = Order{id:2, side: Side::Buy, price:12.2f32, qty:22};
        let order3 = Order{id:3, side: Side::Buy, price:12.2f32, qty:33};

        limit.add_order(order);
        limit.add_order(order2);
        limit.add_order(order3);
        assert_eq!(limit.qty, 155);
        println!("limit = {:?}", limit);

        let bidkey = BidKey::create(12.2f32);
        m.insert(bidkey, limit);
        let match_strategy = |best_availiable_price, current_offered_price| 
        {
            OrderedFloat(best_availiable_price) >= OrderedFloat(current_offered_price)
        };

        let mut order_to_match = Order{id:4, side: Side::Sell, price:12.2f32, qty:50};
        let trades = match_order(&mut m, &match_strategy, &mut order_to_match);
        println!("trades = {:?}", trades);
        assert_eq!(trades.len(), 1);
        let trade = Trade::new(4, 1, 12.2f32, 50);
        let expected_trade = Some(&trade);
        assert_eq!(trades.last(), expected_trade);
    }

    #[test]
    fn can_match_multiple_times_with_bid()
    {
        let mut m = BTreeMap::new();
        let mut limit = Limit::new(12.2f32);
        let order = Order{id:1, side: Side::Buy, price:12.2f32, qty:100};
        let order2 = Order{id:2, side: Side::Buy, price:12.2f32, qty:22};
        let order3 = Order{id:3, side: Side::Buy, price:12.2f32, qty:33};

        limit.add_order(order);
        limit.add_order(order2);
        limit.add_order(order3);

        let bidkey = BidKey::create(12.2f32);
        m.insert(bidkey, limit);
        let match_strategy = |best_availiable_price, current_offered_price| 
        {
            OrderedFloat(best_availiable_price) >= OrderedFloat(current_offered_price)
        };

        let mut order_to_match = Order{id:4, side: Side::Sell, price:12.2f32, qty:135};
        let trades = match_order(&mut m, &match_strategy, &mut order_to_match);
        println!("trades = {:?}", trades);
        assert_eq!(trades.len(), 3);
        assert_eq!(m.iter().next().unwrap().1.qty, 155-135);

        // Expected trades
        let trade1 = Trade::new(4, 1, 12.2f32, 100);
        let trade2= Trade::new(4, 2, 12.2f32, 22);
        let trade3 = Trade::new(4, 3, 12.2f32, 13);
        let expected_trades = vec![trade1,trade2,trade3];
        // let expected_trade = Some(&trade);
        assert_eq!(trades, expected_trades);
    }
}