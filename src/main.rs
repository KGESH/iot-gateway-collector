use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use tokio::time::Duration;

mod mqtt;

use mqtt::mqtt_client::MqttClient;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Set up the client options
    let (mut mqtt_client, mut notifications) = MqttClient::create_mqtt_client();


    // Subscribe to a topic
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

