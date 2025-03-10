use std::process::{Child, Command, Stdio};
use serde::{ Deserialize, Serialize };
use crate::DisplayAction;

use crate::{
    models::{ Handle, WindowHandle },
    Config,
    DisplayServer,
    Manager,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum ReleaseScratchPadOption<H: Handle> {
    #[serde(bound = "")] Handle(WindowHandle<H>),
    None,
}


// todo: remove sh -c unneeeded
// can also exxec with child proc don't need
fn run_cmd(cmd: &str, args: &str) -> Result<Child, Box<dyn std::error::Error>> {
    Command::new(cmd)
        .arg(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(Into::into)
}

// find window by name 

pub fn find_and_move<H: Handle, C: Config, SERVER: DisplayServer<H>>(
    manager: &mut Manager<H, C, SERVER>,
    program: &String,
) -> bool {

    // check minus 1 indexing
    let tag_handle = manager.state.focus_manager.tag(0).unwrap_or_default() - 1;
    let tag = manager.state.tags.get(tag_handle).unwrap().clone();

    let margin_multiplier = manager
    .state
    .windows
    .iter()
    .find(|w| w.has_tag(&tag.id))
    .map_or(1.0, |w| w.margin_multiplier());

    if manager.state.focus_manager.workspace(&manager.state.workspaces).is_some() {
        let window_handle = manager.state.windows.iter_mut()
            .find(|win| win.name.as_ref().map_or(false, |name| name == program)) ;
        let handle_handle = if window_handle.is_some() {
            Some(window_handle.as_ref().unwrap().handle)} 
            else {None};
        
        
        if let Some(handle) = window_handle {
            handle.untag();
            handle.set_floating(false);
            handle.tag(&tag_handle);
            handle.apply_margin_multiplier(margin_multiplier);
        }


        if handle_handle.is_some() {
            let act = DisplayAction::SetWindowTag(handle_handle.unwrap(), Some(tag.id));
            manager.state.actions.push_back(act);
            manager.state.handle_single_border(manager.config.border_width());
            manager.state.goto_tag_handler(tag_handle);
            manager.state.handle_window_focus(&handle_handle.unwrap());
        }
    };
    false
}


pub fn spawn_window<H: Handle, C: Config, SERVER: DisplayServer<H>>(
    manager: &mut Manager<H, C, SERVER>,
    program: &String,
    args: &Vec<String>,
) -> Option<bool> {
   
    if find_and_move(manager, program) {
        return Some(true);
    } else {
        // don't need to do anything event loop will handle it we can just spawn new 
        run_cmd(program, &args.join(" ")).unwrap();
    }
    Some(true)
}
