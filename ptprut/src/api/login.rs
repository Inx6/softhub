use actix_web::{web, post, HttpRequest, HttpResponse};
use std::collections::HashMap;
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};
use crate::data::information::{Info, Login};
use crate::go_str;

#[post("/login")]
async fn login(up: HttpRequest, req: String, db: web::Data<mysql::Pool>) -> HttpResponse{
    let info: Login = serde_json::from_str(&go_str!(req)).unwrap();
    // println!("{:#?}",info);

    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();

    let s = conn.query_first(format!("select pwd from user where name = '{}'", info.name)).map(
        |row|{
            row.map(| pwds | Info {
                data: pwds
            })
        }
    ).unwrap();
    match s{
        Some(e) => {
            // 密码解密模块
            let parsed_hash = PasswordHash::new(&e.data).unwrap();

            // 密码对账
            if Pbkdf2.verify_password(&info.pwd.as_bytes(), &parsed_hash).is_ok(){
                let sql = format!("select nick,name,avatar,level,status,time from user where name = '{}'", info.name);
                let res: Vec<(String, String, String, i32, i32, String)> = conn.query(sql).unwrap();
                let mut inso = HashMap::new();
                inso.insert("登录成功！", res);
                HttpResponse::Ok()
                    .json(inso)
            }else{
                HttpResponse::Ok()
                    .json(Info{
                        data: "密码错误啦！".to_string()
                    })
            }
        },
        None => {
            HttpResponse::Ok()
                .json(Info{
                    data: "用户不存在啦！".to_string()
                })
        }
    }
}