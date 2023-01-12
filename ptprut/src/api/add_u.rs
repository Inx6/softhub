use actix_web::{web, post, HttpRequest, Responder};
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, 
        SaltString
    },
    Pbkdf2
};
use crate::data::information::Add;
use crate::{go_str, go_search};

#[post("/add_u")]
async fn add_u(_up: HttpRequest, req: String, db: web::Data<mysql::Pool>) -> impl Responder{
    let info: Add = serde_json::from_str(&go_str!(req)).unwrap();
    // println!("{:#?}",info);

    // 密码加密
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Pbkdf2.hash_password(&info.pwd.as_bytes(), &salt).unwrap().to_string();
    // println!("{:?}: {}",&info.pwd.as_bytes(), password_hash);

    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();

    let res: Vec<i32> = conn.query(go_search!("id", "user", format!("nick = '{}' || name = '{}'", info.nick, info.name))).unwrap();
    match res.len() {
        0 => {
            let re2 = conn.exec_drop(
                "insert into user set nick=:nick, name=:name, pwd=:pwd, avatar=:avatar, level=0, status=0, time=:time",
                params!(
                    "nick" => &info.nick, 
                    "name" => &info.name, 
                    "pwd" => password_hash, 
                    "avatar" => &info.avatar, 
                    "time" => &info.time
                )
            );
            match re2{
                Ok(())=>{
                    "成功注册啦！"
                },
                Err(_e) => {
                    "不晓得什么原因，注册出错了......"
                }
            }
        },
        _ => {
            "昵称或者账号被使用了哦"
        }
    }
    // "ok"
}