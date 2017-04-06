extern crate crossbeam;

use std::sync::Arc;
use std::thread;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::time::Duration;

use crossbeam::sync::MsQueue;
use crossbeam::sync::SegQueue;

use extra_impls::mpsc_queue::Queue as MpscQueue;

mod extra_impls;

const COUNT: u64 = 10000000;
const THREADS: u64 = 2;

#[cfg(feature = "nightly")]
fn time<F: FnOnce()>(f: F) -> Duration {
    let start = ::std::time::Instant::now();
    f();
    start.elapsed()
}

#[cfg(not(feature = "nightly"))]
fn time<F: FnOnce()>(_f: F) -> Duration {
    Duration::new(0, 0)
}

fn nanos(d: Duration) -> f64 {
    d.as_secs() as f64 * 1000000000f64 + (d.subsec_nanos() as f64)
}

trait Queue<T> {
    fn queue_(&self, T);
    fn dequeue_(&self) -> Option<T>;
}

impl<T> Queue<T> for MsQueue<T> {
    fn queue_(&self, t: T) { self.queue(t) }
    fn dequeue_(&self) -> Option<T> { self.dequeue() }
}

impl<T> Queue<T> for SegQueue<T> {
    fn queue_(&self, t: T) { self.queue(t) }
    fn dequeue_(&self) -> Option<T> { self.dequeue() }
}

impl<T> Queue<T> for MpscQueue<T> {
    fn queue_(&self, t: T) { self.push(t) }
    fn dequeue_(&self) -> Option<T> {
        use extra_impls::mpsc_queue::*;

        loop {
            match self.pop() {
                Data(t) => return Some(t),
                Empty => return None,
                Inconsistent => (),
            }
        }
    }
}

impl<T> Queue<T> for Mutex<VecDeque<T>> {
    fn queue_(&self, t: T) { self.lock().unwrap().push_back(t) }
    fn dequeue_(&self) -> Option<T> { self.lock().unwrap().pop_front() }
}

fn bench_queue_mpsc<Q: 'static + Queue<u64> + Send + Sync>(q: Q) -> f64 {
    let arc = Arc::new(q);
    let d = time(move || {
        let mut v = Vec::new();

        for _i in 0..THREADS {
            let qr = arc.clone();
            v.push(thread::spawn(move || {
                for x in 0..COUNT {
                    let _ = qr.queue_(x);
                }
            }));
        }

        let mut count = 0;
        while count < COUNT*THREADS {
            if arc.dequeue_().is_some() {
                count += 1;
            }
        }

        for i in v {
            i.join().unwrap();
        }
    });

    nanos(d) / ((COUNT * THREADS) as f64)
}

fn bench_queue_mpmc<Q: 'static + Queue<bool> + Send + Sync>(q: Q) -> f64 {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::Relaxed;

    let arc = Arc::new(q);

    let d = time(|| {
        let prod_count = Arc::new(AtomicUsize::new(0));
        let mut v = Vec::new();
        for _i in 0..THREADS {
            let qr = arc.clone();
            let pcr = prod_count.clone();
            v.push(thread::spawn(move || {
                for _x in 0..COUNT {
                    qr.queue_(true);
                }
                if pcr.fetch_add(1, Relaxed) == (THREADS as usize) - 1 {
                    for _x in 0..THREADS {
                        qr.queue_(false)
                    }
                }
            }));

            let qr = arc.clone();
            v.push(thread::spawn(move || {
                loop {
                    if let Some(false) = qr.dequeue_() { break }
                }
            }));
        }

        for i in v {
            i.join().unwrap();
        }
    });

    nanos(d) / ((COUNT * THREADS) as f64)
}

fn bench_chan_mpsc() -> f64 {
    let (tx, rx) = channel();

    let d = time(|| {
        let mut v = Vec::new();
        for _i in 0..THREADS {
            let my_tx = tx.clone();

            v.push(thread::spawn(move || {
                for x in 0..COUNT {
                    let _ = my_tx.send(x);
                }
            }));
        }

        for _i in 0..COUNT*THREADS {
            let _ = rx.recv().unwrap();
        }

        for i in v {
            i.join().unwrap();
        }
    });

    nanos(d) / ((COUNT * THREADS) as f64)
}

fn main() {
    println!("MSQ mpsc: {}", bench_queue_mpsc(MsQueue::new()));
    println!("chan mpsc: {}", bench_chan_mpsc());
    println!("mpsc mpsc: {}", bench_queue_mpsc(MpscQueue::new()));
    println!("Seg mpsc: {}", bench_queue_mpsc(SegQueue::new()));

    println!("MSQ mpmc: {}", bench_queue_mpmc(MsQueue::new()));
    println!("Seg mpmc: {}", bench_queue_mpmc(SegQueue::new()));

//    println!("queue_mpsc: {}", bench_queue_mpsc());
//    println!("queue_mpmc: {}", bench_queue_mpmc());
//   println!("mutex_mpmc: {}", bench_mutex_mpmc());
}
