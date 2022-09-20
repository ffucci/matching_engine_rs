use matching_engine::order_book::OrderBook;
use matching_engine::data_types::{Order, Side};
use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};
use tokio::io::{self, AsyncWriteExt};
use bytes::{BytesMut, Buf};
use bytes::BufMut;
use std::io::Cursor;
use byteorder::{NetworkEndian};


#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6001").await.unwrap();
    let mut order_book = Box::new(OrderBook::new("TSLA"));
    
    loop {
        // The second item contains the IP and port of the new connection.
        let (mut socket, _) = listener.accept().await.unwrap();
        process(&mut socket, &mut order_book).await;
    }
}

fn decode_order(buf : &mut BytesMut) -> Option<Order>
{
    // Decode ID
    let mut id_arr = [0u8; 4];
    let mut pos_id = buf.split_to(4);
    id_arr.copy_from_slice(&pos_id[..4]);
    let received_id =  u32::from_be_bytes(id_arr);

    // Decode SIDE
    let mut side = Side::Buy;
    let side_mut = buf.split_to(1);
    let received_side = u8::from_be(side_mut[0]);
    if received_side == 0x2
    {
        side = Side::Sell;
    }

    // Decode price
    let mut price_arr = [0u8; 4];
    let price_buf = buf.split_to(4);
    price_arr.copy_from_slice(&price_buf[..4]);
    let received_price = f32::from_be_bytes(price_arr);

    // Decode qty
    let mut qty_arr = [0u8; 4];
    let qty_buf = buf.split_to(4);
    qty_arr.copy_from_slice(&qty_buf[..4]);
    let received_qty = u32::from_be_bytes(qty_arr);

    Some(Order::new(received_id, side, received_price, received_qty))
}

async fn process(socket: &mut TcpStream, order_book : &mut OrderBook) {
    println!("socket {:?}", socket);
    let mut rx_bytes = Vec::new();
    socket.read_to_end(&mut rx_bytes).await;
    println!("Received bytes: {:?}", rx_bytes);
    let mut bytes_mut = BytesMut::from(rx_bytes.as_slice());
    while !bytes_mut.is_empty()
    {
        let received_order = decode_order(&mut bytes_mut);
        println!("Received order: {:?}", received_order);
        order_book.insert_order_at_level(&mut received_order.unwrap());
    }

    order_book.summary();
}
