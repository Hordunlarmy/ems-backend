use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse<T>
where
    T: Serialize,
{
    message: String,
    status_code: u16,
    data: Option<T>,
    meta: Option<serde_json::Value>,
}

impl<T: Serialize> APIResponse<T> {
    pub fn success(data: T, meta: Option<serde_json::Value>) -> Self {
        Self {
            message: "Success".to_string(),
            status_code: 200,
            data: Some(data),
            meta,
        }
    }

}
