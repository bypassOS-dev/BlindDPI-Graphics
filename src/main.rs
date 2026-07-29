use futures::executor::block_on;

async fn hello() {
    println!("Hello");
}
fn main() {
    println!("Start");
    let future = hello();
    block_on(future);
    println!("End");
}
