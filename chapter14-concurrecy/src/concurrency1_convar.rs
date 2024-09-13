mod concurrency2_future;

use parking_lot::{Condvar, Mutex};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// 我们启动三个线程，t1，t2，t3。分别执行任务T1，T2，T3。现在要求：T2必须等待T1和T3完成之后再执行
pub fn main() {
    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let pair2 = pair.clone();
    let pair3 = pair.clone();

    let t1 = thread::Builder::new()
        .name("T1".to_string())
        .spawn(move || {
            sleep(Duration::from_secs(4));
            println!("I'm working in T1, step 1");
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock();
            *started += 2;
            cvar.notify_one();
        })
        .unwrap();

    let t2 = thread::Builder::new()
        .name("T2".to_string())
        .spawn(move || {
            println!("I'm working in T2, start");
            let &(ref lock, ref cvar) = &*pair;
            let mut notify = lock.lock();

            while *notify < 5 {
                cvar.wait(&mut notify);
            }
            println!("I'm working in T2, final");
        })
        .unwrap();

    let t3 = thread::Builder::new()
        .name("T3".to_string())
        .spawn(move || {
            sleep(Duration::from_secs(3));
            println!("I'm working in T3, step 2");
            let &(ref lock, ref cvar) = &*pair3;
            let mut started = lock.lock();
            *started += 3;
            cvar.notify_one();
        })
        .unwrap();

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
