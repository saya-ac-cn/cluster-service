#[cfg(test)]
mod test {
    use std::sync::{Arc, mpsc, Mutex};
    use std::thread;
    use std::thread::{Builder, current};
    use std::time::Duration;

    #[test]
    fn thread_16_1(){
        thread::spawn(|| {
            for i in 0..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        for i in 0..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_secs(1))
        }
    }

    #[test]
    fn thread_16_2(){
        let handle = thread::spawn(|| {
            for i in 0..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        for i in 0..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_secs(1))
        }
        handle.join().unwrap();
    }

    #[test]
    fn thread_16_5(){
        let v = vec![1,2,3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }

    #[test]
    fn thread_16_8(){
        let (tx,rx) = mpsc::channel();
        thread::spawn(move || {
           let val = String::from("hello");
            tx.send(val).unwrap();
        });
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    #[test]
    fn thread_16_10(){
        let (tx,rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread")
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });
        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn thread_16_15(){
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }

    #[test]
    fn thread_pool_1() {
        let mut v = vec![];
        for id in 0..5 {
            let thread_name = format!("child-{}", id);
            let size: usize = 3 * 1024;
            let builder = Builder::new().name(thread_name).stack_size(size);
            let child = builder.spawn(move || {
                println!("in child:{}", current().name().unwrap());
            }).unwrap();
            v.push(child);
        }
        for child in v {
            child.join().unwrap_or_default();
        }
    }

}