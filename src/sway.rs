use swayipc::{Connection, Event, EventType};

use crate::ws::State;

pub fn start(output: u32) {
    let mut state = State::new();

    let sway = Connection::new().expect("Failed to connect to sway");

    let mut sway = Connection::new().expect("Failed to connect to sway");
    let wss = &sway.get_workspaces().expect("Failed to get workspaces");

    let stream = sway
        .subscribe([EventType::Workspace])
        .expect("Failed to subscribe to workspace events");
    for e in stream {
        if let Event::Workspace(wse) = e.expect("Failed to read sway event") {
            match wse.change {
                swayipc::WorkspaceChange::Init => if let Some(ws) = wse.current {},
                swayipc::WorkspaceChange::Move => {}
                swayipc::WorkspaceChange::Empty => {}
                swayipc::WorkspaceChange::Focus => {}
                swayipc::WorkspaceChange::Rename => {}
                swayipc::WorkspaceChange::Urgent => {}
                swayipc::WorkspaceChange::Reload => {},
                _ => {}
            }
            state.output();
        }
    }
}
