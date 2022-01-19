use serde::Serialize;

#[derive(Serialize)]
pub enum Msg {
    Text(Text),
    Rich(RichText),
}

#[derive(Serialize)]
pub struct Text {
    pub text: String
}

#[derive(Serialize, Debug)]
pub struct RichText {
    blocks: Vec<Block>
}

impl RichText {
    pub fn from(data: Vec<String>) -> RichText {
        let blocks = data.iter()
            .map(|x| Block{ name: x.to_string() })
            .collect();
        
        RichText{
            blocks: blocks,
        }
    }
}

#[derive(Serialize, Debug)]
struct Block {
    name: String
}

pub trait ToSlack {
    fn to_slack(self) -> Msg;
}

pub async fn send<T>(msg: T) -> String
    where T: ToSlack
{
    let msg = msg.to_slack();

    match msg {
        Msg::Text(t) => {
            let client = reqwest::Client::new();
            let res = client.post("https://hooks.slack.com/services/xxxx/xxxxx/xxxxxxx")
                .json(&t)
                .send()
                .await
                .unwrap();

            res.text().await.unwrap()
        },
        Msg::Rich(r) => {
            format!("{:?}", r)
        }
    }
}
