use std::time::Duration;

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = vec![];
    for i in 0..10 {
        handles.push(runtime.spawn(do_something(i)));
    }
    println!("PS: new_current_thread will not execute until block_on is called");

    for handle in handles {
        runtime.block_on(handle).unwrap();
    }
}

async fn do_something(i: u32) {
    println!("start do_something, {i}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("end do_something, {i}");
}

