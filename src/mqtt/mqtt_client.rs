use std::time::Duration;
use rumqttc::{Client, Connection, MqttOptions};

pub struct MqttClient {}

impl MqttClient {
    pub fn create_mqtt_client() -> (Client, Connection) {
        let mut mqtt_options = MqttOptions::new("client_id", "localhost", 1883);
        mqtt_options.set_keep_alive(Duration::from_secs(10));
        mqtt_options.set_clean_session(true);


        // Create the client
        let (mut mqtt_client, mut notifications) = Client::new(mqtt_options, 10);
        (mqtt_client, notifications)
    }
}
