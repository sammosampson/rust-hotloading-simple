use engine::application_state::*;
use std::alloc::Layout;

pub fn create_application_state() -> ApplicationState {
    ApplicationState {
        stuff: [0, 0, 0, 0]
    }
}

pub fn update_application_state(state: &mut ApplicationState) -> bool {
    state.stuff[0] +=2;
    println!("{:?}", state);
    true
}

#[no_mangle]
pub extern "C" fn initialise<'a>() -> *mut ApplicationState {
    println!("init2");
    Box::into_raw(Box::new(create_application_state()))
}

#[no_mangle]
pub unsafe extern "C" fn update(application_state: *mut ApplicationState) -> bool {
    if application_state.is_null() {
        panic!("[ FATAL ] app_update: app state is null!");
    }

    update_application_state(&mut *application_state)
}

#[no_mangle]
pub unsafe extern "C" fn shutdown(application_state: *mut ApplicationState) {
    if application_state.is_null() {
        panic!("[ FATAL ] app_update: app state is null!");
    }

    std::ptr::drop_in_place(application_state);
    std::alloc::dealloc(application_state as *mut u8, Layout::new::<ApplicationState>());
}

#[no_mangle]
pub unsafe extern "C" fn unload(_app_state: *mut ApplicationState) {
}

#[no_mangle]
pub unsafe extern "C" fn reload(_app_state: *mut ApplicationState) {
}