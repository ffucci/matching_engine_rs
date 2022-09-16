use std::{collections::BTreeMap, ops::Deref};
use crate::data_types::*;
use crate::order_book::*;

type F = fn(f32, f32) -> bool;

fn match_order<T : Ord >(curr_side : &mut BTreeMap<T, Limit>, 
                        can_trade : &dyn Fn(f32, f32) -> bool, 
                        order : &mut Order) -> Vec<Trade>
{
    let mut trades = Vec::new();
    // Here we need to match first and then we can do the rest
    loop 
    {
        if order.qty == 0
        {
            break;
        }

        let best_price_level = &mut curr_side.iter().next();
        let best_price = best_price_level.unwrap().0;
        let limit = curr_side.iter_mut().next().unwrap().1;

        // If there is no order anymore at a certain level let's clean the level
        if limit.num_orders() == 0
        {
        }

        if !can_trade(order.price, limit.price)
        {
            break;
        }

        // Make trades up until we can and reduce the qty accordingly
        let mut trades_at_price = limit.make_trades(order);
        trades.append(&mut trades_at_price);
    }

    if order.qty == 0
    {
        return trades;        
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

    use super::match_order;

    #[test]
    fn can_match()
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
        let match_strategy = |best_availiable_price, current_offered_price| OrderedFloat(best_availiable_price) >= OrderedFloat(current_offered_price);
        let mut order_to_match = Order{id:4, side: Side::Sell, price:12.2f32, qty:50};
        let trades = match_order(&mut m, &match_strategy, &mut order_to_match);
        println!("trades = {:?}", trades);
        assert_eq!(trades.len(), 1);
        let trade = Trade::new(4, 1, 12.2f32, 50);
        let exp = Some(&trade);
        assert_eq!(trades.last(), exp);
    }
}