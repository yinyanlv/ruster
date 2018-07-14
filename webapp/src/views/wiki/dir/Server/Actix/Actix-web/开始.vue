<template>
  <div  id="getting-started">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
                  <h1 id="%E5%BC%80%E5%A7%8B">开始</h1>
<p>我们来编写第一个actix web应用程序！</p>
<h2 id="hello-world">Hello, world!</h2>
<p>首先创建一个新的基于二进制的Cargo项目并进入新目录：</p>
<pre class="hljs"><code><div>cargo new hello-world
<span class="hljs-built_in">cd</span> hello-world
</div></code></pre>
<p>现在，确保actix-web的Cargo.toml 包含以下项目依赖关系：</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies]</span>
<span class="hljs-attr">actix-web</span> = <span class="hljs-string">"0.6"</span>
</div></code></pre>
<p>为了实现一个Web服务器，我们首先需要创建一个请求处理程序。请求处理函数接受一个HttpRequest实例作为其唯一参数，并返回一个可转换为HttpResponse的类型：</p>
<p>文件名: <code>src/main.rs</code></p>
<pre class="hljs"><code><div><span class="hljs-keyword">extern</span> <span class="hljs-keyword">crate</span> actix_web;
<span class="hljs-keyword">use</span> actix_web::{server, App, HttpRequest};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(_req: HttpRequest) -&gt; &amp;<span class="hljs-symbol">'static</span> <span class="hljs-built_in">str</span> {
    <span class="hljs-string">"Hello world!"</span>
}
</div></code></pre>
<p>接下来，创建一个Application实例，并将请求处理程序与应用程序的resource一起注册在特定HTTP方法和路径，然后，应用程序实例可以用于HttpServer来侦听将传入的连接。服务器接受一个应该返回一个HttpHandler实例的函数 。简单来说server::new可以使用了，它是HttpServer::new的简写：</p>
<pre class="hljs"><code><div><span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    server::new(|| App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(index)))
        .bind(<span class="hljs-string">"127.0.0.1:8088"</span>)
        .unwrap()
        .run();
}
</div></code></pre>
<p>仅此而已！现在编译并运行该程序cargo run。去http://localhost:8088 看结果。</p>
<p>如果你想要在开发过程中重新编译后自动重新加载服务器。请查看<a href="../theme/autoreload">自动重新加载模式</a>。</p>

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
  name: 'getting-started',
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