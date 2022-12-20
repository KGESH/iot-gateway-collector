use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use tokio::time::Duration;

mod mqtt;
mod serial;

use mqtt::mqtt_client::MqttClient;
use serial::serial_client::SerialClient;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let serial_client = SerialClient::new("/dev/tty.usbserial-0001", 115200);

    let (mut mqtt_client, mut notifications) = MqttClient::create_mqtt_client();
    mqtt_client.subscribe("hello/rumqtt", QoS::AtLeastOnce);

    loop {
        match notifications.eventloop.poll().await {
            Ok(event) => {
                println!("Event = {:?}", event);
                if let Event::Incoming(packet) = event.clone() {
                    if let Packet::Publish(publish) = packet {
                        println!("Payload = {:?}", String::from_utf8_lossy(&publish.payload));
                    }
                }
                Ok(event)
            }
            Err(err) => {
                println!("ConnectionError = {:?}", err);
                Err(err)
            }
        };
    }
}

