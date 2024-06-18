use std::{sync::{Arc, Mutex}, thread};

fn main() {
   let lock1 = Arc::new(Mutex::new(0));
   let lock2 = Arc::new(Mutex::new(0));
   let lock1_thread1 = lock1.clone();
   let lock2_thread1 = lock2.clone();
   let thread1 = thread::spawn(move || {
            
            let _guard2 = lock2_thread1.lock().unwrap();
            println!("hey thread1 lock1 ");
            thread::sleep(std::time::Duration::from_millis(10000));
            let _guard1 = lock1_thread1.lock().unwrap();
            
   });

   let lock1_thread2 = lock1.clone();
   let lock2_thread2 = lock2.clone();

   let thread2 = thread::spawn(move || {
    let _guard2 = lock2_thread2.lock().unwrap();
    
    println!("hey thread1 lock2 ");
    thread::sleep(std::time::Duration::from_millis(10000));
    let _guard1 = lock1_thread2.lock().unwrap();
   
});

let _ = thread1.join();
let _ = thread2.join();

}
