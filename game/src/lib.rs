use std::alloc::Layout;

#[repr(C)]
pub struct LibraryState {
   // ... lots of fields
}

// Note: the lifetime is actually ignored. 
// The LibraryState's lifetime is manually managed.
#[no_mangle]
pub extern "C" fn initialise<'a>() -> *mut LibraryState {
    // Create the game state
    let game_state: LibraryState = LibraryState{};

    println!("hello init");
    // Turn LibraryState into a manually-managed pointer
    Box::into_raw(Box::new(game_state))
}

#[no_mangle]
pub unsafe extern "C" fn update(game_state: *mut LibraryState) -> bool {
    
    //println!("hello update");
    //println!("hello update2");
    //println!("hello update3");
    if game_state.is_null() {
        panic!("[ FATAL ] game_update: game state is null!");
    }

    let _game_state = &mut *game_state;
    //internal_game_update(LibraryState)
    true
}

// ... reload and unload (both may do nothing at all) ...

#[no_mangle]
pub unsafe extern "C" fn shutdown(game_state: *mut LibraryState) {
    // ... check null ...

    // Destroy the LibraryState
    std::ptr::drop_in_place(game_state);
    // Free its backing memory
    std::alloc::dealloc(game_state as *mut u8, Layout::new::<LibraryState>());
}

#[no_mangle]
pub unsafe extern "C" fn unload(_game_state: *mut LibraryState) {
}

#[no_mangle]
pub unsafe extern "C" fn reload(_game_state: *mut LibraryState) {
}