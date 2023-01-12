use actix_web::{web, get, HttpRequest, HttpResponse};
use crate::api::other::{ips, gets};
use crate::data::information::{Addr, Info};

#[get("/ip")]
async fn ipnet(up: HttpRequest) -> HttpResponse{
    // ip地址查询
    // println!("{:?}", up.peer_addr().unwrap());
    let strs = format!("{}", up.peer_addr().unwrap());
    // for item in strs.split(":"){
    //	  println!("{:?}", item);
    // };
    let nums: Vec<&str> = strs.split(":").collect();

    let info = ips(nums[0]).await;
    if info == String::from("地址异常"){
        HttpResponse::Ok()
            .header("build", "SOFTHUB_HIKONL")
            .json(Info{
                data: "Where are you from?".to_owned()
            })
    }else{
        // 格式化请求数据
        let nrt: Addr = serde_json::from_str(&info).unwrap();
        HttpResponse::Ok()
            .header("build", "SOFTHUB_HIKONL")
            .json(nrt)
    }
}
