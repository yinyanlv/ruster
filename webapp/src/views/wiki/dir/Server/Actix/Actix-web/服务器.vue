<template>
  <div  id="server">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">

<h1 id="http%E6%9C%8D%E5%8A%A1%E5%99%A8">HTTP服务器</h1>
<p>该<a href="../../actix-web/actix_web/server/struct.HttpServer.html"><strong>HttpServer</strong></a>类型负责服务的HTTP请求。</p>
<p><code>HttpServer</code>接受应用程序工厂作为参数，并且应用程序工厂必须具有Send+ Sync边界。p
要绑定到特定的套接字地址， <a href="../../actix-web/actix_web/server/struct.HttpServer.html#method.bind"><code>bind()</code></a> 必须使用，并且可能会多次调用它。绑定ssl套接字使用<a href="../../actix-web/actix_web/server/struct.HttpServer.html#method.bind_ssl"><code>bind_ssl()</code></a>或<a href="../../actix-web/actix_web/server/struct.HttpServer.html#method.bind_tls"><code>bind_tls()</code></a>。启动http服务器，启动方法之一是：</p>
<ul>
<li>use <a href="https://actix.rs/actix-web/actix_web/server/struct.HttpServer.html#method.start"><code>start()</code></a>
for a server</li>
</ul>
<p><code>HttpServer</code>是一位actix actor。它必须在正确配置的actix系统中初始化：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{server::HttpServer, App, HttpResponse};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-keyword">let</span> sys = actix::System::new(<span class="hljs-string">"guide"</span>);

    HttpServer::new(|| App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(|_| HttpResponse::<span class="hljs-literal">Ok</span>())))
        .bind(<span class="hljs-string">"127.0.0.1:59080"</span>)
        .unwrap()
        .start();

    <span class="hljs-keyword">let</span> _ = sys.run();
}
</div></code></pre>
<blockquote>
<p>可以使用该run()方法在单独的线程中启动服务器。在这种情况下，服务器会产生一个新线程并在其中创建一个新的actix系统。要停止此服务器，请发送<code>StopServer</code>消息。</p>
</blockquote>
<p><code>HttpServer</code>被实施为actix actor。可以通过消息传递系统与服务器进行通信。启动方法，例如<code>start()</code>，返回启动的http服务器的地址。它接受几种消息：</p>
<ul>
<li>PauseServer - 暂停接受传入连接</li>
<li>ResumeServer - 继续接受传入连接</li>
<li>StopServer - 停止传入连接处理，停止所有workers并退出</li>
</ul>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{server, App, HttpResponse};
<span class="hljs-keyword">use</span> std::sync::mpsc;
<span class="hljs-keyword">use</span> std::thread;

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-keyword">let</span> (tx, rx) = mpsc::channel();

    thread::spawn(<span class="hljs-keyword">move</span> || {
        <span class="hljs-keyword">let</span> sys = actix::System::new(<span class="hljs-string">"http-server"</span>);
        <span class="hljs-keyword">let</span> addr = server::new(|| {
            App::new()
                .resource(<span class="hljs-string">"/"</span>, |r| r.f(|_| HttpResponse::<span class="hljs-literal">Ok</span>()))
        })
            .bind(<span class="hljs-string">"127.0.0.1:0"</span>).expect(<span class="hljs-string">"Can not bind to 127.0.0.1:0"</span>)
            .shutdown_timeout(<span class="hljs-number">60</span>)    <span class="hljs-comment">// &lt;- Set shutdown timeout to 60 seconds</span>
            .start();
        <span class="hljs-keyword">let</span> _ = tx.send(addr);
        <span class="hljs-keyword">let</span> _ = sys.run();
    });

    <span class="hljs-keyword">let</span> addr = rx.recv().unwrap();
    <span class="hljs-keyword">let</span> _ = addr.send(server::StopServer { graceful: <span class="hljs-literal">true</span> }).wait(); <span class="hljs-comment">// &lt;- Send `StopServer` message to server.</span>
}
</div></code></pre>
<br>
<h2 id="%E5%A4%9A%E7%BA%BF%E7%A8%8B">多线程</h2>
<p><code>HttpServer</code>自动启动一些http worker，默认情况下这个数量等于系统中逻辑CPU的数量。该数量可以用该<a href="../../actix-web/actix_web/server/struct.HttpServer.html#method.workers"><code>HttpServer::workers()</code></a>方法覆盖 。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{server::HttpServer, App, HttpResponse};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    HttpServer::new(|| App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(|_| HttpResponse::<span class="hljs-literal">Ok</span>())))
        .workers(<span class="hljs-number">4</span>); <span class="hljs-comment">// &lt;- Start 4 workers</span>
}
</div></code></pre>
<p>服务器为每个创建的worker创建一个单独的应用实例。应用程序状态不在线程之间共享。分享状态，可以使用Arc。</p>
<blockquote>
<p>应用程序状态并不需要是Send和Sync，但是工厂必须是Send+ Sync。</p>
</blockquote>
<h2 id="ssl">SSL</h2>
<p>有两种功能的ssl服务器：<code>tls</code>和<code>alpn</code>。该tls功能由native-tls集成，alpn由openssl。</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies]</span>
<span class="hljs-attr">actix-web</span> = { version = <span class="hljs-string">"0.6"</span>, features = [<span class="hljs-string">"alpn"</span>] }
</div></code></pre>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{server, App, HttpRequest, Responder};
<span class="hljs-keyword">use</span> openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(req: HttpRequest) -&gt; <span class="hljs-keyword">impl</span> Responder {
    <span class="hljs-string">"Welcome!"</span>
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-comment">// load ssl keys</span>
    <span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(<span class="hljs-string">"key.pem"</span>, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(<span class="hljs-string">"cert.pem"</span>).unwrap();

    server::new(|| App::new().resource(<span class="hljs-string">"/index.html"</span>, |r| r.f(index)))
        .bind_ssl(<span class="hljs-string">"127.0.0.1:8080"</span>, builder)
        .unwrap()
        .run();
}
</div></code></pre>
<blockquote>
<p><strong>注意</strong>：HTTP / 2.0协议需要<a href="https://tools.ietf.org/html/rfc7301">tls alpn</a>。目前，只有openssl有alpn支持。完整示例，请查看<a href="https://github.com/actix/examples/tree/master/tls">examples/tls</a>.</p>
</blockquote>
<p>要创建key.pem和cert.pem，请使用以下命令。<strong>Fill in your own subject</strong></p>
<pre class="hljs"><code><div>$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
  -days 365 -sha256 -subj <span class="hljs-string">"/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"</span>
</div></code></pre>
<p>要删除密码，请将nopass.pem复制到key.pem</p>
<pre class="hljs"><code><div>$ openssl rsa -<span class="hljs-keyword">in</span> key.pem -out nopass.pem
</div></code></pre>
<br>
<h2 id="keep-alive">Keep-Alive</h2>
<p>Actix可以等待keep-alive的请求。</p>
<blockquote>
<p><em>keep-alive</em>连接行为由服务器设置定义。</p>
</blockquote>
<ul>
<li><code>75</code>, <code>Some(75)</code>, <code>KeepAlive::Timeout(75)</code> - 75秒keep alive定时器。</li>
<li><code>None</code> or <code>KeepAlive::Disabled</code> - 禁用 <em>keep alive</em>.</li>
<li><code>KeepAlive::Tcp(75)</code> -  使用 <code>SO_KEEPALIVE</code> socket 选项.</li>
</ul>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{server, App, HttpResponse};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    server::new(|| App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(|_| HttpResponse::<span class="hljs-literal">Ok</span>())))
        .keep_alive(<span class="hljs-number">75</span>); <span class="hljs-comment">// &lt;- Set keep-alive to 75 seconds</span>

    server::new(|| App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(|_| HttpResponse::<span class="hljs-literal">Ok</span>())))
        .keep_alive(server::KeepAlive::Tcp(<span class="hljs-number">75</span>)); <span class="hljs-comment">// &lt;- Use `SO_KEEPALIVE` socket option.</span>

    server::new(|| App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(|_| HttpResponse::<span class="hljs-literal">Ok</span>())))
        .keep_alive(<span class="hljs-literal">None</span>); <span class="hljs-comment">// &lt;- Disable keep-alive</span>
}
</div></code></pre>
<p>如果选择第一个选项，则<em>keep alive</em>状态根据响应的<em>connection-type</em>计算。默认情况下<code>HttpResponse::connection_type</code>未定义。在这种情况下， <em>keep alive</em> 状态由请求的http版本定义。</p>
<blockquote>
<p><em>keep alive</em> 是 <strong>关闭</strong> 对于 <em>HTTP/1.0</em> 然而是 <strong>打开</strong> 对于 <em>HTTP/1.1</em> 和 <em>HTTP/2.0</em>.</p>
</blockquote>
<p><em>Connection type</em>可以用<code>HttpResponseBuilder::connection_type()</code>方法改变。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{http, HttpRequest, HttpResponse};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(req: HttpRequest) -&gt; HttpResponse {
    HttpResponse::<span class="hljs-literal">Ok</span>()
        .connection_type(http::ConnectionType::Close) <span class="hljs-comment">// &lt;- Close connection</span>
        .force_close()                                <span class="hljs-comment">// &lt;- Alternative method</span>
        .finish()
}
</div></code></pre>
<br>
<h2 id="%E4%BC%98%E9%9B%85%E7%9A%84%E5%85%B3%E6%9C%BA">优雅的关机</h2>
<p><code>HttpServer</code>支持优雅的关机。收到停止信号后，workers会有特定的时间完成服务请求。任何在超时后仍然活着的workers（工作线程）都会被迫停止。默认情况下，关机超时设置为30秒。您可以使用<a href="../../actix-web/actix_web/server/struct.HttpServer.html#method.shutdown_timeout"><code>HttpServer::shutdown_timeout()</code></a>方法更改此参数 。</p>
<p>您可以使用服务器地址向服务器发送停止消息，并指定是否要进行正常关机。<a href="../../actix-web/actix_web/server/struct.HttpServer.html#method.start"><code>start()</code></a>方法返回服务器的地址。</p>
<p><code>HttpServer</code>处理几个OS信号。所有操作系统都提供CTRL-C，其他信号在unix系统上可用。</p>
<ul>
<li><em>SIGINT</em> - 强制关闭工作线程</li>
<li><em>SIGTERM</em> - 优雅的停止工作线程</li>
<li><em>SIGQUIT</em> - 制关闭 workers工作线程</li>
</ul>
<blockquote>
<p>可以用<a href="../../actix-web/actix_web/server/struct.HttpServer.html#method.disable_signals"><code>HttpServer::disable_signals()</code></a>
方法禁用信号处理 。</p>
</blockquote>
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
  name: 'server',
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