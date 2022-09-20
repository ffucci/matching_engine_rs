use tokio::net::TcpStream;
use bytes::BytesMut;
use bytes::BufMut;
use tokio::io::{AsyncWriteExt};
use std::error::Error;
use std::io::Read;
use matching_engine::data_types::{Order, Side};

trait Encoder
{
    fn encode(&self, buf : &mut BytesMut);
}
impl Encoder for Order
{
    fn encode(&self, buf : &mut BytesMut)
    {
        // buf.reserve(8);
        buf.put_u32(self.id);

        match self.side
        {
            Side::Buy => { buf.put_u8(0x1); },
            Side::Sell => { buf.put_u8(0x2); },
        }

        buf.put_f32(self.price);

        println!("buffer size: {}", buf.len());
    }
}

const MAX_SEQ : u32 = 100u32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:6001").await?;

    let mut i = 0u32;
    while i <= MAX_SEQ
    {
        let mut side = Side::Buy;
        if i % 5 == 0
        {
            side = Side::Sell;
        }

        let order = Order{id:i, side: side, price:12.2f32, qty:100};
        println!("Sending order: {:?}", order);
        // Write some data.
        let mut buffer = BytesMut::new();
        
        // Encode order
        order.encode(&mut buffer);

        stream.write_all(&buffer).await?;
        i += 1;
    }

    stream.flush();

    Ok(())
}