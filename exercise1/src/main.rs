use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;


fn prision_problem(n: usize) {
  // set up tow channel 
  // 1 for light communication 
  // 2 for waiting to confirm all 
  let (tx, rx) = mpsc::channel();
  let (light_tx, light_rx) = mpsc::channel();
  // a thread safe counter which would count till N - 1
  let count = Arc::new(Mutex::new(0));
  // start a orchestrator thread which would monitor till get N-1 light On communication 
  let orchestrator_tx = tx.clone();
  thread:: spawn(move || {
    while *count.lock().unwrap() < n - 1 {
      if let Ok(light_on) = light_rx.recv() {
        if light_on {
          *count.lock().unwrap() += 1;
        }
      }
    }
    orchestrator_tx.send(()).unwrap();
  });

  let mut rng = rand::thread_rng();
  let order_set: Vec<usize>  = (0..n-1).map(|_| rng.gen_range(0..n-1)).collect();
  // orderset number represent the prisoner who wouldturn on light
    for &i in &order_set {
      let light_tx = light_tx.clone();
      let prisoner_tx = tx.clone();
      thread::spawn(move || {
        light_tx.send(true).unwrap();
        println!("Prisoner {} turned on the light.", i + 1);
        sleep(Duration::from_millis(100));
        prisoner_tx.send(()).unwrap();

      });
    }

    // wait for all prisoner to be released
    for _ in 0..n {
      rx.recv().unwrap();
    }

    println!("All prisoners have been released!");
}

fn main() {
    prision_problem(100);
}
