use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const WSL_INSTANCE_NAME: &str = "suanleme-instance";

fn is_port_occupied(port: u16) -> bool {
    // 使用 PowerShell 执行 netstat 命令并检查端口是否被占用
    let output = Command::new("powershell.exe")
        .arg("-Command")
        .arg(format!(
            "netstat -ano | Select-String -Pattern \":{} \"",
            port
        ))
        .output()
        .expect("Failed to execute PowerShell command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    // 检查输出中是否包含 "LISTENING" 字样
    output_str.contains("LISTENING")
}

fn check_ports(ports: &[u16]) -> bool {
    for &port in ports {
        // println!("Checking port {}", port);
        if !is_port_occupied(port) {
            return false;
        }
    }
    true
}

fn clear_docker_containers() {
    // println!("Clearing docker containers...");
    let stop_output = Command::new("docker")
        .arg("stop")
        .arg("$(docker ps -aq)")
        .output()
        .expect("Failed to stop Docker containers");

    let rm_output = Command::new("docker")
        .arg("rm")
        .arg("$(docker ps -aq)")
        .output()
        .expect("Failed to remove Docker containers");

    // println!(
    //     "Docker stop output: {:?}",
    //     String::from_utf8_lossy(&stop_output.stdout)
    // );
    // println!(
    //     "Docker remove output: {:?}",
    //     String::from_utf8_lossy(&rm_output.stdout)
    // );
}

fn shutdown_wsl() {
    // println!("Shutting down wsl...");
    Command::new("wsl.exe")
        .arg("--terminate")
        .arg(WSL_INSTANCE_NAME)
        .output()
        .expect("Failed to shutdown WSL instance");
}

fn main() {
    let ports_to_check = vec![9552, 49321, 8251, 33663]; // 在这里定义要检查的端口列表

    loop {
        if check_ports(&ports_to_check) {
            // println!("All ports are occupied. Continuing...");
        } else {
            // println!("One or more ports are not occupied. Clearing Docker containers and shutting down...");
            clear_docker_containers();
            shutdown_wsl();
            break;
        }
        sleep(Duration::from_secs(5));
    }
}
