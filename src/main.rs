use futures::executor::block_on;

async fn one() {
    println!("One start");
    println!("One end");
}
async fn two() {
    println!("Two start");
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
