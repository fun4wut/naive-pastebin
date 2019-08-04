# 简易PASTEBIN服务

大致参考 <https://zhuanlan.zhihu.com/p/73961522>  这篇文章依样画葫芦。

加了点注释，强化了封装。把 Web Server 换成了 `Rocket` 。

Web界面已上线，欢迎访问 <https://paste.fun4go.top> 进行尝鲜。

## 截图

![创建paste](https://gofun4-pic.oss-cn-hangzhou.aliyuncs.com/pastebin-1.png)

![显示paste](https://gofun4-pic.oss-cn-hangzhou.aliyuncs.com/pastebin-2.png)

## 部署

```bash
$ cargo build -- release
$ ./target/release/naive-pastebin
```


### 环境变量

Prefix: `PASTEBIN_`

Shared Variable

| var           | default   | unit | description                     |
| ------------- | --------- | ---- | ------------------------------- |
| ADDR          | localhost |      | Binding address                 |
| PORT          | 8085      |      | Binding port                    |
| CRYPT_KEY     | fun4wut   |      | Crypto key for short url        |
| MAX_POST_SIZE | 32768     | byte | Max length of POST request body |
| DOMAIN        | $ADDR:$PORT |    | Domain Where you deploy

Built-in Memory Store

| var            | default   | unit        | description                                                 |
| -------------- | --------- | ----------- | ----------------------------------------------------------- |
| MAX_STORE_SIZE | 104857600 | byte        | An ambiguous size count for controlling server memory usage |
| MAX_EXPIRATION | 604800    | second      | Max expiration time                                         |
| CLEAN_DURATION | 5000      | millisecond | GC interval                                                 |



## API

### 存储

1. Request

    ```http request
    POST /api/save?<title>&[exp]
    ```
    > exp为过期时间，单位为秒，最长不超过7天，默认为7天
    
    ```typescript
    interface Request {
        content: string,
    }
    ```

2. Response

    ```typescript
    interface Response {
        code: number,
        msg: string,
        data: {
            key: string
        }
    }
    ```

### 查找

1. Request

    ```http request
    GET /api/find/<key>
    ```
    
2. Response
    ```typescript
    interface Response {
       code: number,
       msg: string,
       data: {
            title: string,
            lang: string,
            content: string,
            saving_time: number,
            expiration: number,
            view_count: number
       }
    }
    ```
    
### 直接返回 `raw`

```http request
GET /raw/<key>
```

## Benchmark

### find

```bash
$ ab -n 13000 -c 50 http://localhost:8085/api/save/xxxx
```

### save

```bash
$ ab -n 13000 -c 50 -p `json_file` -T "application/json" http://localhost:8085/api/save\?title\=233\&exp\=86400
```

## TODO

- [x] 单元测试
- [x] 健壮的Error机制
- [x] 考虑对命令行用户的友好度
- [x] Web界面
- [x] 以 `iframe` / `js脚本` 形式嵌入【参照 <https://pastebin.com/>】
- [x] 前端代码高亮
- [x] `raw` 字符串显示
- [x] 核心代码优化，减少 `clone` 造成的内存开销
- [ ] 过期时间可为无限
- [ ] 数据备份【 `snapshot` / `log` 】
- [ ] 冷数据存入硬盘
- [ ] 编辑/删除/回滚功能【使用一个密钥来验证，密钥只会在新建时出现，保存在浏览器 `localStorage` 中】
- [ ] 历史版本显示



