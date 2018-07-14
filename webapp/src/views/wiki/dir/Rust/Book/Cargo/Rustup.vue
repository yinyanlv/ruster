<template>
  <div  id="rustup">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
<h1 id="rust-%E7%89%88%E6%9C%AC%E7%AE%A1%E7%90%86%E5%B7%A5%E5%85%B7-rustup">Rust 版本管理工具: rustup</h1>
<p>当然了，您也可以选择使用镜像。只需要设置两个环境变量就可以了：</p>
<p>RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup</p>
<p>rustup 是rust官方的版本管理工具。应当作为安装 Rust 的首选。</p>
<p>项目主页是: <a href="https://github.com/rust-lang-nursery/rustup.rs">https://github.com/rust-lang-nursery/rustup.rs</a></p>
<h2 id="features">Features</h2>
<ul>
<li>管理安装多个官方版本的 Rust 二进制程序。</li>
<li>配置基于目录的 Rust 工具链。</li>
<li>安装和更新来自 Rust 的发布通道: nightly, beta 和 stable。</li>
<li>接收来自发布通道更新的通知。</li>
<li>从官方安装历史版本的 nightly 工具链。</li>
<li>通过指定 stable 版本来安装。</li>
<li>安装额外的 std 用于交叉编译。</li>
<li>安装自定义的工具链。</li>
<li>独立每个安装的 Cargo metadata。</li>
<li>校验下载的 hash 值。</li>
<li>校验签名 (如果 GPG 存在)。</li>
<li>断点续传。</li>
<li>只依赖 bash, curl 和常见 unix 工具。</li>
<li>支持 Linux, OS X, Windows(via MSYS2)。</li>
</ul>
<h2 id="%E5%AE%89%E8%A3%85">安装</h2>
<h3 id="windows">Windows</h3>
<p>在<a href="http://www.rustup.rs">rustup的主页</a>下载并运行<a href="https://win.rustup.rs/">rustup-init.exe</a>,并按照提示选择选项。</p>
<p>在Windows下工具链会安装到<code>%USERPROFILE%\.cargo\bin</code>文件夹下并添加到<code>$PATH</code>环境变量。</p>
<h3 id="linux--osx">Linux &amp; OSX</h3>
<p>运行以下命令</p>
<pre><code>curl https://sh.rustup.rs -sSf | sh
</code></pre>
<p>这个命令将会编译和安装 rustup, 安装过程中可能会提示你输入 sudo 的密码。 然后, 他会下载和安装 stable 版本的工具链, 当执行 rustc, rustdoc 和 cargo 时, 将会配置他为默认工具链。</p>
<p><code>Unix</code> 上安装后工具链会被安装到 <code>$HOME/.cargo/bin</code>目录。</p>
<p><code>.cargo/bin</code>目录会被添加到系统的<code>$PATH</code>环境变量,重新登录后即可使用<code>rustc</code>，<code>cargo</code>等命令。</p>
<h2 id="%E5%8D%B8%E8%BD%BD">卸载</h2>
<pre><code>rustup self uninstall
</code></pre>
<h2 id="%E7%94%A8%E6%B3%95">用法</h2>
<p>安装后会得到一个 rustup 命令, 多使用命令自带的帮助提示, 可以快速定位你需要功能。</p>
<h3 id="%E5%B8%AE%E5%8A%A9">帮助</h3>
<p>运行 <code>rustup -h</code> 你将会得到如下提示:</p>
<pre><code>❯ rustup -h
rustup 0.1.12 (c6e430a 2016-05-12)
The Rust toolchain installer

USAGE:
    rustup [FLAGS] [SUBCOMMAND]

