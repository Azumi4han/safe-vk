use crate::{
    extract::Ctx,
    parse_response,
    responses::{events::User, Message},
    VK,
};

impl Ctx<Message> {
    pub async fn get_users(&self, user_ids: &[i32]) -> crate::Result<Vec<User>> {
        let serialized = serde_json::to_string(&user_ids[0])?;
        let response = self
            .request
            .post(VK, "users.get", &[("user_ids", &serialized)], {})
            .await?;
        Ok(parse_response!(response, Vec<User>)?)
    }
}
