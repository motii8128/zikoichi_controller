use safe_drive::{
    context::Context, 
    error::DynError, 
    logger::Logger, 
    msg::common_interfaces::{nav_msgs, geometry_msgs}, 
    pr_info, 
};

use async_std;
use serialport;
use zikoichi_controller::Wheel;
use ros2_rust_util::get_str_parameter;

#[async_std::main]
async fn main()->Result<(), DynError>
{
    let ctx = Context::new()?;
    let node = ctx.create_node("ZikoichiAuto", None, Default::default())?;
    let mut turtle_sub = node.create_subscriber::<nav_msgs::msg::Odometry>("/turtle1/odom", None)?;
    let mut euler_sub = node.create_subscriber::<geometry_msgs::msg::Vector3>("/euler", None)?;
    let mut i_odom_sub = node.create_subscriber::<nav_msgs::msg::Odometry>("/imu/odom", None)?;
    let log = Logger::new(node.get_name());

    let port_name = get_str_parameter(node.get_name(), "port_name", "/dev/ttyACM0");

    let mut port = serialport::new(port_name, 115200).timeout(std::time::Duration::from_millis(100)).open().unwrap();


    let diagonal = ((2.0_f32).sqrt() / 2.0) as f64;

    pr_info!(log, "Start {}", node.get_name());

    loop {
        let t_odom = turtle_sub.recv().await.unwrap();
        let imu_euler = euler_sub.recv().await.unwrap();
        let imu_odom = i_odom_sub.recv().await.unwrap();

        let imu_yaw = imu_euler.z;
        let t_yaw = t_odom.pose.pose.position.z;

        pr_info!(log, "imu:{}, tur:{}", imu_yaw, t_yaw);

        let x_vec = t_odom.pose.pose.position.x - imu_odom.pose.pose.position.x;
        let y_vec = t_odom.pose.pose.position.y - imu_odom.pose.pose.position.y;
        let ro_vec = t_yaw - imu_yaw;


        let x = control_vec(x_vec);
        let y = control_vec(y_vec);
        let ro = control_vec(ro_vec);

        let mut cmd = Wheel::new(0.0, 0.0, 0.0, 0.0);

        cmd.front_left = x * diagonal - y * diagonal + 1.0 * ro;
        cmd.front_right = -x * diagonal - y * diagonal + 1.0 * ro;
        cmd.rear_left = x * diagonal + y * diagonal + 1.0 * ro;
        cmd.rear_right = - x * diagonal + y * diagonal + 1.0 * ro;

        match port.write(cmd.serialize().as_bytes()) {
            Ok(_)=>{
                
            }
            Err(_)=>{
                
            }
        }
    }
}

fn control_vec(value:f64)->f64
{
    if value > 0.05
        {
            if value > 1.0
            {
                0.5
            }
            else if value > 0.5
            {
                0.25
            }
            else {
                0.15
            }
        }
        else if value < -0.05
        {
            if value < -1.0
            {
                -0.5
            }
            else if value < -0.5
            {
                -0.25
            }
            else {
                -0.15
            }
        }
        else
        {
            0.0
        }
}