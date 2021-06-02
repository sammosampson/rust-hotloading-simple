
// Note: this is an opaque type
#[repr(C)]
#[derive(Debug)]
pub struct ApplicationState {
    pub stuff: [u32; 4],
}