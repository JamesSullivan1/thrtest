/// Functional tests for thread::Thread::join.
///
/// # Tested Assertions:
/// * thread::join returns Ok on success and Error on panic.
#[cfg(all(test))]
mod tests {
    use std::thread;

    /// Verifies that Thread::join returns Ok(status) on successful
    /// thread exit.
    #[test]
    fn thread_join_success() {
        let thr = thread::spawn(|| {
            42
        });
        let status = thr.join().unwrap();
        assert_eq!(status, 42);
    }

    /// Verifies that Thread::join returns Ok(status) on successful
    /// thread exit.
    #[test]
    fn thread_join_panic() {
        let thr = thread::spawn(|| {
            panic!("Alas, poor Yorrick!");
        });
        match thr.join() {
            Ok(_status) => panic!("Unexpected successful return"),
            Err(_e) => {}
        }
    }
}
