use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();

    // clone tx1 before it is moved to the 1st thread's lambda.
    let tx2 = mpsc::Sender::clone(&tx1);

    let th1 = thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
        // println!("{}", val);
        //                ^^^ value borrowed here after move

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for v in vals {
            println!("🧵 Sending: {}", v);
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let th2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(400));
        }
    });

    println!("🅼 before receiving");
    loop {
        match rx.try_recv() {
            Ok(received) => {
                println!("🅼 Got: {}", received);
            },
            Err(TryRecvError::Empty) => {
                println!("🅼 Nothing to read");
            },
            Err(TryRecvError::Disconnected) => {
                println!("🅼 Disconnected");
                break;
            },
        }
        thread::sleep(Duration::from_millis(100));
    }

    th1.join().unwrap();
    th2.join().unwrap();
}
