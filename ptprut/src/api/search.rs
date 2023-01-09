use actix_web::{web, get, HttpRequest, HttpResponse};
use crate::data::information::Info;

// 模糊查询
#[get("/search")]
async fn search(req: web::Query<Info>, db: web::Data<mysql::Pool>) -> HttpResponse{
    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();
    
    let sql = "select * from address where word like '%".to_owned() + &req.data + "%'";
    let info: Vec<(i32, String, String, String, String, String, String, String)> = conn.query(sql).unwrap();

    // println!("{:?}", info.get_mut(0).unwrap());
    HttpResponse::Ok()
        .json(info)
}