#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn a_channel() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("im"),
                String::from("another"),
                String::from("generator"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1500));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }

        println!("main thread ends");
    }
}
