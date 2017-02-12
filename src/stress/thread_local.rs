/// Stress tests for thread_local!.
///
/// Causes significant system load, and may prevent other processes
/// from being created, (i.e. if PIDs are exhausted)
///
/// # Tested Assertions:
/// * thread_local! correctly redirects reads and writes when many
///   threads are using the resource.
///
/// # Test Strategy:
/// 1) Spawn many threads
/// 2) Concurrently access the thread_local resource in each thread,
///    verifying that reads and writes are consistent (i.e. a read
///    following a write returns the written value)
///
#[cfg(all(test))]
mod tests {
    use std::cell::RefCell;
    use std::thread;

    // Number of concurrent threads to run.
    const NUM_THREADS : u32 = 64;

    // Number of write-read iterations.
    const NUM_ITERS : u32 = 1000;

    #[test]
    fn thread_local_consistency() {
        thread_local!(static VAR: RefCell<u32> = RefCell::new(0));
        let mut v = vec![];
        for _i in 0..NUM_THREADS {
            v.push(thread::spawn(move || {
                for j in 0..NUM_ITERS {
                    VAR.with(|value| {
                        assert_eq!(j, *value.borrow());
                        *value.borrow_mut() += 1;
                    });
                }
            }));
        }

        for t in v {
            match t.join() {
                Ok(_status) => { },
                Err(_e) => {
                    panic!("Unexpected thread panic");
                }
            }
        }
    }
}
