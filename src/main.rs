// To get a token, you need to log in to the 
// converter website and go to your profile profile.
// Link -> https://conversiontools.io/profile
// Author: WinsomeQuill (https://github.com/WinsomeQuill)

use std::{collections::HashMap, thread, time};
use conversion_tools_api::api::Api;

const TOKEN: &str = "YOUR TOKEN";
const URL: &str = "https://api.conversiontools.io/v1/"; //API URL

fn main() {
    let object: Api = Api::new(TOKEN.to_string(), URL.to_string());
    file_convert_example(&object, &"king_cat.jpg");
    site_convert_example(&object, &"https://quartzland.ru/");
}

fn file_convert_example(object: &Api, path: &str) {
    let mut args: HashMap<&str, &str> = HashMap::new(); //create HashMap for arguments
    args.insert("orientation", "Portrait"); //argument for convertor

    let upload_result = object.upload_file(&path).expect("Error when upload file!");

    if upload_result.error != None {
        panic!("{:?}", upload_result.error);
    }

    let create_task_result = object.create_task(&"convert.jpg_to_pdf", &upload_result.file_id, &args)
        .expect("Error when create task!");

    if create_task_result.error != None {
        panic!("{:?}", create_task_result.error);
    }

    let mut get_task_result = object.get_task(&create_task_result.task_id)
        .expect("Error when get task!");

    if get_task_result.error != None {
        panic!("{:?}", get_task_result.error);
    }

    loop {
        get_task_result = object.get_task(&create_task_result.task_id)
            .expect("Error when get task!");
        if get_task_result.status == "SUCCESS" {
            break;
        } else if get_task_result.status == "ERROR" {
            println!("[File Convert] Error => {:?}", get_task_result.error);
            return;
        } else {
            println!("[File Convert] Wait...");
            thread::sleep(time::Duration::from_millis(1500));
        }
    }

    println!("[File Convert] Downloading...");
    let file_id = get_task_result.file_id.unwrap();
    match object.download_file(&file_id, "result.pdf") {
        Ok(_) => println!("[File Convert] Okay"),
        Err(e) => panic!("[File Convert] {}", e),
    };
}

fn site_convert_example(object: &Api, site_url: &str) {
    let mut hash: HashMap<&str, &str> = HashMap::new(); //create HashMap for arguments
    hash.insert("url", &site_url); //argument for convertor
    hash.insert("images", "yes"); //argument for convertor
    hash.insert("javascript", "yes"); //argument for convertor

    let create_task_result = object.create_task(&"convert.website_to_png", "", &hash)
        .expect("Error when create task!");

    if create_task_result.error != None {
        panic!("{:?}", create_task_result.error);
    }

    let mut get_task_result = object.get_task(&create_task_result.task_id)
        .expect("Error when get task!");

    if get_task_result.error != None {
        panic!("{:?}", get_task_result.error);
    }

    loop {
        get_task_result = object.get_task(&create_task_result.task_id)
            .expect("Error when get task!");
        if get_task_result.status == "SUCCESS" {
            break;
        } else if get_task_result.status == "ERROR" {
            println!("[Site Convert] Error => {:?}", get_task_result.error);
            return;
        } else {
            println!("[Site Convert] Wait...");
            thread::sleep(time::Duration::from_millis(1500));
        }
    }

    println!("[Site Convert] Downloading...");
    let file_id = get_task_result.file_id.unwrap();
    match object.download_file(&file_id, "conv.png") {
        Ok(_) => println!("[Site Convert] Okay"),
        Err(e) => panic!("[Site Convert] {}", e),
    };
}