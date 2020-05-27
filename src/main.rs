use chrono::NaiveDate;

use gitekikun::config::*;
use gitekikun::db::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();

    let token = egg_mode::Token::Access {
        consumer: egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret),
        access: egg_mode::KeyPair::new(config.access_key, config.access_secret),
    };

    let database = connect_database();
    database_init(&database);

    loop {
        let mut giteki_params = giteki::list::RequestParameters::new();
        giteki_params.set_dc(7);
        giteki_params.set_sk(12);
        let resp: giteki::list::Response = giteki::list::get(&giteki_params).await?;

        for giteki_info in resp.giteki {
            let device = GitekiDevice::from_giteki_info(giteki_info);
            if !is_giteki_exists(&database, &device) {
                let dt = NaiveDate::parse_from_str(&device.date, "%Y-%m-%d").unwrap();

                let tweet_text = format!(
                    "{}\n{}の{}が{}として認定されました。",
                    dt.format("%Y年%m月%d日"),
                    device.name,
                    device.type_name,
                    device.radio_equipment_code
                );

                egg_mode::tweet::DraftTweet::new(tweet_text)
                    .send(&token)
                    .await?;

                add_giteki(&database, device);

                let wait_time = std::time::Duration::from_secs(36);
                std::thread::sleep(wait_time);
            }
        }

        let wait_time = std::time::Duration::from_secs(1800);
        std::thread::sleep(wait_time);
    }
}
