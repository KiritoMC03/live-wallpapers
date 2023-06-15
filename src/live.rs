pub mod app;
pub mod physics;
pub mod graphics;
pub mod bacteries;

#[derive(Default)]
pub struct LiveData {
    pub bacteries: bacteries::Bacteries,
}

impl LiveData {
    pub const fn empty() -> LiveData {
        LiveData {
            bacteries: bacteries::Bacteries::empty(),
        }
    }
}