FLAGS:
    -v, --verbose    Enable verbose output
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    show         Show the active and installed toolchains
    update       Update Rust toolchains
    default      Set the default toolchain
    toolchain    Modify or query the installed toolchains
    target       Modify a toolchain's supported targets
    override     Modify directory toolchain overrides
    run          Run a command with an environment configured for a given toolchain
    which        Display which binary will be run for a given command
    doc          Open the documentation for the current toolchain.
    self         Modify the rustup installation
    telemetry    rustup telemetry commands
    help         Prints this message or the help of the given subcommand(s)

rustup installs The Rust Programming Language from the official
release channels, enabling you to easily switch between stable, beta,
and nightly compilers and keep them updated. It makes cross-compiling
simpler with binary builds of the standard library for common platforms.

If you are new to Rust consider running `rustup doc --book`
to learn Rust.

</code></pre>
<p>根据提示, 使用 <code>rust help &lt;command&gt;</code> 来查看子命令的帮助, 基本看这些帮助文档就足够了。</p>
<p>我们可以使用<code>rustup default &lt;toolchain&gt;</code> 配置默认的工具链。</p>
<p>其中标准的 <code>&lt;toolchain&gt;</code>具有如下的形式
<code>&lt;channel&gt;[-&lt;date&gt;][-&lt;host&gt;]</code></p>
<pre><code>channel  = stable|beta|nightly|version
date  = YYYY-MM-DD
host> = target-triple
</code></pre>
<p>如<code>stable-x86_64-pc-windows-msvc</code> <code>nightly-2014-12-18</code> <code>1.8.0</code>等都是合法的toolchain名称。</p>
<p>我们也可以采用<a href="https://github.com/rust-lang-nursery/rustup.rs#working-with-custom-toolchains">自定义toolchain</a>配合rustup。</p>
<p>下面着重介绍几个常用的命令。</p>
<p><code>rustup override add &lt;toolchain&gt;</code> 添加一个目录以及其子目录的默认工具链。</p>
<p><code>rustup override remove &lt;toolchain&gt;</code> 删除一个目录以及其子目录的默认工具链。</p>
<p><code>rustup show</code> 显示当前安装的工具链信息。</p>
<p><code>rustup toolchain install &lt;toolchain&gt;</code> 安装特定的的工具链。</p>
<p><code>rustup toolchain link &lt;toolchain-name&gt; &quot;&lt;toolchain-path&gt;&quot;</code> 设置自定义工具链。</p>
<p><code>rustup target add &lt;target&gt;</code> 设置工具链的可用目标。</p>
<p><code>rustup update</code> 检查安装工具链的更新。</p>
              </div>
          </div>
        </div>
      </main>
  </div>
</template>

<script>
/* eslint-disable */
import Wikiside from '../../../../../../components/wikiside/Wikiside'
export default {
  name: 'rustup',
  components: {
    "wikiside": Wikiside
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
a{
    color: #0541af;
}
#center {
  padding: 1rem;
  background-color: #FFFFFF;
}
main img {
        margin: 1rem auto;
        padding: 0.1rem;
        width: 100%;
}
main pre {
    display: block;
    padding: 8px;
    margin: 5px 0;
    font-size: 13.3px;
    line-height: 1.5;
    color: var(--purple);
    word-break: break-all;
    word-wrap: break-word;
    background-color: #f5f5f5;
    border: 1px solid rgb(246, 226, 252);
    text-shadow: none;
}

main code {
    padding: 2px 4px;
    font-size: 90%;
    background-color: #f5f5f5;
    border-radius: 4px;
    border: 1px solid #ccc;
    color: var(--purple);
    text-shadow: none;
}

main pre code {
    padding: 0;
    font-size: inherit;
    color: inherit;
    white-space: pre-wrap;
    background-color: transparent;
    border-radius: 0;
    border: 0;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 1vh auto;
        width: 97%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 70%;
        margin-left: 1vw;
    }
    #container #wikiside {
        flex: 1;
        background-color: #f1ebeb;
    }
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 75%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-left: 1vw;
    }
    #container #wikiside {
        flex: 1;
        background-color: #f1ebeb;
    }
}
</style>