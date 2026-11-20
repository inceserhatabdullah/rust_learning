use std::{pin::Pin, thread, time::Duration, vec};

use trpl::{Either, Html, StreamExt};

fn main() {
    let url = "https://www.rust-lang.org";

    // Future tipinde result
    // Future'ın çalıştırılması için bir executor/runtime tarafından yürütülmesi gerekir. Async Runtime.
    // let result = page_title(url);

    // block_on, verilen Future'ı tamamlanana kadar çalıştırır ve mevcut thread'i bu sırada bloklar.
    trpl::block_on(async {
        match page_title(url).await {
            Some(title) => print!("title: {title}"),
            None => println!("title no found!"),
        }
    });

    // trpl::join, iki isteği eşzamanlı çalıştırmak için kullanılır.

    trpl::block_on(async { trpl::join!(concurrent(url), concurrent(url)) });

    // spawn_task();
    // spawn_task_join();
    // channel_conn();
    // join_all();
    // _yield_now();
    // own_timeout()
    stream();
}

async fn page_title(url: &str) -> Option<String> {
    // Rust compiler, async fonksiyonun hangi noktada durduğunu ve nereden devam etmesi gerektiğini takip eden state machine'i otomatik oluşturur.

    let response = trpl::get(url).await;
    let response_text = response.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

async fn concurrent(url: &str) {
    let response = trpl::get(url).await;
    let text = response.text().await;

    let result = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());

    println!("text = {:?}, url = {}", result.unwrap_or_default(), url);
}

fn spawn_task() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Task A => {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        // Yukarıdaki kodda ilk task 9'a kadar saymak istiyor. Ancak ana async blok yalnızca 4'e kadar sayıyor.
        // Ana blok tamamlandığında program da sonlanır ve spawn edilen task durdurulabilir.
        // Yani ilk task'ın tamamlanması garanti değildir.
        for i in 1..5 {
            println!("Task B => {}", i);
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // Bunu çözmek için spawn_task tarafından döndürülen handle'ı kullanabiliriz:
        handle.await.unwrap();
    });
}

fn spawn_task_join() {
    trpl::block_on(async {
        let future1 = async {
            for i in 1..10 {
                println!("Task join A => {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let future2 = async {
            for i in 1..5 {
                println!("Task join B => {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Bu fonksiyon iki Future alır ve ikisi de tamamlandığında tamamlanan yeni bir Future döndürür.
        trpl::join(future1, future2).await;
    });
}

fn channel_conn() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        //         let tx_future = async {
        //             let messages = vec![
        //                 String::from("hello"),
        //                 String::from("from"),
        //                 String::from("async"),
        //                 String::from("rust"),
        //             ];
        //
        //             for message in messages {
        //                 tx.send(message).unwrap_or_default();
        //                 trpl::sleep(Duration::from_millis(500)).await;
        //             }
        //         };

        let tx_future = async move {
            let messages = vec![
                String::from("hello"),
                String::from("from"),
                String::from("async"),
                String::from("rust"),
            ];

            for message in messages {
                tx.send(message).unwrap_or_default();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_future = async {
            // Fakat alıcı şu kodda beklemeye devam eder. recv() ne zaman None döndürür? -> Channel'ın gönderici tarafı kapandığında. çözüm async move
            while let Some(message) = rx.recv().await {
                println!("received {message}");
            }
        };

        trpl::join(tx_future, rx_future).await;
    });
}

// dinamik sayıda future çalıştırmak
fn join_all() {
    trpl::block_on(async {
        // Rust'ta her async bloğu farklı bir anonim tür üretir. Bu nedenle farklı async blokları doğrudan aynı Vec içerisine koyamayız.
        // let futures = vec![
        //     async {
        //         trpl::sleep(Duration::from_secs(1)).await;
        //         println!("Task A completed.");
        //     },
        //     async {
        //         trpl::sleep(Duration::from_secs(2)).await;
        //         println!("Task B completed.");
        //     },
        //     async {
        //         trpl::sleep(Duration::from_secs(3)).await;
        //         println!("Task C completed.");
        //     },
        // ];

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
            Box::pin(async {
                trpl::sleep(Duration::from_secs(1)).await;
                println!("Task A completed.");
            }),
            Box::pin(async {
                trpl::sleep(Duration::from_secs(2)).await;
                println!("Task B completed.");
            }),
        ];

        // Bu fonksiyon bütün Future'ların tamamlanmasını bekler.
        // Yaklaşık çalışma süresi 2 saniyedir. Çünkü üç Future birlikte ilerler ve en uzun süren Future'ın tamamlanması beklenir.
        trpl::join_all(futures).await;
    });
}

// Async fonksiyonlar yalnızca await noktalarında runtime'a kontrolü bırakır.
// Ancak bir async blok içerisinde uzun süre await kullanmadan çalışırsak ne olur?
// Diğer Future'ların ilerlemesini engelleyebiliriz.
// Buna Starvation (aç kalma) denir.
// yield_now() — Kontrolü runtime'a bırakmak
// yield_now() paralellik sağlamaz. Yalnızca scheduler'a başka task'ları çalıştırma fırsatı verir.

fn _yield_now() {
    trpl::block_on(async {
        let _yield = async {
            println!("A started.");

            slow("A", 30);
            trpl::yield_now().await;

            slow("A", 10);
            trpl::yield_now().await;

            slow("A", 20);
            trpl::yield_now().await;

            println!("A tamamlandı");
        };

        let _yieldb = async {
            println!("B started.");

            slow("B", 30);
            trpl::yield_now().await;

            slow("B", 10);
            trpl::yield_now().await;

            slow("B", 20);
            trpl::yield_now().await;

            println!("B tamamlandı");
        };

        // trpl::select() ile iki Future'ı yarışa sokuyoruz.
        // select() iki Future'dan hangisi önce tamamlanırsa onun sonucunu döndürür.

        trpl::select(_yield, _yieldb).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms");
}

fn own_timeout() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(1)).await;
            "Completed"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("{message}"),
            Err(duration) => {
                eprintln!("timeout: {} secs", duration.as_secs());
            }
        }
    });
}

// Fonksiyonumuz herhangi bir Future türü kabul edebilir.
// Yani yalnızca HTTP isteği için değil, farklı async işlemler için de kullanılabilir.
// Ok(F::Output): Future başarıyla tamamlandı ve kendi sonucunu döndürdü. string ise string, integer ise integer. generic.
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        // işlem önce tamamlanırsa sonucu Ok olarak döner
        // Timer önce tamamlanır timeout oluştuğu bildirilir
        // Either iki farklı olasılığı temsil eden enum
        Either::Left(output) => Ok(output),

        // Bir diğer önemli nokta: Timeout gerçekleştiğinde kaybeden Future artık seçimin içerisinde yürütülmez ve drop edilir.
        // Dolayısıyla timeout, işlemin sonucunu beklemeyi bırakır; fakat dış sistemde başlamış bir işlemin kesin olarak geri alındığını garanti etmez.
        // Örneğin karşı sunucuya gönderilmiş bir HTTP isteği timeout sonrasında da sunucu tarafında işleniyor olabilir.
        Either::Right(_) => Err(max_time),
    }
}

fn stream() {
    trpl::block_on(async {
        let values = [1, 2, 3];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("Value: {value}");
        }
    });

    trpl::block_on(async {
        let values = 1..=20;
        let stream = trpl::stream_from_iter(values);
        let mut filtered = stream.filter(|val| *val % 3 == 0 || *val % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("Value => {value}");
        }
    });
}
