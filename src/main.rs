// To get a token, you need to log in to the 
// converter website and go to your profile profile.
// Link -> https://conversiontools.io/profile
// Author: WinsomeQuill (https://github.com/WinsomeQuill)

use std::{collections::HashMap, thread, time};
use conversion_tools_api::api::Api;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let token = std::env::var("TOKEN").expect("TOKEN is invalid!");
    let url = std::env::var("URL").expect("URL is invalid!");

    let api = Api::new(token, url).await;


    file_convert_example(&api, "king_cat.jpg", String::from("king_cat.jpg")).await;
    site_convert_example(&api, "https://github.com/WinsomeQuill").await;

    Ok(())
}

async fn file_convert_example(object: &Api, path: &str, file_name: String) {
    let mut args: HashMap<&str, &str> = HashMap::with_capacity(1); //create HashMap for arguments
    args.insert("orientation", "Portrait"); //argument for convertor

    let upload_result = object
        .upload_file(path, file_name)
        .await
        .expect("Error when upload file!");

    if upload_result.result.is_none() {
        panic!("{}", upload_result.json);
    }

    let file_id = upload_result.result.unwrap().file_id;
    let create_task_result = object
        .create_task("convert.jpg_to_pdf", &file_id, &args)
        .await
        .expect("Error when create task!");

    if create_task_result.result.is_none() {
        panic!("{}", create_task_result.json);
    }

    let task_id = &create_task_result.result.unwrap().task_id;
    loop {
        let get_task_result = object.get_task(task_id)
            .await
            .expect("Error when get task!");

        let task = match get_task_result.result {
            Some(o) => o,
            None => panic!("{}", get_task_result.json),
        };

        match task.status.as_str() {
            "SUCCESS" => {
                println!("[File Convert] Downloading...");
                let file_id = task.file_id.unwrap();
                match object.download_file(&file_id, "result.pdf").await {
                    Ok(_) => println!("[File Convert] Okay"),
                    Err(e) => panic!("[File Convert] {}", e),
                };
                return;
            },
            "ERROR" => {
                println!("[File Convert] Error => {}", get_task_result.json);
                return;
            },
            _ => {
                println!("[File Convert] Wait...");
                thread::sleep(time::Duration::from_millis(1500));
            },
        };
    }
}

async fn site_convert_example(object: &Api, site_url: &str) {
    let mut hash: HashMap<&str, &str> = HashMap::with_capacity(3); //create HashMap for arguments
    hash.insert("url", site_url); //argument for convertor
    hash.insert("images", "yes"); //argument for convertor
    hash.insert("javascript", "yes"); //argument for convertor

    let create_task_result = object
        .create_task("convert.website_to_png", "", &hash)
        .await
        .expect("Error when create task!");

    if create_task_result.result.is_none() {
        panic!("{}", create_task_result.json);
    }

    let task_id = &create_task_result.result.unwrap().task_id;
    loop {
        let get_task_result = object.get_task(task_id)
            .await
            .expect("Error when get task!");

        let task = match get_task_result.result {
            Some(o) => o,
            None => panic!("{}", get_task_result.json),
        };

        match task.status.as_str() {
            "SUCCESS" => {
                println!("[Site Convert] Downloading...");
                let file_id = task.file_id.unwrap();
                match object.download_file(&file_id, "result_site.png").await {
                    Ok(_) => println!("[Site Convert] Okay"),
                    Err(e) => panic!("[Site Convert] {}", e),
                };
                return;
            },
            "ERROR" => {
                println!("[Site Convert] Error => {}", get_task_result.json);
                return;
            },
            _ => {
                println!("[Site Convert] Wait...");
                thread::sleep(time::Duration::from_millis(1500));
            },
        };
    }
}