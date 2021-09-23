use std::thread;
use std::sync::{Mutex, Arc};

struct Account(i32);

impl Account {
    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }
    }

    fn balance(&self) -> i32 {
        self.0
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(Account(0)));

    {
        let mutex_clone = Arc::clone(&mutex);
        thread::spawn(move || {
            mutex_clone.lock().unwrap().deposit(40);
        }).join().unwrap();
    }
    {
        let mutex_clone = Arc::clone(&mutex);
        thread::spawn(move || {
            mutex_clone.lock().unwrap().withdraw(30);
        }).join().unwrap();
    }
    {
        let mutex_clone = Arc::clone(&mutex);
        thread::spawn(move || {
            mutex_clone.lock().unwrap().deposit(60);
        }).join().unwrap();
    }
    {
        let mutex_clone = Arc::clone(&mutex);
        thread::spawn(move || {
            mutex_clone.lock().unwrap().withdraw(70);
        }).join().unwrap();
    }

    let savings = mutex.lock().unwrap().balance();

    println!("Balance: {:?}", savings);
}
