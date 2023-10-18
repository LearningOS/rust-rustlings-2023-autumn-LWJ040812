use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
    tx: Arc<Mutex<mpsc::Sender<u32>>>,
}

impl Queue {
    fn new(tx: Arc<Mutex<mpsc::Sender<u32>>>) -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
            tx,
        }
    }

    fn send_values(&self) {
        let tx = Arc::clone(&self.tx);
        let first_half = self.first_half.clone();
        thread::spawn(move || {
            for val in first_half {
                println!("sending {:?}", val);
                tx.lock().unwrap().send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        let tx = Arc::clone(&self.tx);
        let second_half = self.second_half.clone();
        thread::spawn(move || {
            for val in second_half {
                println!("sending {:?}", val);
                tx.lock().unwrap().send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let shared_tx = Arc::new(Mutex::new(tx));

    let queue = Queue::new(Arc::clone(&shared_tx));
    queue.send_values();

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue.length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue.length);
}
