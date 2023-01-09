use actix_web::{web, get, HttpRequest, HttpResponse};
use crate::data::information::Tonge;
use std::collections::HashMap;

// 一级查询
#[get("/top")]
async fn top(req: web::Query<Tonge>, db: web::Data<mysql::Pool>) -> HttpResponse{
    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();
    
    let pagetotal: Vec<i32> = conn.query(format!("SELECT COUNT(*) FROM address where type = {:?}", req.data)).unwrap();
    if req.size == 0{
        let sql = format!("select * from address where type = {:?} limit 0, 10", req.data);
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
            let sql = format!("select * from address where type = {:?} limit {:?}, 10", req.data, req.size*10);
            let res: Vec<(i32, String, String, String, String, String, String, String)> = conn.query(sql).unwrap();
            let mut info = HashMap::new();
            info.insert(pagetotal[0], res);
            HttpResponse::Ok()
                .json(info)
        }
    }
}

// 二级查询
#[get("/sed")]
async fn sed(req: web::Query<Tonge>, db: web::Data<mysql::Pool>) -> HttpResponse{
    use mysql::*;
    use mysql::prelude::Queryable;
    // 获取连接
    let mut conn = db.get_conn().unwrap();

    let pagetotal: Vec<i32> = conn.query(format!("SELECT COUNT(*) FROM address where label = {:?}", req.data)).unwrap();
    if req.size == 0{
        let sql = format!("select * from address where label = {:?} limit 0, 10", req.data);
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
            let sql = format!("select * from address where label = {:?} limit {:?}, 10", req.data, req.size*10);
            let res: Vec<(i32, String, String, String, String, String, String, String)> = conn.query(sql).unwrap();
            let mut info = HashMap::new();
            info.insert(pagetotal[0], res);
            HttpResponse::Ok()
                .json(info)
        }
    }
}

