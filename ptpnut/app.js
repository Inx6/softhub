const {Worker} = require("worker_threads");
const express = require("express");
const multer = require('multer');
const bodyParser = require('body-parser');
const db = require("./db.js");

// 本机外网ip,port地址
const ip = "127.0.0.1";
const port = "251";
const app = express();

app.use(bodyParser.json({limit: '500mb'}));
app.use(bodyParser.urlencoded({extended: false, limit: '500mb'}));
app.use(multer({ dest: './tmp/'}).array('file'));

app.post("/",async (req,res)=>{
    await data(req.files[0]);
    switch (req.body.type){
        case "apk":
            let sql0 = `select url from pubapi where url = 'http://${ip}:${port}/apks/${req.files[0].originalname}'`;
            db.query(sql0, (err, result)=>{
                if(err){
                    console.log(err)
                }else if(result.length !== 0){
                    res.send("上传失败,文件库里面已经有你的上传地址了,改下文件名试试？(°ー°〃) ")
                }else{
                    let sql1 = `insert into pubapi set uuid = '${req.body.name}', url = 'http://${ip}:${port}/apks/${req.files[0].originalname}'`;
                    db.query(sql1,(errs, results)=>{
                        if(errs){
                            console.log(errs)
                        }else if(results.affectedRows === 1){
                            res.send({
                                "上传成功啦！地址为": `http://${ip}:${port}/apks/${req.files[0].originalname}`
                            })
                        }else{
                            res.send("不晓得什么原因，上传失败啦(°ー°〃) ")
                        }  
                    });
                }
            });
            break;
            case "img":
                let sql3 = `select url from pubapi where url = 'http://${ip}:${port}/apks/${req.files[0].originalname}'`;
                db.query(sql3, (err, result)=>{
                    if(err){
                        console.log(err)
                    }else if(result.length !== 0){
                        res.send({
                            data: "上传失败,文件库里面已经有你的上传地址了,改下文件名试试？(°ー°〃) ",
                            url: `http://${ip}:${port}/apks/${req.files[0].originalname}`
                        })
                    }else{
                        let sql4 = `insert into pubapi set uuid = '${req.body.name}', url = 'http://${ip}:${port}/apks/${req.files[0].originalname}'`;
                        db.query(sql4,(errs, results)=>{
                            if(errs){
                                console.log(errs)
                            }else if(results.affectedRows === 1){
                                res.send({
                                    data: "上传成功啦！",
                                    url: `http://${ip}:${port}/apks/${req.files[0].originalname}`
                                })
                            }else{
                                res.send("不晓得什么原因，上传失败啦(°ー°〃) ")
                            }  
                        });
                    }
                });
                break;
                default:
                    res.send("ERROR!你上传的文件是什么牛马类型?我们只需要apk和img!")
                    break;
    }
})

app.use("/apks",express.static(__dirname+"/apks"));

async function data(n){
    let worker = new Worker("./upload.js", { workerData: n });
    return new Promise((res)=>{
        worker.on('message', (val)=>{
            res(val); //接收工作线程计算完毕后返回的结果
        })
    })
}

app.listen(`${port}`, ()=>{
    console.log(`服务器启动成功啦！来自: ${ip}:${port}`)
})