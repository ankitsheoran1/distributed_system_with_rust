mod publish;

use publish::Multiplex;
use std::thread;
fn main() {
    let mut multi = Multiplex::new();
    let recv1 = multi.subscribe();
    let recv2 = multi.subscribe();

    let rc_th_1 = thread::spawn(move || {
        loop {
            let msg = recv1.recv();
            match msg {
                Ok(msg) => println!("msg in rec1 {}", msg),
                Err(e) => {
                    println!("error in rec1 {}", e);
                    break;
                }

            }
        }
    });

    let rc_th_2 = thread::spawn(move || {
        loop {
            let msg = recv2.recv();
            match msg {
                Ok(msg) => println!("msg in rec2 {}", msg),
                Err(e) => {
                    println!("error in rec2 {}", e);
                    break;
                }

            }
        }
    });

    let publisher = thread::spawn(move || {
        for i in 0..10 {
            multi.publish(i % 2 == 0);
        }
    });

    rc_th_1.join().unwrap();
    rc_th_2.join().unwrap();
    publisher.join().unwrap();
}
