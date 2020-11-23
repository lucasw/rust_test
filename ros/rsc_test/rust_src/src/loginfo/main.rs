use rosrust_msg::rosgraph_msgs::Log;
use rosrust_msg::std_msgs::Float32;
use rosrust::api::raii as ros;
use std::sync::Mutex;

#[allow(dead_code)]
struct TestNode {
    rosout_pub: ros::Publisher<Log>,
    float_pub: ros::Publisher<Float32>,
    val : f32,
    // TODO(lucasw) this is generating a dead code warning
    // warning: field is never read: `float_sub`
    // val2 : f32,
    val2_arc2 : std::sync::Arc<Mutex<f32>>,
    float_sub: ros::Subscriber,  // <Float32>,
}


impl TestNode {
    /*
    // fn float_callback(msg: &Float32) {
    fn float_callback(&mut self, msg: &Float32) {
        rosrust::ros_info!("preferred callback {}, val {}", msg.data, self.val);
    }
    */

    fn new() -> Self {
        // Default::default()
        //            rosout_pub: rosrust::publish("rosout", 10).unwrap(),
        let rosout_pub = rosrust::publish("rosout", 10).unwrap();
        let float_pub = rosrust::publish("float_from_rust", 10).unwrap();
        let val = 0.001490;

        let val2 = Mutex::new(0.0);
        let val2_arc = std::sync::Arc::new(val2);
        let val2_arc2 = val2_arc.clone();

        let float_callback = {
            // TODO(lucasw) how to use val or another struct member variable here?
            move |msg: Float32| {
                // rosrust::ros_info!("preferred callback {} {}", msg.data, self.val);
                let mut val2_guard = val2_arc.lock().unwrap();
                *val2_guard += msg.data;
                rosrust::ros_info!("preferred callback {} {}", msg.data, *val2_guard);
            }
        };

        let float_sub = rosrust::subscribe("float_to_rust", 10, float_callback).unwrap();

        // let float_sub = rosrust::subscribe("float_to_rust", 10, Self::float_callback).unwrap();

        // float_sub: rosrust::subscribe("float_to_rust", 10,
        //                              TestNode::float_callback).unwrap(),
        // test_node.float_sub = rosrust::subscribe("float_to_rust", 10, &test_node.float_callback).unwrap();

        let test_node = Self {
            rosout_pub,
            float_pub,
            val,
            val2_arc2,
            float_sub,
        };

        test_node
    }

    fn update(&mut self, text: &str) {
        let mut msg = Log::default();
        msg.msg = text.to_string();
        self.rosout_pub.send(msg).unwrap();

        let mut float_msg = Float32::default();
        self.val *= -0.998;
        float_msg.data = self.val;
        self.float_pub.send(float_msg).unwrap();

        let val2_guard = self.val2_arc2.lock().unwrap();
        rosrust::ros_info!("update {}", *val2_guard);
    }
}

fn main() {
    rosrust::init("log_test");
    // TODO(lucasw) make a default new
    let mut test_node = TestNode::new();
    let rate = rosrust::rate(2.0);
    // print_type_of(&chatter_pub);

    while rosrust::is_ok() {
        test_node.update("test manual log");
        rosrust::ros_info!("test ros_info");
        rate.sleep();
    }
}
