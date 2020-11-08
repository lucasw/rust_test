use rosrust_msg::rosgraph_msgs::Log;
use rosrust::api::raii as ros;

struct Logger {
    rosout_pub: ros::Publisher<Log>,
}

impl Logger {
    fn loginfo(&self, text: &str) {
        let mut msg = Log::default();
        msg.msg = text.to_string();
        self.rosout_pub.send(msg).unwrap();
    }
}

fn main() {
    rosrust::init("logger");
    // TODO(lucasw) make a default new
    let logger = Logger {
        rosout_pub: rosrust::publish("rosout", 10).unwrap(),
    };

    let rate = rosrust::rate(2.0);
    // print_type_of(&chatter_pub);

    while rosrust::is_ok() {
        logger.loginfo("test manual log");
        rosrust::ros_info!("test ros_info");
        rate.sleep();
    }
}
