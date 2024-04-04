use safe_drive::{
    context::Context, 
    error::DynError, 
    logger::Logger, 
    msg::common_interfaces::geometry_msgs, 
    pr_error, 
    pr_info, 
};
use async_std;
use serialport;
use zikoichi_controller::Wheel;

#[async_std::main]
async fn main()->Result<(), DynError>
{
    let ctx = Context::new()?;
    let node = ctx.create_node("ZikoichiManual", None, Default::default())?;
    let mut subscriber = node.create_subscriber::<geometry_msgs::msg::Twist>("/cmd_vel", None)?;
    let log = Logger::new(node.get_name());

    let mut port = serialport::new("/dev/ttyACM0", 115200).timeout(std::time::Duration::from_millis(100)).open().unwrap();

    let diagonal = ((2.0_f32).sqrt() / 2.0) as f64;

    pr_info!(log, "Start {}", node.get_name());

    loop {
        let get_msg = subscriber.recv().await.unwrap();

        let x = get_msg.linear.x;
        let y = get_msg.linear.y;
        let ro = get_msg.angular.z;

        let mut cmd = Wheel::new(0.0, 0.0, 0.0, 0.0);

        cmd.front_left = x * diagonal - y * diagonal + 0.5 * ro;
        cmd.front_right = -x * diagonal - y * diagonal + 0.5 * ro;
        cmd.rear_left = x * diagonal + y * diagonal + 0.5 * ro;
        cmd.rear_right = - x * diagonal + y * diagonal + 0.5 * ro;

        match port.write(cmd.serialize().as_bytes()) {
            Ok(_)=>{
            }
            Err(e)=>{
                pr_error!(log, "Err {:?}", e);
            }
        }
    }
}