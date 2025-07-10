
use std::result;
use std::time::Instant;
use std::{thread, time::Duration};
use std::pin::{pin, Pin};
use trpl::{Either, Html,Stream, ReceiverStream, StreamExt};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed"),
        }
    });

    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await
        }

        handle.await.unwrap();
    });

    let fut1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::run(async {
        trpl::join(fut1, fut2).await;
    });

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();
        let async_fut1 = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let async_fut2 = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        let async_fut3 = async {
            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(async_fut1),Box::pin(async_fut2),Box::pin(async_fut3)];
        trpl::join_all(futures).await;
    });

    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    });

    
    let a = async {
        println!("'a' started");
        slow("a", 30);
        trpl::yield_now().await;
        slow("a", 30);
        trpl::yield_now().await;
        slow("a", 30);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'a' finished");
    };

    let b = async {
        println!("'b' started.");
        slow("b",75);
        trpl::yield_now().await;
        slow("b",10);
        trpl::yield_now().await;
        slow("b",15);
        trpl::yield_now().await;
        slow("b",350);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'b' finished.");
    };

    trpl::run(async {
        trpl::race(a,b).await;
    });

    trpl::run(async {

        let one_ns = Duration::from_millis(1);
        let start = Instant::now();
    
        async {
            for _ in 1..100 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;

        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
    });

    let slow = async {
        trpl::sleep(Duration::from_secs(5)).await;
        "I Finished"
    };

    trpl::run (async {
        match timeout(slow,Duration::from_secs(2)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }
    }
    });

    trpl::run(async{
        let values = [1,2,3,4,5,6,7,8,9,10];
        let iter = values.iter().map(|n| n*2);
        let mut stream = trpl::stream_from_iter(iter);

        let mut filtered = 
        stream.filter(|value| value % 3 == 0 || value % 5 == 0);
        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });

    trpl::run(async {
        let messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        let intervals = get_intervals()
        .map( |count| format!("Interval: {count}"))
        .throttle(Duration::from_millis(100))
        .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("problem: {reason:?}"),
            }
        }
    });
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx,rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count){
                eprintln!("Could not send interval {count}: {send_error}");
            };
        }
    });

    ReceiverStream::new(rx)
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx,rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a","b","c","d","e","f","g","h","i","j",];
        for (index,message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 {100} else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }

    });

    ReceiverStream::new(rx)
}

async fn timeout<F: Future> (
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output,Duration> {
    match trpl::race(future_to_try,trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}