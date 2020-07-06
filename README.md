# fgit

[fastgit](https://fastgit.org/) 是一个适用于 GitHub 的加速服务，fgit 是为方便使用 fastgit 而开发的命令行工具。

对于本身拥有代理服务的用户，我们推荐您使用 git 自身的代理功能。

在 `~/.gitconfig` 中加入如下配置：

```ini
[http]
    proxy = http://127.0.0.1:1234
[https]
    proxy = http://127.0.0.1:1234
; 1234 为代理软件提供的端口号
```

## 功能

### clone

`fgit clone <url> <dir>`

`<url>` 必须存在，其应该为一个 GitHub 的 repo 链接（https/ssh 协议均可）。

`<dir>` 是可选项，即指定 clone 到本地时的文件夹名称。

fgit 首先会把 github 的链接替换为 fasthub 的链接，然后调用 `git clone` 命令，最后将 `remote "origin"` 的 url 再替换为 github 的链接。

### pull

`fgit pull`

执行此命令时，fgit 会将 `remote "origin"` 的 url 替换为 fasthub 的，然后调用 `git pull` 命令，最后再将其改回原 url。

### dl

`fgit dl <url>`

fgit 将支持的 url 替换为 fastgit 提供的镜像，然后下载。

支持的链接可见 [fastgit 的文档](https://doc.fastgit.org/zh-cn/guide.html) 。

## LICENSE

使用 fgit 时，我们自动认为您同意了 [fastgit 的服务条款](https://doc.fastgit.org/zh-cn/tos.html) 。

如果您对该条款持有不同意见，请勿使用本程序。

fgit 的源代码以 MIT LICENSE 开源

所涉及的第三方库或源代码：

* [rsget](https://github.com/otavio/rsget), [Apache License 2.0](https://github.com/otavio/rsget/blob/master/LICENSE)
* [rust-ini](https://github.com/zonyitoo/rust-ini), [MIT License](https://github.com/zonyitoo/rust-ini/blob/master/LICENSE)
* [clap](https://github.com/clap-rs/clap), [MIT License](https://github.com/clap-rs/clap/blob/master/LICENSE-MIT), [Apache License 2.0](https://github.com/clap-rs/clap/blob/master/LICENSE-APACHE)
* [regex](https://github.com/rust-lang/regex), [MIT License](https://github.com/rust-lang/regex/blob/master/LICENSE-MIT), [Apache License 2.0](https://github.com/rust-lang/regex/blob/master/LICENSE-APACHE)


## to-do && 已知问题

fgit 仍处于早期阶段，请审慎使用。

* [ ] 对 `git clone` 的所有命令行选项提供支持
* [x] ~~由于 fastgit 的反代 headers 中不存在 `content-length` ，下载功能可能会受到影响~~已由上游修复
* [ ] `--verbose` flag
* [ ] 完善注释
