/// Functional tests for thread_local!.
///
/// # Tested Assertions:
/// * thread::join returns Ok on success and Error on panic.
#[cfg(all(test))]
mod tests {
    use std::cell::RefCell;
    use std::thread;

    /// Verifies that thread_local! has the same initial value in
    /// every thread.
    #[test]
    fn thread_local_initial_value() {
        thread_local!(static VAR: RefCell<u32> = RefCell::new(42));

        VAR.with(|value| {
            assert_eq!(42, *value.borrow());
        });

        let thr = thread::spawn(move || {
            VAR.with(|value| {
                assert_eq!(42, *value.borrow());
            });
        });

        thr.join().unwrap();
    }

    /// Verifies that thread_local! are unique across threads.
    #[test]
    fn thread_local_not_shared() {
        thread_local!(static VAR: RefCell<u32> = RefCell::new(0));

        VAR.with(|value| {
            *value.borrow_mut() = 1;
            assert_eq!(1, *value.borrow());
        });

        let thr = thread::spawn(move || {
            VAR.with(|value| {
                assert_eq!(0, *value.borrow());
                *value.borrow_mut() = 42;
                assert_eq!(42, *value.borrow());
            });
        });

        thr.join().unwrap();
        VAR.with(|value| {
            assert_eq!(1, *value.borrow());
        });

    }
}
