use std::sync::mpsc;
pub struct Multiplex {
    channels: Vec<mpsc::Sender<bool>>
}

impl Multiplex {
    pub fn new() -> Self {
        Multiplex {
            channels: Vec::new()
        }
    }

    pub fn subscribe(&mut self) -> mpsc::Receiver<bool> {
        let (tx, rx) = mpsc::channel();
        self.channels.push(tx);
        rx
    }

    pub fn publish(&mut self, msg: bool) {
        for channel in self.channels.iter() {
            let res = channel.send(msg).unwrap();
            println!("publish result: {:?}", res);
        }
    }


    
}