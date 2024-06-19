mod config;
mod api;

#[tokio::main]
async fn main() {
    open_data_file().await;

    // let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(20).to_std().unwrap());
    let mut interval_timer = tokio::time::interval(chrono::Duration::minutes(7).to_std().unwrap());
    loop {
        interval_timer.tick().await;
        tokio::spawn(async { run_bundle().await; }); // async task
    }

}

use std::{fs::{File, OpenOptions}, io::Write};

use tokio::task::JoinSet;

use crate::{api::config::exec_url_as, config::{print_date, RespItem}};

pub async fn run_bundle() {
    let (dt_str, _dt2_str) = print_date();
    let tt: Vec<RespItem> = lookup_concur().await;
    // for item in tt.iter() {
    //     println!("Print CHECK >> {:?}", item);        
    // }
    
    append_respitem_to_file(&tt, &dt_str).await;
    println!("RES >> {:?}", tt);
    // if the length of the array is > 0 then send SMS to admin
}

pub async fn lookup_concur() -> Vec<RespItem> {
    // let mut handles = vec![];
    let mut results: Vec<config::RespItem> = Vec::new();
    let mut set: JoinSet<RespItem> = JoinSet::new();
    
    for url in config::URLS.iter() {
        set.spawn(exec_url_as(url.to_string()));
    }
    
    while let Some(res) = set.join_next().await {
        let out = res;

        let _ = match out {
            Ok(resp) => {
                // println!("success :: {:#?}", resp);
                if resp.code != 200 {
                    results.push(resp);
                }
            },
            Err(err) => {
                println!("error :: {:#?}", err);
            }
        };
    }

    return results;
}

async fn open_data_file() -> File {
    let data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("monitoring.txt")
        .expect("File could not be found/opened");

    data_file
}

async fn append_respitem_to_file(content: &Vec<RespItem>, cur_date: &str) {
    let mut data_file = append_data_to_file("monitoring.txt").await;

    let content_str: String = content.iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // check for empty string
    if content_str.len() > 0 {
        // eprintln!("String length is: {}", content_str.len());
        let data_string: String = format!("{} | {}", cur_date, content_str);

        // newline
        if let Err(e) = writeln!(data_file, "") {
            eprintln!("Couldn't write newline to file: {}", e);
        }
            
        // Write to a file
        data_file
            .write_all(data_string.as_bytes())
            .expect("write failed");
    }
    
}

async fn append_data_to_file(path: &str) -> File {
    let data_file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("cannot open file");

    data_file
}

// fn append_string_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
//     let mut file = BufWriter::new(file);

//     file.write_all(&data.as_bytes())?;

//     // Or try to write as much as possible, but need
//     // to take care of the remaining bytes yourself
//     let remaining = file.write(&data.as_bytes())?;
//     if remaining != 0 {
//         // handle...
//     }

//     // You definitely need to flush a BufWriter
//     // as it cannot guarantee emptying its buffer
//     // when it goes out of scope
//     file.flush()?;

//     Ok(())
// }



// async fn open_file() {
//     let data_result = File::open("monitoring.json");

//     // Reading a file returns a Result enum
//     // Result can be a file or an error
//     let data_file = match data_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the data file: {:?}", error),
//     };

//     println!("Data file: {:?}", data_file);
// }
