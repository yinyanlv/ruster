import Cargo基础 from '../views/wiki/dir/Rust/Book/Cargo/Cargo基础'
import Cargo配置 from '../views/wiki/dir/Rust/Book/Cargo/Cargo配置'
import Rustup from '../views/wiki/dir/Rust/Book/Cargo/Rustup'
import 匹配模式 from '../views/wiki/dir/Rust/Book/Express/匹配-模式'
import 迭代器 from '../views/wiki/dir/Rust/Book/Express/迭代器'
import 表达式 from '../views/wiki/dir/Rust/Book/Express/表达式'

import 何为Actix from '../views/wiki/dir/Server/Actix/Actix-web/何为Actix'
import 安装 from '../views/wiki/dir/Server/Actix/Actix-web/安装'
import 开始 from '../views/wiki/dir/Server/Actix/Actix-web/开始'
import 应用 from '../views/wiki/dir/Server/Actix/Actix-web/应用'
import 服务器 from '../views/wiki/dir/Server/Actix/Actix-web/服务器'
import 静态文件 from '../views/wiki/dir/Server/Actix/Actix-web/静态文件'
import Websocket from '../views/wiki/dir/Server/Actix/Actix-web/Websocket'
import HTTP2 from '../views/wiki/dir/Server/Actix/Actix-web/HTTP2'
import 自动重加载 from '../views/wiki/dir/Server/Actix/Actix-web/自动重加载'
import 数据库 from '../views/wiki/dir/Server/Actix/Actix-web/数据库'

export default function (router) {
    router.addRoutes([
        { path: '/a/wiki/rust/book/cargo/cargobase', name: 'cargobase', component: Cargo基础 },
        { path: '/a/wiki/rust/book/cargo/cargoset', name: 'cargoset', component: Cargo配置 },
        { path: '/a/wiki/rust/book/cargo/rustup', name: 'rustup', component: Rustup },
        { path: '/a/wiki/rust/book/express/pattern', name: 'pattern', component: 匹配模式 },
        { path: '/a/wiki/rust/book/express/iterator', name: 'iterator', component: 迭代器 },
        { path: '/a/wiki/rust/book/express/express', name: 'express', component: 表达式 },

    



        { path: '/a/wiki/server/actix/actix-web/whatis', name: 'whatis', component: 何为Actix },
        { path: '/a/wiki/server/actix/actix-web/installation', name: 'installation', component: 安装 },
        { path: '/a/wiki/server/actix/actix-web/getting-starte', name: 'getting-starte', component: 开始 },
        { path: '/a/wiki/server/actix/actix-web/application', name: 'application', component: 应用 },
        { path: '/a/wiki/server/actix/actix-web/server', name: 'server', component: 服务器 },
        { path: '/a/wiki/server/actix/actix-web/staticfile', name: 'staticfile', component: 静态文件 },
        { path: '/a/wiki/server/actix/actix-web/websocket', name: 'websocket', component: Websocket },
        { path: '/a/wiki/server/actix/actix-web/http2', name: 'http2', component: HTTP2 },
        { path: '/a/wiki/server/actix/actix-web/autoreload', name: 'autoreload', component: 自动重加载 },
        { path: '/a/wiki/server/actix/actix-web/databases', name: 'databases', component: 数据库 }
    ])
}