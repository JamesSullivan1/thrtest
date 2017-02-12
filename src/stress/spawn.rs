/// Stress tests for thread::spawn and thread::Builder::spawn.
///
/// Causes significant system load, and may prevent other processes
/// from being created, (i.e. if PIDs are exhausted)
///
/// # Tested Assertions:
/// * If Builder::spawn fails because of a lack of resources, an error
///   from the OS is returned (e.g EAGAIN).
///
/// # Test Strategy:
/// 1) Spawn threads until failure, parking each thread to prevent
///    them from pinning the CPU.
#[cfg(all(test))]
mod tests {
    use std::thread;

    #[test]
    fn thread_builder_spawn_raises_error() {
        let mut v = vec![];
        loop {
            let res = thread::Builder::new().spawn(move || {
                thread::park();
            });

            match res {
                Ok(t) => {
                    v.push(t);
                },
                Err(e) => {
                    println!("Error {}", e);
                    break;
                }
            }
        }

        for thr in v.into_iter() {
            thr.thread().unpark();
            match thr.join() {
                Ok(_status) => { },
                Err(_e) => {
                    panic!("Unexpected thread panic");
                }
            }
        }
    }
}
