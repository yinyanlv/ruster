<template>
  <div  id="websocket">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">

<p>Actix支持WebSockets开箱即用。可以使用<a href="https://actix.rs/actix-web/actix_web/ws/struct.WsStream.html">ws :: WsStream</a>将请求的Payload转换为<a href="https://actix.rs/actix-web/actix_web/ws/enum.Message.html">ws :: Message</a>流，然后使用流组合器来处理实际的消息，但处理websocket通信使用http actor更简单。</p>
<p>以下是一个简单的websocket echo server的例子：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix::*;
<span class="hljs-keyword">use</span> actix_web::*;

<span class="hljs-comment">/// Define http actor</span>
<span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">Ws</span></span>;

<span class="hljs-keyword">impl</span> Actor <span class="hljs-keyword">for</span> Ws {
    <span class="hljs-class"><span class="hljs-keyword">type</span> <span class="hljs-title">Context</span></span> = ws::WebsocketContext&lt;<span class="hljs-keyword">Self</span>&gt;;
}

<span class="hljs-comment">/// Handler for ws::Message message</span>
<span class="hljs-keyword">impl</span> StreamHandler&lt;ws::Message, ws::ProtocolError&gt; <span class="hljs-keyword">for</span> Ws {

    <span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">handle</span></span>(&amp;<span class="hljs-keyword">mut</span> <span class="hljs-keyword">self</span>, msg: ws::Message, ctx: &amp;<span class="hljs-keyword">mut</span> Self::Context) {
        <span class="hljs-keyword">match</span> msg {
            ws::Message::Ping(msg) =&gt; ctx.pong(&amp;msg),
            ws::Message::Text(text) =&gt; ctx.text(text),
            ws::Message::Binary(bin) =&gt; ctx.binary(bin),
            _ =&gt; (),
        }
    }
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    App::new()
      .resource(<span class="hljs-string">"/ws/"</span>, |r| r.f(|req| ws::start(req, Ws)))
      .finish();
}
</div></code></pre>
<p><a href="https://github.com/actix/examples/tree/master/websocket/">websocket directory</a>提供了一个简单的websocket echo server示例 。</p>
<p><a href="https://github.com/actix/examples/tree/master/websocket-chat/">websocket-chat directory</a>提供了一个聊天服务器，可以通过websocket或tcp连接进行聊天</p>

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
  name: 'websocket',
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