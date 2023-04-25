pub mod app;
mod gen3;
mod gen4;
mod util;

#[derive(Ord, PartialOrd, Eq, PartialEq, Default, Copy, Clone)]
pub enum ProfileEditorResult {
    #[default]
    Pending,
    New,
    Edit,
    Cancel,
}
