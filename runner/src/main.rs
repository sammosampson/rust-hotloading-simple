mod reloadable_libraries;

use core::time;
use std::thread;
use engine::application_state::*;
use reloadable_libraries::*;

struct Application {
    app_library: HotReloadableLibrary,
    state: *mut ApplicationState
}

impl Application {
    fn new(library_folder: &str) -> Self {
        let app_library = HotReloadableLibrary::new(library_folder, "app");
        let state = Application::create_state(&app_library);
        Self {
            app_library,
            state 
        }
    }

    fn create_state(library: &HotReloadableLibrary) -> *mut ApplicationState {
        library.load_symbol::<fn() -> *mut ApplicationState>("initialise")()
    }

    fn update_state(&self) -> bool {
        self.app_library.load_symbol::<fn(*mut ApplicationState) -> bool>("update")(self.state)
    }

    fn shutdown(&self) {
        self.app_library.load_symbol::<fn(*mut ApplicationState)>("shutdown")(self.state)
    }

    fn unload(&self) {
        self.app_library.load_symbol::<fn(*mut ApplicationState)>("unload")(self.state)
    }

    fn reload(&self) {
        self.app_library.load_symbol::<fn(*mut ApplicationState)>("reload")(self.state)
    }

    fn reload_app_library_if_changed(&mut self) {
        if !self.app_library.has_changed() {
            return;
        }

        self.unload();
        self.app_library.reload();
        self.reload();
    }
}

fn main() {
    let mut app = Application::new("target/debug");
    
    loop {
        if !app.update_state() {
            break;
        }

        app.reload_app_library_if_changed();
        
        thread::sleep(time::Duration::from_millis(200));
    }

    app.shutdown();
}