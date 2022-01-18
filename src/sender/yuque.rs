use serde::Deserialize;

use crate::receiver::slack;

#[derive(Deserialize)]
pub struct Yuque {
    data: YuqueData,
}

#[derive(Deserialize)]
struct YuqueData {
    id: u32,
    path: String,
    action_type: String,
}

impl slack::ToSlack for Yuque {
    fn to_slack(self) -> slack::SlackMessage {
        slack::SlackMessage{
            text: format!("id: {}, action: {}, path: {}", self.data.id, self.data.action_type, self.data.path),
        }
    }
}
