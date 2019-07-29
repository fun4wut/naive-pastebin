# 简易PASTEBIN服务

大致参考 <https://zhuanlan.zhihu.com/p/73961522>  这篇文章依样画葫芦。

加了点注释，强化了封装。把 Web Server 换成了 `Rocket` 。

目前只有REST API。

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
    POST /api/save
    ```
    
    ```typescript
    interface Request {
        title: string,
        lang: string,
        content: string,
        expiration: number, // 秒数
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

## TODO

- [x] 单元测试
- [x] 健壮的Error机制
- [ ] Web界面
- [ ] 以 `iframe` / `js脚本` 形式嵌入【参照 <https://pastebin.com/>】
- [ ] `raw` 字符串显示
- [ ] 历史版本显示
- [ ] 代码高亮
- [ ] 数据备份
- [ ] 编辑/删除功能【使用一个密钥来验证，密钥只会在新建时出现，保存在浏览器 `localStorage` 中】



