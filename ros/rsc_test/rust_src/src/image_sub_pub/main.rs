// Subscribe to an image topic and publish a modified version of the image in response.
// For now modify the image directly in the callback and publish that output,
// but later maybe store the image and then modify it in a separate thread.
//

use rosrust_msg::sensor_msgs::Image;
use rosrust::api::raii as ros;
// use std::sync::Mutex;

struct ImageSubPub {
    _image_sub: ros::Subscriber,  // <Image>
    // image_pub: ros::Publisher<Image>,
    // TODO(lucasw) add a variable float to scale the incoming image with
    // fr : f32,
}


impl ImageSubPub {
    fn new() -> Self{
        rosrust::ros_info!("new ImageSubPub");
        let image_pub = rosrust::publish("image_out", 4).unwrap();
        let fr = 8.0;

        let image_callback = {
            move |mut msg: Image| {
                rosrust::ros_info!("image callback {} {} {} {} {}",
                                   msg.width, msg.height, msg.data.len(), msg.encoding,
                                   fr);
                for pixel in msg.data.iter_mut() {
                    *pixel = (*pixel as f32 * fr) as u8;
                }
                image_pub.send(msg).unwrap();
            }
        };

        let _image_sub = rosrust::subscribe("image_in", 4, image_callback).unwrap();

        let image_sub_pub = Self {
            _image_sub,
            // image_pub,
            // fr,
        };

        image_sub_pub
    }
}

fn main() {
    rosrust::init("image_sub_pub_rs");
    let _image_sub_pub = ImageSubPub::new();
    rosrust::spin();
}
