use actix_web::{web, get, HttpRequest, HttpResponse};
use crate::data::information::Ia;
use std::collections::HashMap;

#[get("/index")]
async fn index(req: web::Query<Ia>, db: web::Data<mysql::Pool>) -> HttpResponse{
    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();
    
    let pagetotal: Vec<i32> = conn.query("SELECT COUNT(*) FROM address").unwrap();
    if req.size == 0{
        let sql = "select * from address limit 0, 10";
        let res: Vec<(i32, String, String, String, String, String, String, String)> = conn.query(sql).unwrap();
        let mut info = HashMap::new();
        info.insert(pagetotal[0], res);
        HttpResponse::Ok()
            .json(info)
    }else{
        if req.size*10 > pagetotal[0]{
            let mut dat = HashMap::new();
            dat.insert("Total",pagetotal[0]);
            let info = dat;
            HttpResponse::Ok()
                .json(info)
        }else{
            let sql = format!("select * from address limit {:?}, 10", req.size*10);
            let res: Vec<(i32, String, String, String, String, String, String, String)> = conn.query(sql).unwrap();
            let mut info = HashMap::new();
            info.insert(pagetotal[0], res);
            HttpResponse::Ok()
                .json(info)
        }
    }
}