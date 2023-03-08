use serde::ser::Serialize;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct LightControl {
    ip: String,
    user_id: String,
    room_id: u8,
    scene_config: HashMap<String, String>,
}

impl LightControl {
    pub fn execute_command(&self, command: String) {
        if self.scene_config.contains_key(&command) {
            self.trigger_scene(command)
        } else if command == "off" {
            self.power_off()
        } else {
            println!("No idea what you mean with: {command}")
        }
    }

    fn trigger_scene(&self, name: String) {
        let scene_code = self.scene_config.get(&name);

        let payload = HashMap::from([("scene", scene_code)]);

        self.run_request(payload);
    }

    fn power_off(&self) {
        let payload = HashMap::from([("on", false)]);

        self.run_request(payload);
    }

    fn run_request<T: Serialize>(&self, payload: T) {
        let path = format!(
            "http://{0}/api/{1}/groups/{2}/action",
            self.ip, self.user_id, self.room_id
        );

        let res = reqwest::blocking::Client::new()
            .put(path)
            .json(&payload)
            .send();

        match res {
            Ok(inner_res) => {
                if !inner_res.status().is_success() {
                    println!("Error: {}", inner_res.text().unwrap());
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
