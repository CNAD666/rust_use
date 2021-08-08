pub fn main() {
    let p = Processor {
        callback: |msg| {
            println!("{}", msg);
        },
    };
    p.process_events("111111111111111");
}

pub fn main_set() {
    let mut p = Processor::default();
    p.set_callback(|msg| {
        println!("{}", msg);
    });
    p.process_events("2222222222222222");
}

type Callback = fn(msg: &str);

struct Processor {
    callback: Callback,
}

impl Processor {
    fn default() -> Self {
        Processor {
            callback: |_| {}
        }
    }

    fn set_callback(&mut self, c: Callback) {
        self.callback = c;
    }

    fn process_events(&self, value: &str) {
        (self.callback)(value);
    }
}