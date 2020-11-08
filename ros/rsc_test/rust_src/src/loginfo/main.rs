use rosrust_msg::rosgraph_msgs::Log;
use rosrust_msg::std_msgs::Float32;
use rosrust::api::raii as ros;

struct Logger {
    rosout_pub: ros::Publisher<Log>,
    float_pub: ros::Publisher<Float32>,
    val : f32,
    // TODO(lucasw) this is generating a dead code warning
    // warning: field is never read: `float_sub`
    float_sub: ros::Subscriber,  // <Float32>,
}

impl Logger {
    fn loginfo(&mut self, text: &str) {
        let mut msg = Log::default();
        msg.msg = text.to_string();
        self.rosout_pub.send(msg).unwrap();

        let mut float_msg = Float32::default();
        self.val *= -0.998;
        float_msg.data = self.val;
        self.float_pub.send(float_msg).unwrap();
    }

    // fn float_callback(&mut self, msg: &Float32) {
    //    rosrust::ros_info!("preferred callback {}", msg.data);
    // }
}

fn main() {
    rosrust::init("logger");
    // TODO(lucasw) make a default new
    let mut logger = Logger {
        rosout_pub: rosrust::publish("rosout", 10).unwrap(),
        float_pub: rosrust::publish("float_from_rust", 10).unwrap(),
        val: 0.001490,
        float_sub: rosrust::subscribe("float_to_rust", 10,
                                    |msg: Float32| {
            rosrust::ros_info!("not preferred callback {}", msg.data);
        }).unwrap(),
        // float_sub: rosrust::subscribe("float_to_rust", 10,
        //                              Logger::float_callback).unwrap(),
    };
    // logger.float_sub = rosrust::subscribe("float_to_rust", 10, &logger.float_callback).unwrap();

    let rate = rosrust::rate(0.5);
    // print_type_of(&chatter_pub);

    while rosrust::is_ok() {
        logger.loginfo("test manual log");
        rosrust::ros_info!("test ros_info");
        rate.sleep();
    }
}
