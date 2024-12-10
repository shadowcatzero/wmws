use std::{
    collections::BTreeMap,
    env,
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
    process::Command,
};

use serde::Deserialize;

use crate::ws::{Names, State, WS};

#[derive(Deserialize, Debug)]
struct ActiveWSJson {
    pub id: u32,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct MonJson {
    pub id: u32,
    pub name: String,
    pub focused: bool,
    pub activeWorkspace: ActiveWSJson,
}

#[derive(Deserialize, Debug)]
struct WSJson {
    pub id: u32,
    pub monitor: String,
}

pub fn start(mon_id: u32, names: Names) {
    let mut state = create_state(mon_id, names);
    let runtime_dir = env::var("XDG_RUNTIME_DIR").expect("Failed to read wayland runtime dir");
    let isig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("Failed to read hyprland instance signature");
    let stream = UnixStream::connect(format!("{runtime_dir}/hypr/{isig}/.socket2.sock"))
        .or(UnixStream::connect(format!(
            "/tmp/hypr/{isig}/.socket2.sock"
        )))
        .expect("Failed to connect to hyprland socket");
    let reader = BufReader::new(stream);
    state.output();
    for res in reader.lines() {
        match res {
            Ok(line) => {
                parse(&mut state, line);
            }
            Err(err) => eprintln!("Failed to read from hyprland socket: {}", err),
        }
    }
}

fn parse(state: &mut State, line: String) {
    let (cmd, data) = line
        .split_once(">>")
        .map_or((line.as_str(), None), |p| (p.0, Some(p.1)));
    match cmd {
        "focusedmon" => {
            let (mon, _) = data.unwrap().split_once(",").unwrap();
            state.focused = mon == state.mon_name;
        }
        "createworkspace" => {
            if state.focused {
                state.add_ws(data.unwrap().parse().unwrap());
                state.output();
            }
        }
        "destroyworkspace" => {
            if state
                .workspaces
                .remove(&data.unwrap().parse().unwrap())
                .is_some()
            {
                state.output();
            }
        }
        "workspace" => {
            if state.focused {
                if let Some(ws) = state.workspaces.get_mut(&state.prev_focused) {
                    ws.focused = false;
                }
            }
            let id = data.unwrap().parse().unwrap();
            if let Some(ws) = state.workspaces.get_mut(&id) {
                ws.focused = true;
                state.prev_focused = id;
            }
            state.output();
        }
        "moveworkspace" => {
            let (ws_name, mon_name) = data.unwrap().split_once(",").unwrap();
            if mon_name == state.mon_name {
                state.add_ws(ws_name.parse().unwrap());
                state.output();
            } else {
                state.workspaces.remove(&ws_name.parse().unwrap());
                state.output();
            }
        }
        _ => {}
    }
}

fn create_state(mon_id: u32, names: Names) -> State {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .expect("Failed to get hyprland monitors")
        .stdout;
    let json_str = String::from_utf8(output).expect("Failed to get string from hyprland output");
    let mons: Vec<MonJson> = serde_json::from_str(&json_str).expect("failed to read hyprland json");

    let output = Command::new("hyprctl")
        .args(["workspaces", "-j"])
        .output()
        .expect("Failed to get hyprland workspaces")
        .stdout;
    let json_str = String::from_utf8(output).expect("Failed to get string from hyprland output");
    let mon = &mons
        .iter()
        .find(|m| m.id == mon_id)
        .unwrap_or_else(|| panic!("No monitor with id {} found", mon_id));
    let mon_name = mon.name.clone();
    let active_id = mon.activeWorkspace.id;
    let wss: Vec<WSJson> = serde_json::from_str(&json_str).expect("failed to read hyprland json");
    let wss = wss.iter().filter(|ws| ws.monitor == mon_name).map(|ws| {
        (
            ws.id,
            WS {
                id: ws.id,
                name: names.get(&ws.id).unwrap_or(&"?".to_string()).clone(),
                focused: ws.id == active_id,
                urgent: false,
            },
        )
    });

    State {
        mon_name: mon_name.to_owned(),
        names: names.to_owned(),
        focused: mon.focused,
        workspaces: BTreeMap::from_iter(wss),
        prev_focused: active_id,
    }
}
