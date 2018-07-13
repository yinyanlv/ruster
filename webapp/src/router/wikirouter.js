import Cargo基础 from '../views/wiki/dir/Rust/Book/Cargo/Cargo基础'
import Cargo配置 from '../views/wiki/dir/Rust/Book/Cargo/Cargo配置'

export default function (router) {
    router.addRoutes([
        { path: '/a/wiki/rust/book/cargo/cargobase', name: 'Cargo基础', component: Cargo基础 },
        { path: '/a/wiki/rust/book/cargo/cargoset', name: 'Cargo基础', component: Cargo配置 }
    ])
}