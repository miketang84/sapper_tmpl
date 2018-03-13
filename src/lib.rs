#[macro_use]
extern crate lazy_static;
extern crate tera;
#[cfg(feature = "monitor")]
extern crate notify;

use tera::Tera;
use std::sync::RwLock;
pub use tera::Context;

lazy_static! {
    static ref TERA: RwLock<Tera> = RwLock::new(Tera::new("views/**/*").unwrap());
}

pub fn render(path: &str, context: Context) -> String {
    #[cfg(feature = "monitor")]
    monitor();

    TERA.read().unwrap().render(path, &context).unwrap_or_else(|e| {
        println!("rendering error: {:?}", e);
        // warn!("rendering error: {:?}", e);
        "rendering error".to_owned()
    })
}

#[cfg(feature = "monitor")]
fn monitor() {
    use std::sync::{Once, ONCE_INIT};
    use notify::{Watcher, RecursiveMode, watcher};
    use std::sync::mpsc::channel;
    use std::thread::spawn;
    use std::time::Duration;

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        spawn(move || {
            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();
            watcher.watch("./views", RecursiveMode::Recursive).unwrap();

            loop {
                // println!("start monitor");
                match rx.recv() {
                    Ok(_) => {
                            let _ = TERA.write().unwrap().full_reload();
                            println!("views change");
                        },
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });
    });
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
