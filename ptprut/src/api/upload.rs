use actix_web::{web, post, HttpRequest, Responder};
use crate::data::information;

// 上传api
#[post("/add_url")]
pub async fn add(up: HttpRequest, req: String, db: web::Data<mysql::Pool>) -> impl Responder{
    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();

    // ip地址查询
    // if ips(format!("{}", up.peer_addr().unwrap())).await == String::from("地址异常"){
    //     "<h1>你来自哪个星球？</h1><h1>where are you from?</h1>"
    // }else{
        // 格式化请求数据
        let dt: information::Apk = serde_json::from_str(&(format!("{}",req))).unwrap();

        // 数据库应用下载地址（多地址）查询并进行requewt
        let nums: Vec<i32> = conn.query(format!("select id from address where address = '{}'", dt.url)).unwrap();
        if nums.len() >= 1{
            "应用已经存在了哦！"
        }else{
            let res = conn.exec_drop(
                "insert into address set address =:url , word = :name, icon = :icon, type = :ty, label = :lb, version = :vr, Package = :pg",
                params!{
                    "url" => &dt.url,
                    "name"  => &dt.name,
                    "icon" => &dt.ic,
                    "ty" => &dt.ty,
                    "lb" => &dt.label,
                    "vr" => &dt.version,
                    "pg" => &dt.package
                }
            );
            match res{
                Ok(())=>{
                    "上传成功啦！"
                },
                Err(_e) => {
                    "不晓得什么原因，上传出错了......"
                }
            }
        }
    // }
}