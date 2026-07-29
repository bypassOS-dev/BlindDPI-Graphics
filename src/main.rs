use futures::executor::block_on;
use std::time::Duration;
use futures_timer::Delay;

async fn one() {
    println!("One start");
    Delay::new(Duration::from_millis(100)).await;
    println!("One end");
}
async fn two() {
    println!("Two start");
    Delay::new(Duration::from_millis(100)).await;
    println!("Two end");
}
async fn async_main() {
    let one = one();
    let two = two();
    futures::join!(one, two);
}
fn main() {
    let q = async_main();
    block_on(q);
}
