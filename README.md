# [Ruster](http://ruster.xyz/) [![Build Status](https://travis-ci.org/ruster-xyz/ruster.svg?branch=master)](https://travis-ci.org/ruster-xyz/ruster)
online community in rust for rust community

Ruster is single page webapp written in [actix-web](https://github.com/actix/actix-web) with vuejs.
- Async stable Actix-web framework 
- diesel, postgresql r2d2
- SPA CORS JWT
- Vuejs 

## How To
    first create a name 'ruster' postgresql database for this project.

## when development/开发
```bash
$ git clone https://github.com/ruster-xyz/ruster.git
$ cd ruster
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run

// another shell nodejs(v10.1.0 on my machine)

$ cd ruster/webapp
$ npm install
$ npm run serve
```
then open browser 'http://localhost:8080'

## when production/生产

```bash
$ git clone https://github.com/ruster-xyz/ruster.git
$ cd ruster
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cd webapp
$ npm install
$ npm run build
$ cd ..
$ cargo run --release
```
then open broswer 'http://localhost:8000/'

## Support Ruster/支持Ruster
Ruster is community project for community, It's need money for Cloud server to support the [Ruster](http://ruster.xyz/). If you want let the project have a bright future and can help the project, please gave your hand thanks. at this moment you can contact [krircc](https://github.com/krircc) or krircc@aliyun.com


## License
[GPL-3](https://github.com/ruster-xyz/ruster/blob/master/LICENSE)

Copyright (c) 2018-present, Xiangfei Wang
