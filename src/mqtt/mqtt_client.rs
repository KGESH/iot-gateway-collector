use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, DistString};
use std::time::Duration;
use rumqttc::{Client, Connection, ConnectionError, Event, MqttOptions, Packet, QoS};

pub struct MqttClient {
    client_id: String,
    host: String,
    port: u16,
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
        let client_id = MqttClient::generate_random_client_id(10);
        let mut mqtt_options = MqttOptions::new(client_id.as_str(), host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(10));
        mqtt_options.set_clean_session(true);

        let (mut mqtt_client, mut notifications) = Client::new(mqtt_options, 10);


        MqttClient {
            client_id,
            host: String::from(host),
            port,
            mqtt_client,
            notifications,
        }
    }

    /** Subscribe topics here */
    fn subscribe_topics(&mut self) {
        self.mqtt_client.subscribe("hello/rumqtt", QoS::AtLeastOnce);
    }

    pub fn generate_random_client_id(length: usize) -> String {
        Alphanumeric.sample_string(&mut thread_rng(), length)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_client_success() {
        let mqtt_client = MqttClient::new("localhost", 1883);
        assert_eq!("localhost", mqtt_client.host);
        assert_eq!(1883, mqtt_client.port);
    }
}