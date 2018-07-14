<template>
  <div  id="http2">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">

<p>如果可能<code>actix-web</code>自动升级到<em>HTTP/2.0</em>的连接。</p>
<h1 id="%E5%8D%8F%E8%AE%AE">协议</h1>
<p><em>HTTP/2.0</em> protocol over tls  without prior knowledge requires <a href="https://tools.ietf.org/html/rfc7301">tls alpn</a>.</p>
<blockquote>
<p>目前，只有<code>rust-openssl</code>支持</p>
</blockquote>
<p><code>alpn</code>协议需要启用该功能。启用后，HttpServer提供 serve_tls方法。
<a href="https://actix.rs/actix-web/actix_web/server/struct.HttpServer.html#method.serve_tls">serve_tls</a> method.</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies]</span>
<span class="hljs-attr">actix-web</span> = { version = <span class="hljs-string">"0.6"</span>, features = [<span class="hljs-string">"alpn"</span>] }
<span class="hljs-attr">openssl</span> = { version = <span class="hljs-string">"0.10"</span>, features = [<span class="hljs-string">"v110"</span>] }
</div></code></pre>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> std::fs::File;
<span class="hljs-keyword">use</span> actix_web::*;
<span class="hljs-keyword">use</span> openssl::ssl::{SslMethod, SslAcceptor, SslFiletype};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-comment">// load ssl keys</span>
    <span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(<span class="hljs-string">"key.pem"</span>, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(<span class="hljs-string">"cert.pem"</span>).unwrap();

    HttpServer::new(
        || App::new()
            .resource(<span class="hljs-string">"/index.html"</span>, |r| r.f(index)))
        .bind(<span class="hljs-string">"127.0.0.1:8080"</span>).unwrap();
        .serve_ssl(builder).unwrap();
}
</div></code></pre>
<p>不支持升级到<a href="https://http2.github.io/http2-spec/#rfc.section.3.2">rfc section 3.2</a> 节中描述的HTTP/2.0模式 。明文连接和tls连接都支持<em>HTTP/2</em> with prior knowledge启动,<a href="https://http2.github.io/http2-spec/#rfc.section.3.4">rfc section 3.4</a></p>
<p>查看具体示例<a href="https://github.com/actix/examples/tree/master/tls">examples/tls</a>.</p>

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
  name: 'http2',
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