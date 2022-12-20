use std::time::Duration;
use rumqttc::{Client, Connection, ConnectionError, Event, MqttOptions, Packet, QoS};

pub struct MqttClient {
    mqtt_client: Client,
    notifications: Connection,
}

impl MqttClient {
    pub fn new(host: &str, port: u16) -> MqttClient {
        let mut mqtt_client = MqttClient::create_mqtt_client(host, port);
        mqtt_client.subscribe_topics();

        mqtt_client
    }

    pub fn create_mqtt_client(host: &str, port: u16) -> MqttClient {
        let mut mqtt_options = MqttOptions::new("client_id", host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(10));
        mqtt_options.set_clean_session(true);

        let (mut mqtt_client, mut notifications) = Client::new(mqtt_options, 10);
        println!("created client: {}", host);
        MqttClient {
            mqtt_client,
            notifications,
        }
    }

    /** Subscribe topics here */
    fn subscribe_topics(&mut self) {
        self.mqtt_client.subscribe("hello/rumqtt", QoS::AtLeastOnce);
    }

    pub async fn listening_mqtt_broker(&mut self) {
        match self.notifications.eventloop.poll().await {
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
