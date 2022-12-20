use std::time::Duration;
use serialport::SerialPort;


pub struct SerialClient {
    port: Box<dyn SerialPort>,

}

impl SerialClient {
    pub fn check_ports() {
        let ports = serialport::available_ports().expect("serial port not found!");
        for port in ports {
            println!("port name: {}", port.port_name);
        }
    }

    pub fn new(port_name: &str, baudrate: u32) -> SerialClient {
        let port = serialport::new(port_name, baudrate)
            .timeout(Duration::from_millis(10))
            .open()
            .unwrap_or_else(|e| {
                eprintln!("Fail to open serial port {}. Error: {}", port_name, e);
                ::std::process::exit(1);
            });

        SerialClient {
            port
        }
    }
}