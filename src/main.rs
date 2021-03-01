use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

static LOOPS: usize = 10_000;
static BASE: u32 = 10_000_000;

fn collatz(n: u32) -> Option<u32> {
    if n == 1 {
        None
    } else if n % 2 == 0 {
        Some(n / 2)
    } else {
        Some(3 * n + 1)
    }
}

fn baseline() {
    let mut cur = BASE;
    loop {
        match collatz(cur) {
            None => return (),
            Some(next) => cur = next,
        }
    }
}

fn mutex_single_reader_single_writer() {
    let num = Arc::new(Mutex::new(10_000_000));
    let reader_num = num.clone();

    let writer_hndl = thread::spawn(move || loop {
        let mut lock = num.lock().unwrap();
        match collatz(*lock) {
            None => return (),
            Some(next) => *lock = next,
        }
    });
    let reader_hndl = thread::spawn(move || loop {
        let lock = reader_num.lock().unwrap();
        if collatz(*lock).is_none() {
            break;
        }
    });

    writer_hndl.join().unwrap();
    reader_hndl.join().unwrap();
}

fn mutex_100_reader_single_writer() {
    let num = Arc::new(Mutex::new(10_000_000));
    let writer_num = num.clone();

    let writer_hndl = thread::spawn(move || loop {
        let mut lock = writer_num.lock().unwrap();
        match collatz(*lock) {
            None => return (),
            Some(next) => *lock = next,
        }
    });

    let mut reader_hndls = Vec::with_capacity(100);
    for _ in 0..100 {
        let reader_num = num.clone();
        let reader_hndl = thread::spawn(move || loop {
            let lock = reader_num.lock().unwrap();
            if collatz(*lock).is_none() {
                break;
            }
        });
        reader_hndls.push(reader_hndl);
    }

    writer_hndl.join().unwrap();
    for hndl in reader_hndls {
        hndl.join().unwrap();
    }
}

fn rwlock_100_reader_single_writer() {
    let num = Arc::new(RwLock::new(10_000_000));
    let writer_num = num.clone();

    let writer_hndl = thread::spawn(move || loop {
        let mut lock = writer_num.write().unwrap();
        match collatz(*lock) {
            None => return (),
            Some(next) => *lock = next,
        }
    });

    let mut reader_hndls = Vec::with_capacity(100);
    for _ in 0..100 {
        let reader_num = num.clone();
        let reader_hndl = thread::spawn(move || loop {
            let lock = reader_num.read().unwrap();
            if collatz(*lock).is_none() {
                break;
            }
        });
        reader_hndls.push(reader_hndl);
    }

    writer_hndl.join().unwrap();
    for hndl in reader_hndls {
        hndl.join().unwrap();
    }
}

fn bench<F>(name: &str, func: F)
where
    F: Fn() -> (),
{
    let mut times = Vec::with_capacity(LOOPS);
    for _ in 0..LOOPS {
        let now = Instant::now();
        func();
        times.push(now.elapsed());
    }
    times.sort();
    println!(
        "{} || high: {:?}, low: {:?}, mid: {:?}",
        name,
        times[LOOPS - 1],
        times[0],
        times[LOOPS / 2]
    );
}

fn main() {
    bench("BASELINE", baseline);
    bench("MUTEX SINGLE R/W", mutex_single_reader_single_writer);
    bench("MUTEX 100 R / SINGLE W", mutex_100_reader_single_writer);
    bench("RWLOCK 100 R / SINGLE W", rwlock_100_reader_single_writer);
}
