mod game_api;
mod reloadable_libraries;

use core::time;
use std::thread;
use game_api::LibraryState;
use reloadable_libraries::*;

struct Application {
    game_library: HotReloadableLibrary,
    state: *mut LibraryState
}

impl Application {
    fn new(library_folder: &str) -> Self {
        let game_library = HotReloadableLibrary::new(library_folder, "game");
        let state = Application::create_state(&game_library);
        Self {
            game_library,
            state 
        }
    }

    fn create_state(library: &HotReloadableLibrary) -> *mut LibraryState {
        library.load_symbol::<fn() -> *mut LibraryState>("initialise")()
    }

    fn update_state(&self) -> bool {
        self.game_library.load_symbol::<fn(*mut LibraryState) -> bool>("update")(self.state)
    }

    fn shutdown(&self) {
        self.game_library.load_symbol::<fn(*mut LibraryState)>("shutdown")(self.state)
    }

    fn unload(&self) {
        self.game_library.load_symbol::<fn(*mut LibraryState)>("unload")(self.state)
    }

    fn reload(&self) {
        self.game_library.load_symbol::<fn(*mut LibraryState)>("reload")(self.state)
    }

    fn reload_game_library_if_changed(&mut self) {
        if !self.game_library.has_changed() {
            return;
        }

        self.unload();
        self.game_library.reload();
        self.reload();
    }
}

fn main() {
    let mut app = Application::new("target/debug");
    
    loop {
        if !app.update_state() {
            break;
        }

        app.reload_game_library_if_changed();
        
        thread::sleep(time::Duration::from_millis(200));
    }

    app.shutdown();
}