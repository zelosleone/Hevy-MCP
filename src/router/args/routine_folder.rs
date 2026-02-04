use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreateRoutineFolderArgs {
    pub title: String,
    pub index: Option<u32>,
}
