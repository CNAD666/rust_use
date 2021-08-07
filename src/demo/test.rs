use std::thread;

pub fn test() {
    let p = Processor {
        callback: |num| {
            println!("------------------------");
            println!("{}", num);
        },
    };
    p.process_events(12222); // hello world!

    thread::spawn(|| {
        println!("Here's a vector")
    });
}

type Callback = fn(num: i32);

struct Processor {
    callback: Callback,
}

impl Processor {
    fn set_callback(&mut self, c: Callback) {
        self.callback = c;
    }

    fn process_events(&self, value: i32) {
        (self.callback)(value);
    }
}

fn simple_callback() {
    println!("hello world!");
}