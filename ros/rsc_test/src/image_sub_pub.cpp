// Subscribe to an image topic and publish a modified version of the image in response.
// For now modify the image directly in the callback and publish that output.
//

#include <algorithm>
#include <ros/ros.h>
#include <sensor_msgs/Image.h>

class ImageSubPub
{
public:
  ImageSubPub()
  {
    image_pub_ = nh_.advertise<sensor_msgs::Image>("image_out", 4);
    image_sub_ = nh_.subscribe("image_in", 4, &ImageSubPub::imageCallback, this);
  }
private:

  void imageCallback(sensor_msgs::Image msg)
  {
    ROS_INFO_STREAM("image callback " << msg.width << " " << msg.height << " " << msg.data.size() <<
                    " " << msg.encoding << " " << fr);
    for (size_t i = 0; i < msg.data.size(); ++i) {
      msg.data[i] = std::clamp(msg.data[i] * fr, 0.0f, 255.0f);
    }
    image_pub_.publish(msg);
  }

  ros::NodeHandle nh_;
  ros::Subscriber image_sub_;
  ros::Publisher image_pub_;
  float fr = 8.0;
};

int main(int argc, char** argv)
{
  ros::init(argc, argv, "image_sub_pub");
  ImageSubPub image_sub_pub;
  ros::spin();
}
