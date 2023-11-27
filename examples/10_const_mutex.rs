use std::sync::Mutex;
use std::thread;

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = LOGGER.lock().unwrap();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    {
        let logger = LOGGER.lock().unwrap();
        logger.log("some message".to_string());
    }
    {
        let logger2 = LOGGER.lock().unwrap();
        logger2.log("other message".to_string());
    }

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

static LOGGER: Mutex<Logger> = Mutex::new(Logger);

impl Logger {
    fn log(&self, message: String) {
        println!("{}", message)
    }
}
