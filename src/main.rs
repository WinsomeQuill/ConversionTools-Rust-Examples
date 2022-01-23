// To get a token, you need to log in to the 
// converter website and go to your profile profile.
// Link -> https://conversiontools.io/profile
// Author: WinsomeQuill (https://github.com/WinsomeQuill)

use std::{collections::HashMap, thread, time};
use conversion_tools_api::api::{get_tasks, upload_file, create_task, get_task, download_file};
use serde_json::{Value, from_str};

const TOKEN: &str ="YOUR TOKEN";
const URL: &str = "https://api.conversiontools.io/v1/"; //API URL

fn main() {
    file_convert_example(&"king_cat.jpg");
    site_convert_example(&"https://quartzland.ru/");
    //get_all_task_example();
}

fn file_convert_example(path: &str) {
    let mut args: HashMap<&str, &str> = HashMap::new();
    args.insert("orientation", "Portrait");

    let upload_result: Value = from_str(&upload_file(URL, TOKEN, &path)).unwrap();
    let create_task_result: Value = from_str(&create_task(URL, TOKEN, &"convert.jpg_to_pdf", &upload_result["file_id"].as_str().unwrap(), &args)).unwrap();
    let mut get_task_result: Value = from_str(&get_task(URL, TOKEN, &create_task_result["task_id"].as_str().unwrap())).unwrap();

    while get_task_result["status"] != "SUCCESS" {
        thread::sleep(time::Duration::from_millis(1500));
        get_task_result = from_str(&get_task(URL, TOKEN, &create_task_result["task_id"].as_str().unwrap())).unwrap();
        println!("Wait...");
    }

    println!("Downloading...");
    download_file(URL, TOKEN, &get_task_result["file_id"].as_str().unwrap(), "result.pdf");
    println!("Okay");
}

fn site_convert_example(site_url: &str) {
    let mut hash: HashMap<&str, &str> = HashMap::new();
    hash.insert("url", &site_url);
    hash.insert("images", "yes");
    hash.insert("javascript", "yes");

    let create_task_result: Value = from_str(&create_task(URL, TOKEN, &"convert.website_to_png", "", &hash)).unwrap();
    let mut get_task_result: Value = from_str(&get_task(URL, TOKEN, &create_task_result["task_id"].as_str().unwrap())).unwrap();
    while get_task_result["status"] != "SUCCESS" {
        thread::sleep(time::Duration::from_millis(1500));
        get_task_result = from_str(&get_task(URL, TOKEN, &create_task_result["task_id"].as_str().unwrap())).unwrap();
        println!("Wait...");
    }

    println!("Downloading...");
    download_file(URL, TOKEN, &get_task_result["file_id"].as_str().unwrap(), "conv.png");
    println!("Okay");
}

fn get_all_task_example() {
   println!("{}", get_tasks(URL, TOKEN))
}