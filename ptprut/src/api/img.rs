use actix_web::{web, get, HttpRequest, HttpResponse};
use crate::data::information::Info;
use crate::api::other::gets;

// 图片查询api
#[get("/")]
pub async fn img(up: HttpRequest, req: web::Query<Info>, db: web::Data<mysql::Pool>) -> HttpResponse{
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();

    // // ip地址查询
    // if ips(format!("{}", up.peer_addr().unwrap())).await == String::from("地址异常"){
    //     HttpResponse::Ok()
    //         .header("Rust", "Web")
    //         .body("<h1>你来自哪个星球？</h1><h1>where are you from?</h1>")
    // }else{
        // 数据库应用图片地址（多地址）查询并进行requewt
        let res: Vec<String> = conn.query(format!("select url from pubapi where uuid = '{}'", req.data)).unwrap();
        let mut url: Vec<(i32, String)> = Vec::new();
        for i in res{
            // 地址通讯测试
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

        HttpResponse::Ok()
            .header("Rust", "Web")
            .json(url)
    // }
}