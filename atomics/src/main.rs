use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }
    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                // std::thread::yield_now();
            }
            // std::thread::yield_now();
        }
        // std::thread::yield_now();

        // Safety: hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

use std::thread::spawn;
use std::time::Duration;

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..100)
        .map(|_| {
            spawn(move || {
                for _ in 0..1000 {
                    l.with_lock(|v| {
                        *v += 1;
                    });
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}", l.with_lock(|v| *v));
    assert_eq!(l.with_lock(|v| *v), 100 * 1000);
}

#[test]
#[ignore]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let t1 = spawn(move || {
        std::thread::sleep(Duration::new(0, 500));
        let r1 = y.load(Ordering::SeqCst);
        x.store(r1, Ordering::SeqCst);
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Ordering::SeqCst);
        y.store(42, Ordering::SeqCst);
        r2
    });
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    println!("{}  {}", r1, r2);
}

#[test]
fn ordering_test() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        std::thread::sleep(Duration::new(0, 0));
        x.store(true, Ordering::SeqCst);
    });
    let _ty = spawn(move || {
        std::thread::sleep(Duration::new(0, 600));
        y.store(true, Ordering::SeqCst);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::SeqCst) {}
        if y.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::SeqCst) {}
        if x.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);
    println!("{}", z);
}
