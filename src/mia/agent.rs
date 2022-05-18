use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

#[derive(serde::Serialize)]
struct Agent {
    id: String,

    #[serde(skip_serializing)]
    client: Client,

    #[serde(skip_serializing)]
    connection: Connection,
}

impl Agent {
    pub fn new_with_credentials(
        id: &str,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Self {
        let mut mqtt_options = MqttOptions::new(format!("mia-agent-{}", id), host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        if !username.is_empty() || !password.is_empty() {
            mqtt_options.set_credentials(username, password);
        }

        let (client, connection) = Client::new(mqtt_options, 10);

        Agent {
            id: id.to_string(),
            connection,
            client,
        }
    }

    pub fn new(id: &str, host: &str, port: u16) -> Self {
        Self::new_with_credentials(id, host, port, "", "")
    }

    pub fn run(&mut self) {
        loop {
            self.client
                .publish(
                    "I1820/main/agent/ping",
                    QoS::AtLeastOnce,
                    false,
                    serde_json::to_string(self).unwrap(),
                )
                .unwrap_or(());
            thread::sleep(Duration::from_millis(100));
        }
    }
}
