pub fn test() {
    //闭包写法
    call_back();
}

fn call_back() {
    let p = Processor {
        callback: |num| {
            println!("------------------------");
            println!("{}", num);
        },
    };
    p.process_events(123456789);
}

type Callback = fn(num: i32);

struct Processor {
    callback: Callback,
}

impl Processor {
    // fn set_callback(&mut self, c: Callback) {
    //     self.callback = c;
    // }

    fn process_events(&self, value: i32) {
        (self.callback)(value);
    }
}