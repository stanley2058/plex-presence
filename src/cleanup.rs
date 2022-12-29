pub mod cleanup {
    use crate::Config;
    use ctrlc;
    use std::{fs, process::exit};

    pub struct Cleanup;

    fn delete_lock_and_exit() {
        let lockfile = Config::get_lockfile();
        let _ = fs::remove_file(lockfile);
        exit(0);
    }

    impl Cleanup {
        pub fn new() -> Self {
            let _ = ctrlc::set_handler(move || {
                delete_lock_and_exit();
            });
            Cleanup {}
        }
    }

    impl Drop for Cleanup {
        fn drop(&mut self) {
            delete_lock_and_exit();
        }
    }
}
