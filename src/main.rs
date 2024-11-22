use mini_async_runtime::{executor::CountExecutor, future::CountFuture};

fn main() {
    use futures::future::join_all;
    let mut ex = CountExecutor::<()>::new();

    ex.run(async{
        CountFuture::new(5).await;
        CountFuture::new(10).await;
        let futures = vec![
            CountFuture::new(100),
            CountFuture::new(4),
            CountFuture::new(3),
            CountFuture::new(2),
            CountFuture::new(1),
        ];
        let rets = join_all(futures).await;
        println!("All futures are ready: {:?}", rets);
        // retsを全て足し合わせる
        let sum = rets.iter().fold(0, |acc, x| acc + x);
        println!("Sum of all futures: {}", sum);
    });
}
