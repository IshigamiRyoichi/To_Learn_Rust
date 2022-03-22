// 非同期処理を行うため、ここの記述が必要
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "http://openccpm.com/blog";
//     println!("call {}",url);
//     // 指定したURLの呼び出しが可能
//     // awaitで対応を待つために
//     // ？はエラーが発生した際にmain関数に返すために付ける
//     let res = reqwest::get(url).await?;
//     // body部分を抜き出している
//     let body = res.text().await?;
//     println!("response is \n{}", body);
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let n = 7;
//     let url = format!("http://openccpm.com/blog/?p={}",n);
//     println!("call {}",url);
//     let res = reqwest::get(&url).await?;
//     let body = res.text().await?;
//     println!("response is \n{}", body);
//     Ok(())
// }

use reqwest::StatusCode;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "http://openccpm.com/blog/unkown.txt";
//     println!("call {}",url);
//     let res = reqwest::get(url).await?;
//     match res.status() {
//         StatusCode::OK => {
//             let body = res.text().await?;
//             println!("response is \n{}", body);
//         },
//         StatusCode::NOT_FOUND => {
//             println!("error: 目的のページがありませんでした。");
//         },
//         _ => {
//             println!("error: その他のエラーが発生しました。");
//         },
//     }
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>>{
//     let url = "http://unknown.openccpm.com/blog";
//     println!("call {}",url);
//     if let Ok(res) = reqwest::get(url).await{
//         let body = res.text().await?;
//         println!("response is \n{}",body);
//     } else {
//         println!("error: Webサーバーが見つかりませんでした。");
//     }
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let url = "http://openccpm.com/redmine/projects.json";
    println!("call {}",url);
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let projects = json["projects"].as_array().unwrap();
    for item in projects{
        let identifier = &item["identifier"].as_str().unwrap();
        let name = &item["name"].as_str().unwrap();
        println!("tag: {} {}", identifier, name);
    }
    Ok(())
}