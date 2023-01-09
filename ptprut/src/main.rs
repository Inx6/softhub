use actix_web::{App, web, HttpServer};
use actix_web::middleware::Logger;
use colored::Colorize;
use std::fs::File;
use std::io::Read;

mod data;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let mut script = File::open(std::path::PathBuf::from("./config.yaml")).unwrap();
    let mut conten = String::new();
    script.read_to_string(&mut conten).unwrap();
    let i: data::information::Config = serde_yaml::from_str(&conten).unwrap();
    println!("\n{}:\n\n{:#?}\n","您的基本设置如下".blue(),i);

    use mysql::*;
    let pool = Pool::new(format!("mysql://{}:{}@localhost:3306/{}",i.db_name, i.db_pwd, i.db)).unwrap();
    println!("{}\n", "数据库链接成功！".green());
    
    println!("{}\n", "应用启动成功！".green());
    
    println!("{}\n", "全力以赴运行中！".red()); 

    println!("{}\n", format!("
                    _a88_                                 ___,
                   d88888baa__________            ____aad88888b
                  d888P''~~~~~~~~~~~~~~-----___ad88888888888888
              __-''                           'Y888888888888888b
             /                                  ~~Y8888P'''aaaa88a
            ,,,aaaaaaaaa,,,...__                   ~Yaaa8888888888
        __ d88888888888888888888888b                 8888888888888
      _- ,d88888P''''''~~~~~'''Y8888baa,             Y'''88888888P
     /  /~~~~~~                    '''Y88a...         888''''888P
    /  /      ,     |      |    |         |  `.       8888888aP
   |  |      |     | ||    |     |         |  `;      Y888888P      
  |  |     /|     |  | |    |    |          | ' ;      88888P         
  |  |    / |     |   | |    |    |         |          8888P     
 |  |    /  |    |    |  |    |    |         |         Y888     
|  |    |  |    /     |   |.   |_   |        |          Y8P         
| |    |  |  __|_      `. __|====__   |__----|          |'          
  |__-|__-| ~~~|           ~ ~~~  ~~,.  | | |    |      |          
        .| ~~~_____        _,aaaa._     | | |     |     |          
        | | ,''88b        ''Y'P:888ba   | | |     |      |       
        |  |   8o8           d8d888P~~  | | |     |      |      
        |  |   Y88b          888888     | |  |    |      |             
        |  |    888          888888     | |  |    |       |             
        |  |    888          Y88888     | |  |     |      |      
        |  |    Y8P         __8888Pa    | |  |     |      |     
       |   |   ~~~         ~~''Y88P''   | |  |     |       |       
       |   |        /                  |  |   |    |       |     
       |   |        |                  |  |   |     |      |      
       |   |                           |  |   |     |       |     
       |    |         -                |  |    |    |       |    
       |    | |                        |  |    |     |      |    
       |    ||  |                 __ --|  |    |     |       |  
       |    || |  |         __ --      |  |     |    |       |    
      |     || |     |__ -,            |  |     |     |      |    
      |    | |  |         |            |  ||     |    |       |   
      |    | |  |         |           ||  ||      |    |      |   
      |    ||   |        ,|           ||  ||       |    |     |   
     |     ||   |       a8|           ||  | |       |   |      | 
     |    | |   |     d88P|---8888'''| |  | |         |         
    |     ||   |     '''''''''''''888| |  || |
    |     ||   |    d8888888888888888| |  | |          --Hello!Sir    
    ").green());

    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(api::home::home)
            .service(api::upload::add)
            .service(api::img::img)
            .service(api::add_u::add_u)
            .service(api::login::login)
            .service(api::top_t::top)
            .service(api::top_t::sed)
            .service(api::search::search)
            .service(api::index::index)
    })
    .bind(format!("{}:{}",i.ip, i.port))?
    .run()
    .await
}
