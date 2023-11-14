use structure::txn::Txn;

#[derive(Debug)]
pub struct EventLoop {
    pub tasks: Vec<Txn>,
}

impl EventLoop {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn task_enqueue(&mut self, txn: Txn) {
        self.tasks.push(txn);
    }

    pub fn task_dequeue(&mut self) -> Option<Txn> {
        match self.tasks.first() {
            Some(_) => Some(self.tasks.remove(0)),
            None => None,
        }
    }

    pub fn poll(&mut self) -> Result<(), String> {
        Ok(())
    }
}
