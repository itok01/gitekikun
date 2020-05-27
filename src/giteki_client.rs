use chrono::NaiveDate;

use super::db::*;

pub async fn get_latest_giteki_list() -> Result<giteki::list::Response, Box<dyn std::error::Error>>
{
    let mut giteki_params = giteki::list::RequestParameters::new();
    giteki_params.set_sk(12);
    Ok(giteki::list::get(&giteki_params).await?)
}

pub async fn get_many_giteki_list() -> Result<giteki::list::Response, Box<dyn std::error::Error>> {
    let mut giteki_params = giteki::list::RequestParameters::new();
    giteki_params.set_dc(7);
    giteki_params.set_sk(12);
    Ok(giteki::list::get(&giteki_params).await?)
}

pub async fn update_giteki_db() -> Result<(), Box<dyn std::error::Error>> {
    let database = connect_database();
    database_init(&database);

    let num_resp: giteki::num::Response =
        giteki::num::get(&giteki::num::RequestParameters::new()).await?;
    println!("{}", num_resp.giteki.count);

    let mut end_date = chrono::Utc::today()
        .naive_local()
        .format("%Y%m%d")
        .to_string();

    loop {
        println!("{}", end_date);

        let mut giteki_params = giteki::list::RequestParameters::new();
        giteki_params.set_dc(7);
        giteki_params.set_sk(12);
        giteki_params.set_de(&end_date);
        let resp: giteki::list::Response = giteki::list::get(&giteki_params).await?;

        for giteki_info in resp.giteki {
            let device = GitekiDevice::from_giteki_info(giteki_info);
            if !is_giteki_exists(&database, &device) {
                let dt = NaiveDate::parse_from_str(&device.date, "%Y-%m-%d").unwrap();
                end_date = dt.format("%Y%m%d").to_string();

                //println!(
                //    "{}\n{}の{}が{}として認定されました。",
                //    dt.format("%Y年%m月%d日"),
                //    device.name,
                //    device.type_name,
                //    device.radio_equipment_code
                //);
                add_giteki(&database, device);
            } else {
                println!("存在します");
            }
        }
    }
}
