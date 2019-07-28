# 简易PASTEBIN服务

大致参考 <https://zhuanlan.zhihu.com/p/73961522>  这篇文章依样画葫芦。

加了点注释，强化了封装。把 `&str` 全换成了 `String`【这里偷懒了，会造成不小的内存开销】。

目前只有REST API。

## TODO

- [ ] 单元测试
- [ ] 完善 `logger`
- [ ] Web界面
- [ ] 管理员用的API



