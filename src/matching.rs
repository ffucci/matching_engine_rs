use std::collections::BTreeMap;
use crate::data_types;

trait OrderMatcher<T>
{
    fn perform_match();
}

type F = fn(f32, f32) -> bool;

fn match_order<T : Ord + Creator>(curr_side : &mut BTreeMap<T, Limit>, 
                                  can_trade : fn(f32, f32) -> bool, 
                                  order : Order) -> (Option<Order>, Vec<Trade>)
{
    let mut trades = Vec::new();
    // Here we need to match first and then we can do the rest
    while let mut best_price = curr_side.iter().next() && order.qty > 0
    {
        // If there is no order anymore at a certain level let's clean the level
        if best_price.orders.is_empty()
        {
            curr_side.remove(best_price);
        }

        if !can_trade(order.price, best_price)
        {
            break;
        }

        // // Make trades up until we can and reduce the qty accordingly
        // while let curr_trade = best_price.make_trade(order)
        // {
        //     trades.push(curr_trade);
        // }
    }

    if order.qty == 0
    {
        return (None, trades);        
    }

    return (order, trades);
}

#[cfg(test)]
mod tests
{

}