use actix_web::{web, post, HttpRequest, HttpResponse};
use crate::api::other::{ips, gets};
use crate::data::information;

// 应用链接查询api
#[post("/")]
pub async fn home(up: HttpRequest, req: String, db: web::Data<mysql::Pool>) -> HttpResponse{
    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();

    // // ip地址查询
    // if ips(format!("{}", up.peer_addr().unwrap())).await == String::from("地址异常"){
    //     HttpResponse::Ok()
    //         .header("Rust", "Web")
    //         .body("<h1>你来自哪个星球？</h1><h1>where are you from?</h1>")
    // }else{
        // 格式化请求数据
        let dt: information::Info = serde_json::from_str(&(format!("{}",req))).unwrap();

        // 数据库应用下载地址（多地址）查询并进行requewt
        let res: Vec<String> = conn.query(format!("select address from address where word = '{}'", dt.data)).unwrap();
        let mut url: Vec<(i32, String)> = Vec::new();
        for i in res{
            // 重载查询
            let df = gets(&i).await;
            if df == String::from("Ok"){
                url.push((1, i));
            }else{
                if df == String::from("Other"){
                    url.push((0, i));
                }else{
                    url.push((-1, i));
                }
            };
        };

        // println!("{:?}",url);
        HttpResponse::Ok()
            .header("Rust", "Web")
            .json(url)
    // }
}