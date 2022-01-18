use serde::Serialize;

#[derive(Serialize)]
pub struct SlackMessage {
    pub text: String
}

pub trait ToSlack {
    fn to_slack(self) -> SlackMessage;
}

pub async fn send<T>(msg: T) -> String
    where T: ToSlack
{
    let msg = msg.to_slack();

    let client = reqwest::Client::new();
    let res = client.post("https://hooks.slack.com/services/xxxx/xxxxx/xxxxxxx")
        .json(&msg)
        .send()
        .await
        .unwrap();

    res.text().await.unwrap()
}
