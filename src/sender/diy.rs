use serde::Deserialize;

use crate::receiver::slack;

#[derive(Deserialize)]
pub struct Slice {
    name: Vec<String>,
}

impl slack::ToSlack for Slice {
    fn to_slack(self) -> slack::Msg {
        let inner = slack::RichText::from(self.name);

        slack::Msg::Rich(inner)
    }
}
