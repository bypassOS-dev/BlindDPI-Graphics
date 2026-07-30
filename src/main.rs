use futures::{executor::block_on, join};
use std::time::Duration;
use futures_timer::Delay;

async fn brew_tea() {
    println!("Чай начал завариваться...");
    Delay::new(Duration::from_millis(3000)).await;
    println!("Чай готов!");
}

async fn toast_bread() {
    println!("Тост начал жариться...");
    Delay::new(Duration::from_millis(2000)).await;
    println!("Тост поджарен!");
}

async fn butter_toast(a: impl Future<Output = ()>, b: impl Future<Output = ()>) {
    join!(a, b);
    println!("Готово!");
}
async fn make_toast_with_butter() {
    toast_bread().await;
    println!("Тост намазан маслом!");
}

fn main() {
    let f1 = brew_tea();
    let f2 = make_toast_with_butter();
    let f3 = butter_toast(f1, f2);

    block_on(f3);
}
