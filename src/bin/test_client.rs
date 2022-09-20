use rand_distr::Bernoulli;
use tokio::net::TcpStream;
use bytes::BytesMut;
use bytes::BufMut;
use tokio::io::{AsyncWriteExt};
use std::error::Error;
use matching_engine::data_types::{Order, Side};
use rand_distr::{Distribution, Normal, Uniform};
use rand::thread_rng;

const MAX_SEQ : u32 = 1000u32;

trait Encoder
{
    fn encode(&self, buf : &mut BytesMut);
}
impl Encoder for Order
{
    fn encode(&self, buf : &mut BytesMut)
    {
        buf.put_u32(self.id);

        match self.side
        {
            Side::Buy => { buf.put_u8(0x1); },
            Side::Sell => { buf.put_u8(0x2); },
        }

        buf.put_f32(self.price);
        buf.put_u32(self.qty);
        println!("buffer size: {}", buf.len());
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:6001").await?;
    let mut i = 0u32;

    // Prepare random price generation
    let mut rng = thread_rng();
    let normal = Normal::new(0.05, 0.22)?;
    let qty_distr = Uniform::from(1..500);
    let bernoulli = Bernoulli::new(0.55).unwrap();

    // This is the evolving price, which is price += N(0.05, (0.22)^2)
    let mut evolving_price = 1021.2f32;

    while i <= MAX_SEQ
    {
        let mut side = Side::Buy;
        let which_side = bernoulli.sample(&mut rand::thread_rng());
        if which_side == true
        {
            side = Side::Sell;
        }

        let v = normal.sample(&mut rng);
        evolving_price += v;

        let _qty = qty_distr.sample(&mut rng);
        let order = Order{id:i, side: side, price: evolving_price, qty: _qty};
        println!("[CLIENT] -> Sending order: {:#?}", order);
        
        // Write the message.
        let mut buffer = BytesMut::new();
        // Encode order
        order.encode(&mut buffer);

        stream.write_all(&buffer).await?;
        i += 1;
    }

    stream.flush();

    Ok(())
}