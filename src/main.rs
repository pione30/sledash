use std::env;

mod emoji_list;

#[tokio::main]
async fn main() {
    let env_key = "SLACK_APP_ACCESS_TOKEN";
    let token =
        env::var(env_key).expect("SLACK_APP_ACCESS_TOKEN environment variable should be fetched");

    let response = emoji_list::fetch(&token).await;
    if response.ok {
        println!("{:#?}", response.emoji);
    } else {
        eprintln!(
            "Error is returned from emoji.list API: {}",
            response
                .error
                .expect("Some error value should be present when the response_body.ok is false")
        );
    }
}
