const mysql = require("mysql");

let db = mysql.createPool({
    host: 'localhost',      //数据库地址
    port: '3306',           //端口号
    user: 'root',           //用户名
    password: 'admin123',   //密码
    database: 'list'        //数据库名称
});

module.exports = db;