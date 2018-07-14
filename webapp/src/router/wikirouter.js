import Cargo基础 from '../views/wiki/dir/Rust/Book/Cargo/Cargo基础'
import Cargo配置 from '../views/wiki/dir/Rust/Book/Cargo/Cargo配置'
import Whatis from '../views/wiki/dir/Server/Actix/Actix-web/Whatis'
import Installation from '../views/wiki/dir/Server/Actix/Actix-web/Installation'
import GettingStarte from '../views/wiki/dir/Server/Actix/Actix-web/GettingStarte'
import Application from '../views/wiki/dir/Server/Actix/Actix-web/Application'

export default function (router) {
    router.addRoutes([
        { path: '/a/wiki/rust/book/cargo/cargobase', name: 'cargobase', component: Cargo基础 },
        { path: '/a/wiki/rust/book/cargo/cargoset', name: 'cargoset', component: Cargo配置 },
        { path: '/a/wiki/server/actix/actix-web/whatis', name: 'whatis', component: Whatis },
        { path: '/a/wiki/server/actix/actix-web/installation', name: 'installation', component: Installation },
        { path: '/a/wiki/server/actix/actix-web/getting-starte', name: 'getting-starte', component: GettingStarte },
        { path: '/a/wiki/server/actix/actix-web/application', name: 'application', component: Application }
    ])
}