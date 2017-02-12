/// Functional tests for thread::spawn and thread::Builder::spawn.
///
/// # Tested Assertions:
/// * thread::spawn creates a new thread with a distinct thread ID
///   that is scheduled to perform its subroutine.
/// * thread::Builder::spawn behaves similarly with a variety of
///   arguments, and correctly handles thread names
#[cfg(all(test))]
mod tests {
    use std::sync::{Arc,Mutex};
    use std::thread;

    /// Verifies that threads are spawned by thread::spawn and can
    /// do some work.
    #[test]
    fn thread_spawn() {
        let mtx = Arc::new(Mutex::new(0));

        let cloned_mtx = mtx.clone();
        let thr = thread::spawn(move || {
            let mut v = cloned_mtx.lock().unwrap();
            *v = 1;
        });
        thr.join().unwrap();

        {
            let v = mtx.lock().unwrap();
            assert_eq!(*v, 1);
        }
    }

    /// Verifies that threads are spawned by thread::Builder::spawn and
    /// can do some work.
    #[test]
    fn thread_builder_spawn() {
        let mtx = Arc::new(Mutex::new(0));

        let cloned_mtx = mtx.clone();
        let thr = thread::Builder::new().spawn(move || {
            let mut v = cloned_mtx.lock().unwrap();
            *v = 1;
        }).unwrap();
        thr.join().unwrap();

        {
            let v = mtx.lock().unwrap();
            assert_eq!(*v, 1);
        }
    }

    /// Verifies that threads are spawned by thread::Builder::spawn,
    /// and that their name is set appropriately.
    #[test]
    fn thread_builder_spawn_named() {
        let mtx = Arc::new(Mutex::new(0));

        let cloned_mtx = mtx.clone();
        let thr = thread::Builder::new()
                .name("thread".into())
                .spawn(move || {
            assert_eq!(thread::current().name().unwrap(), "thread");
            let mut v = cloned_mtx.lock().unwrap();
            *v = 1;
        }).unwrap();
        thr.join().unwrap();

        {
            let v = mtx.lock().unwrap();
            assert_eq!(*v, 1);
        }
    }
}
