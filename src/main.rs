use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use tokio::time::Duration;

mod mqtt;
mod serial;

use mqtt::mqtt_client::MqttClient;
use serial::serial_client::SerialClient;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let serial_client = SerialClient::new("/dev/tty.usbserial-0001", 115200);

    let mut mqtt_client = MqttClient::new("localhost", 1883);

    loop {
        mqtt_client.listening_mqtt_broker().await;
    }
}

