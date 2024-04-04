# zikoichi_controller

# 注意
/serial_zikoichiはマイコン側のコードです。

# Node
## manual
### Subscriber
|topic name|type|purpose|
|:--:|:--:|:--:|
|/cmd_vel|geometry_msgs/msg/Twist|subscribe Vector command|

### Parameters
|name|default|purpose|
|:--:|:--:|:--:|
|port_name|"/dev/ttyACM0"|set serialport name|
|power_rate|1.0|set rate(0.1~1.0)|

## auto
### Subscriber
|topic name|type|purpose|
|:--:|:--:|:--:|
|/turtle1/odom|nav_msgs/msg/Odometry|subscribe odometry from turtle_odometer|
|/euler|geometry_msgs/msg/Vector3|subscribe euler angle|
|/imu/odom|nav_msgs/msg/Odometry|subscribe odometry from imu_localizer|

### Parameters
|name|default|purpose|
|:--:|:--:|:--:|
|port_name|"/dev/ttyACM0"|set serialport name|
