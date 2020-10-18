struct Talker {
    // TODO(lucasw) is it possible to simplify this?
    chatter_pub: rosrust::api::raii::Publisher<rosrust_msg::std_msgs::String>,
    count: i32,
}

impl Talker {
    fn update(&mut self) {
        let mut msg = rosrust_msg::std_msgs::String::default();
        msg.data = format!("hello world {}", self.count);
        self.chatter_pub.send(msg).unwrap();
        self.count += 1;
    }
}

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

fn main() {
    println!("test");
    rosrust::init("talker");
    let mut talker = Talker {
        chatter_pub: rosrust::publish("chatter", 10).unwrap(),
        count: 0,
    };
    let rate = rosrust::rate(2.0);
    // print_type_of(&chatter_pub);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        talker.update();
        rate.sleep();
    }
}
