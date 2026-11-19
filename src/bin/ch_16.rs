use std::sync::Arc;
use std::thread;
use std::time::Duration;
// multiple producer, single consumer
use std::sync::Mutex;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // joinhandle thread çalışmasını bitmesini bekler
    // join çağırısı konumu önemli, for öncesine koyarsan thread çalışmasını bitmesini bekler, sonrasında main thread çalışır.
    handle.join().unwrap_or_default();

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        // move kullanmanın nedeni sender değişkenin ownership'ini spawned thread'e aktarmak.
        let message = String::from("hello from spawned thread");

        sender.send(message).unwrap();
        // send(message) değişkenin ownership'ini aktarır. thread artık bu string değeri kullanamaz.
        //println!("{}", message);
    });

    // mesaj yok ise yeni mesaj gelene veya bütün göndericiler kapatılana kadar bekler.
    let received = receiver.recv().unwrap();

    println!("received: {}", received);

    let (snd, rcv) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![String::from("message 1"), String::from("message 2")];

        for message in messages {
            snd.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rcv {
        println!("received message {received}");
    }

    let (tx, rx) = mpsc::channel();

    // iki farklı thread aynı channel'a mesaj gönderebilir.  sender kopyası oluşturduk
    let tx1 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("message 1, thread 1"),
            String::from("message 2, thread 1"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("message 1, thread 2"),
            String::from("message 2, thread 2"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("received message {received}");
    }

    // try_recv() mesaj yoksa beklemez. Hemen sonuç döndürür.
    // match rx.try_recv() {
    //     Ok(message) => println!("Received: {message}"),
    //     Err(_) => println!("No message available"),
    // }

    mutext_thread();
}

fn vec_thread() {
    let vector = vec![1, 23, 5];

    // spawn thread ne kadar çalışacağını compiler bilemez.
    // move closure kullanmak vector değişkeninin ownership'ini aktarır
    // spawn thread artık vector değişkenin sahibi
    // move ownership kurallarını ortadan kaldırmaz, clouse değişkenlerin nasıl yakalayacağını belirler
    let handle = thread::spawn(move || {
        println!("{:?}", vector);
    });

    handle.join().unwrap();

    //let message = String::from("selam");

    // let handle = thread::spawn(move || {
    //     println!("{}", message);
    // });

    //  value borrowed here after move (rustc E0382). Main thread içerisinde tkerar message kullanılmaz.
    // println!("{}", message)
}

fn mutext_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 10 thread ile counter değer artırma. Arc kullanılmalı
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

    println!("result {:?}", *counter.lock().unwrap());

    share_reference_between_threads();
}

struct Task {
    id: u8,
    progress: Arc<Mutex<u8>>,
}

fn required_send<T: Send>(_: &T) {}
fn required_sync<T: Sync>(_: &T) {}

fn share_reference_between_threads() {
    let task = Task {
        id: 1,
        progress: Arc::new(Mutex::new(0)),
    };

    required_send(&task);
    required_sync(&task);

    let shared_progress = Arc::clone(&task.progress);

    let handle = thread::spawn(move || {
        let mut p = shared_progress.lock().unwrap();
        *p += 5;
    });

    handle.join().unwrap();

    println!(
        "Task {} progress: {}",
        task.id,
        *task.progress.lock().unwrap()
    );
}
