var path = require('path')

var config = {
    entry:{
        main:'./main'
    },
    output:{
        path:path.join(__dirname,'..','..','target','debug','front_end'),//打包后文件输出目录
        publicPath:'/front_end/',//资源引用目录
        filename:'main.js'//输出文件名称
    },
}


module.exports = config