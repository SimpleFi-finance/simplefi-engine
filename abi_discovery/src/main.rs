use redis::AsyncCommands;
use third_parties::redis::{ connect };
use settings::{ load_settings };

#[tokio::main]
fn main() {
    let settings = load_settings().expect("Failed to load settings");
    let redis_uri = settings.redis_uri.to_string();

    let mut con = connect(redis_uri.as_str()).await.unwrap();



    println!("{:?}", result);

}
