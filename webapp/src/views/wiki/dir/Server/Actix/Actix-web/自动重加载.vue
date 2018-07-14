<template>
  <div  id="autoreload">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">

<h1 id="%E8%87%AA%E5%8A%A8%E9%87%8D%E6%96%B0%E5%8A%A0%E8%BD%BD%E5%BC%80%E5%8F%91%E6%9C%8D%E5%8A%A1%E5%99%A8">自动重新加载开发服务器</h1>
<p>在开发过程中，cargo自动重新编译变更代码会非常方便。这可以通过使用<a href="https://github.com/passcod/cargo-watch">cargo-watch</a>来完成 。由于actix应用程序通常会绑定到端口以侦听传入的HTTP请求，因此将它与<a href="https://crates.io/crates/listenfd">listenfd</a>和<a href="https://github.com/mitsuhiko/systemfd">systemfd</a>实用程序结合起来以确保套接字在应用程序编译和重新加载时保持打开状态是有意义的。</p>
<p><code>systemfd</code>将打开一个套接字并将其传递给<code>cargo-watch</code>以监视更改，然后调用编译器并运行您的actix应用程序。actix应用程序将使用<code>listenfd</code>获取 <code>systemfd</code>打开的套接字systemfd。</p>
<h2 id="%E9%9C%80%E8%A6%81%E7%9A%84%E4%BA%8C%E8%BF%9B%E5%88%B6%E6%96%87%E4%BB%B6">需要的二进制文件</h2>
<p>对于自动重新加载体验，您需要安装<code>cargo-watch</code>和 <code>systemfd</code>。两者都用<code>cargo install</code>安装</p>
<pre><code>cargo install systemfd cargo-watch
</code></pre>
<h2 id="%E4%BF%AE%E6%94%B9%E4%BB%A3%E7%A0%81">修改代码</h2>
<p>此外，您需要稍微修改您的actix应用程序，以便它可以获取由<code>systemfd</code>打开的外部套接字。将<code>listenfd</code>添加到您的应用依赖项中：</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencices]</span>
<span class="hljs-attr">listenfd</span> = <span class="hljs-string">"0.3"</span>
</div></code></pre>
<p>然后修改您的服务器代码以仅以<code>bind</code>作为回调：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">extern</span> <span class="hljs-keyword">crate</span> listenfd;

<span class="hljs-keyword">use</span> listenfd::ListenFd;
<span class="hljs-keyword">use</span> actix_web::{server, App, HttpRequest, Responder};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(_req: HttpRequest) -&gt; <span class="hljs-keyword">impl</span> Responder {
    <span class="hljs-string">"Hello World!"</span>
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> listenfd = ListenFd::from_env();
    <span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> server = server::new(|| {
        App::new()
            .resource(<span class="hljs-string">"/"</span>, |r| r.f(index))
    });

    server = <span class="hljs-keyword">if</span> <span class="hljs-keyword">let</span> <span class="hljs-literal">Some</span>(l) = listenfd.take_tcp_listener(<span class="hljs-number">0</span>).unwrap() {
        server.listen(l)
    } <span class="hljs-keyword">else</span> {
        server.bind(<span class="hljs-string">"127.0.0.1:3000"</span>).unwrap()
    };

    server.run();
}
</div></code></pre>
<h2 id="%E8%BF%90%E8%A1%8C%E6%9C%8D%E5%8A%A1%E5%99%A8">运行服务器</h2>
<p>现在运行开发服务器调用这个命令：</p>
<pre><code>systemfd --no-pid -s http::3000 -- cargo watch -x run
</code></pre>
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
  name: 'autoreload',
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