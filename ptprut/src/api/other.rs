use std::collections::HashMap;

// 应用查询
pub async fn gets(data: &String) -> String{
    // 伪装浏览器
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36 Edg/108.0.1462.54".parse().unwrap());
    let ds = reqwest::Client::new()
        .get(data)
        .headers(headers)
        .send()
        .await;

    // 分析请求返回数据
    let res = match ds{
        Ok(e)=>{
            if e.status() != 200{
                format!("Other")
            }else{
                format!("Ok")
            }
        },
        Err(_e)=>{
            format!("Err")
        }
    };
    String::from(res)
}
// ip地址查询
pub async fn ips(data: &str) -> String{
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36 Edg/108.0.1462.54".parse().unwrap());

    println!("{}",data);
    let info = format!("https://www.fkcoder.com/ip?ip={}",data);
    // println!("{}",info);
    let ds = reqwest::Client::new()
        .get(&info)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json::<HashMap<String, serde_json::Value>>()
        .await;
    
    // 分析请求返回数据
    match ds{
        Ok(e)=>{
            match e.get("country"){
                Some(es) => {
                    let nrt = serde_json::to_string(&e).unwrap();
                    nrt
                },
                None => {
                    String::from("地址异常")
                }
            }
        },
        Err(_e)=>{
            String::from("地址异常")
        }
    }
}
