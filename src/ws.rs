use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

pub type Names = HashMap<u32, String>;

#[derive(Serialize, Deserialize)]
pub struct WS {
    pub id: u32,
    pub name: String,
    pub focused: bool,
    pub urgent: bool,
}

pub struct State {
    pub mon_name: String,
    pub names: Names,
    pub focused: bool,
    pub prev_focused: u32,
    pub workspaces: BTreeMap<u32, WS>,
}

impl State {
    pub fn output(&self) {
        // yeah ok fine this isn't the most efficient
        // a for loop would be better so I'm not allocing
        let output = self
            .workspaces
            .values()
            .map(|w| serde_json::to_string(w).expect("failed to serialize"))
            .collect::<Vec<String>>()
            .join(",");
        println!("[{}]", output);
    }
    pub fn add_ws(&mut self, id: u32) {
        self.workspaces.insert(
            id,
            WS {
                id,
                name: self.names.get(&id).unwrap_or(&"?".to_string()).clone(),
                focused: false,
                urgent: false,
            },
        );
    }
}
