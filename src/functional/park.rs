/// Functional tests for thread parking/unparking.
///
/// # Tested Assertions:
/// * thread::park() pauses execution of a thread.
/// * thread::Thread::unpark() unpauses execution of a thread.
#[cfg(all(test))]
mod tests {
    use std::sync::{Arc,Mutex};
    use std::thread;
    use std::time::{Duration,Instant};

    /// Verifies that threads are paused by park_timeout.
    #[test]
    fn thread_park() {
        // The minimum time the test may take to complete.
        const TIMEOUT_THRESHOLD_S : u64 = 4;

        // The time to park the thread.
        const TIMEOUT_S : u64 = 5;

        let start_time = Instant::now();
        let thr = thread::spawn(move || {
            thread::park_timeout(Duration::from_secs(TIMEOUT_S));
        });
        thr.join().unwrap();

        // False positives are possible in this assertion if the test
        // actually took a long time. This is very unlikely.
        //
        // False negatives are possible if park_timeout completes before
        // its timeout is expired (which is possible since park can
        // spuriously return, but also not likely).
        assert!(start_time.elapsed()
            >= Duration::from_secs(TIMEOUT_THRESHOLD_S));
    }

    /// Verifies that threads are re-started by unpark.
    #[test]
    fn thread_unpark() {
        let mtx = Arc::new(Mutex::new(0));
        let outer_thr = thread::current();

        // The maximum time the test may take to complete.
        const TIMEOUT_THRESHOLD_S : u64 = 8;

        // The timer that we assign to the park so the test doesn't hang
        // if the unpark operation fails.
        const TIMEOUT_S : u64 = 10;

        let cloned_mtx = mtx.clone();
        let thr = thread::spawn(move || {
            // If the unpark fails, we will hang on acquiring the
            // lock.
            outer_thr.unpark();
            {
                let mut v = cloned_mtx.lock().unwrap();
                *v = 1;
            }
        });

        // This thread ensures that the test does not take too long.
        // If the run-time exceeds the threshold, there are one of two
        // possibilities:
        //
        // 1) The call to unpark() did not actually start the main
        //    thread again, i.e. unpark() is broken.
        // 2) The test actually took a long time.
        //
        // The second is possible but unlikely.
        let start_time = Instant::now();
        let timer_thr = thread::spawn(move || {
            thr.join().unwrap();
            if start_time.elapsed() >
                    Duration::from_secs(TIMEOUT_THRESHOLD_S) {
                panic!("The thread probably hung.")
            } else {
                0
            }
        });

        {
            let _v = mtx.lock().unwrap();
            thread::park_timeout(Duration::from_secs(TIMEOUT_S));
        }
        timer_thr.join().unwrap();

        {
            let v = mtx.lock().unwrap();
            assert_eq!(*v, 1);
        }
    }
}
