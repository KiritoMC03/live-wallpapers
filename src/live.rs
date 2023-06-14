pub mod bacteries;
pub mod spatial_hash;

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