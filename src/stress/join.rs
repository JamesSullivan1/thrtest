/// Stress tests for thread::join.
///
/// Causes significant system load, and may prevent other processes
/// from being created, (i.e. if PIDs are exhausted)
///
/// # Tested Assertions:
/// * thread::join consistently captures 'Ok' for threads
///   that exit successfully, even when many threads are pending.
/// * thread::join consistently captures 'Error' for threads
///   that panic, even when many threads are pending.
///
/// # Test Strategy:
/// 1) Spawn threads using thread::Builder::spawn until failure, parking
///    each thread to prevent them from pinning the CPU.
/// 2) Unpause each thread and let them go to completion (or panic)
/// 3) Join each thread and check the expected value

#[cfg(all(test))]
mod tests {
    use std::thread;

    #[test]
    fn thread_join_all_success() {
        let mut v = vec![];
        for ind in 0.. {
            let res = thread::Builder::new()
                    .name(format!("{}", ind))
                    .spawn(move || {
                thread::park();
                0
            });
            
            match res {
                Ok(t) => { v.push(t); },
                Err(_e) => { break }
            }
        }

        for t in v {
            t.thread().unpark();
            match t.join() {
                Ok(_status) => {},
                Err(_e) => {
                    panic!("Unexpected error during join");
                }
            }
        }
    }

    #[test]
    fn thread_join_all_panic() {
        let mut v = vec![];
        for ind in 0.. {
            let res = thread::Builder::new()
                    .name(format!("{}", ind))
                    .spawn(move || {
                thread::park();
                panic!("Alas, poor Yorrick!");
            });
            
            match res {
                Ok(t) => { v.push(t); },
                Err(_e) => { break }
            }
        }

        for t in v {
            t.thread().unpark();
            match t.join() {
                Ok(_status) => {
                    panic!("Unexpected successful thread exit");
                },
                Err(_e) => {}
            }
        }
    }

    #[test]
    fn thread_join_mixed() {
        let mut v = vec![];
        for ind in 0.. {
            let res = thread::Builder::new()
                    .name(format!("{}", ind))
                    .spawn(move || {
                thread::park();
                // Even-indexed threads will return, odd will panic.
                if ind & 1 == 0 {
                    0
                } else {
                    panic!("Alas, poor Yorrick!");
                }
            });
            
            match res {
                Ok(t) => { v.push(t); },
                Err(_e) => { break }
            }
        }

        for (t,ind) in v.into_iter().zip(0..) {
            t.thread().unpark();
            match t.join() {
                Ok(_status) => {
                    assert!(ind & 1 == 0, "Thread {} should have panicked", ind);
                },
                Err(_e) => {
                    assert!(ind & 1 == 1, "Thread {} should have exited", ind);
                },
            }
        }
    }
}
