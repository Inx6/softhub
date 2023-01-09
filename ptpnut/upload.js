const { workerData, parentPort } = require('worker_threads');
const fs = require("fs");

let data = workerData;

function save(n){
    let url = "/apks/" + n.originalname;
    let saveURL = __dirname + url; //文件名
    fs.readFile( n.path, function (err, data) {  // 异步读取文件内容
        fs.writeFile(saveURL, data, function (err) { // saveURL是文件名，data，文件数据，异步写入到文件
            if( err ){
                console.log( err );
            }else{
                // 文件上传成功，删除原文件
                fs.unlink( n.path, (err)=>{
                    if(err){
                        console.log( err )
                    }
                })
            }
       });
   });
}

parentPort.postMessage(save(data)); //向主线程返回结果