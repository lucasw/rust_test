#include <iostream>
#include <ros/ros.h>
#include <std_msgs/Float32.h>

class TestPubSub
{
public:
  TestPubSub()
  {
    pub_ = nh_.advertise<std_msgs::Float32>("float_to_rust", 2);
    sub_ = nh_.subscribe("float_from_rust", 2, &TestPubSub::callback, this);
    timer_ = nh_.createTimer(ros::Duration(2.0), &TestPubSub::update, this);
  }

  void callback(const std_msgs::Float32ConstPtr& msg)
  {
    ROS_INFO_STREAM("float from rust " << msg->data);
  }

  void update(const ros::TimerEvent& event)
  {
    std_msgs::Float32 msg;
    val *= 1.0045243;
    msg.data = val;
    ROS_INFO_STREAM("float to rust " << msg.data);
    pub_.publish(msg);
  }

private:
  ros::NodeHandle nh_;
  ros::Publisher pub_;
  ros::Subscriber sub_;
  ros::Timer timer_;
  float val = 0.134;
};

int main(int argc, char** argv)
{
  ros::init(argc, argv, "test_pub_sub");
  TestPubSub test_pub_sub;
  ros::spin();
}
