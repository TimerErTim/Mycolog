#[derive(Clone, Debug, Default)]
pub struct MycologResult {
    pub(crate) exit_code: i32,
    pub(crate) error: Option<anyhow::Error>,
}
