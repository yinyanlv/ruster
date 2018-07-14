<template>
  <div  id="application">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
                  <h1 id="%E7%BC%96%E5%86%99%E5%BA%94%E7%94%A8">编写应用</h1>
<p>actix-web提供了用Rust构建Web服务器和应用程序的各种基础类型。它提供了路由，中间件，请求的预处理，响应的后处理，websocket协议处理，multipart流，等等。</p>
<br>
所有actix web服务器都是围绕该App实例构建的。它用于为资源和中间件注册路由。它还存储同一应用程序中所有处理程序之间共享的应用程序状态.
<br>
应用程序充当所有路由的命名空间，即特定应用程序的所有路由具有相同的url路径前缀。应用程序前缀总是包含一个前导的“/”斜杠。如果提供的前缀不包含前导斜杠，则会自动插入。前缀应该由路径值组成。
<br>
对于具有前缀的应用程序/app，与任何请求路径中有/app，/app/或/app/test匹配; 然而，路径/application不匹配。
<pre class="hljs"><code><div><span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(req: HttpRequest) -&gt; <span class="hljs-keyword">impl</span> Responder {
    <span class="hljs-string">"Hello world!"</span>
}

<span class="hljs-keyword">let</span> app = App::new()
    .prefix(<span class="hljs-string">"/app"</span>)
    .resource(<span class="hljs-string">"/index.html"</span>, |r| r.method(Method::GET).f(index))
    .finish()
</div></code></pre>
<p>在此示例中，将创建具有/app前缀和index.html资源的应用。该资源可通过/app/index.html路径获得。</p>
<blockquote>
<p>有关更多信息，请查看<a href="../advance/url-dispatch">URL Dispatch</a>部分。</p>
</blockquote>
<br>
但单服务器服务多个应用：
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> server = server::new(|| {
    <span class="hljs-built_in">vec!</span>[
        App::new()
            .prefix(<span class="hljs-string">"/app1"</span>)
            .resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>())),
        App::new()
            .prefix(<span class="hljs-string">"/app2"</span>)
            .resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>())),
        App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>())),
    ]
});
</div></code></pre>
<p>所有/app1请求路由到第一个应用程序，/app2到第二个，所有其他到第三个。 应用程序根据注册顺序进行匹配。如果具有更通用的前缀的应用程序在不通用的应用程序之前注册，它将有效地阻止较不通用的应用程序匹配。例如，如果App将前缀&quot;/&quot;注册为第一个应用程序，它将匹配所有传入的请求。</p>
<br>
<h2 id="%E7%8A%B6%E6%80%81">状态</h2>
<p>同一应用程序内应用程序状态被所有路由和资源共享。当使用http actor时，状态可以HttpRequest::state()作为只读访问，但内部可变性RefCell可用于实现状态可变性。状态也可用于路由匹配谓词和中间件。</p>
<p>我们来编写一个使用共享状态的简单应用程序。我们打算将请求计数存储在状态中：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{http, App, HttpRequest};
<span class="hljs-keyword">use</span> std::cell::Cell;

<span class="hljs-comment">// This struct represents state</span>
<span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">AppState</span></span> {
    counter: Cell&lt;<span class="hljs-built_in">usize</span>&gt;,
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(req: HttpRequest&lt;AppState&gt;) -&gt; <span class="hljs-built_in">String</span> {
    <span class="hljs-keyword">let</span> count = req.state().counter.get() + <span class="hljs-number">1</span>; <span class="hljs-comment">// &lt;- get count</span>
    req.state().counter.set(count); <span class="hljs-comment">// &lt;- store new count in state</span>

    <span class="hljs-built_in">format!</span>(<span class="hljs-string">"Request number: {}"</span>, count) <span class="hljs-comment">// &lt;- response with count</span>
}
</div></code></pre>
<p>应用程序需要通过初始化状态来初始化。</p>
<pre class="hljs"><code><div>App::with_state(AppState { counter: Cell::new(<span class="hljs-number">0</span>) })
    .resource(<span class="hljs-string">"/"</span>, |r| r.method(http::Method::GET).f(index))
    .finish()
</div></code></pre>
<blockquote>
<p><strong>注意</strong>：http服务器接受应用程序工厂而不是应用程序实例。Http服务器为每个线程构造一个应用程序实例，因此应用程序状态必须多次构建。如果你想在不同线程之间共享状态，应该使用共享对象，例如Arc。应用程序状态并不需要是Send和Sync，但应用程序的工厂必须是Send+ Sync。</p>
</blockquote>
<p>要启动以前的应用程序，请为其创建闭包：</p>
<pre class="hljs"><code><div>server::new(|| {
    App::with_state(AppState { counter: Cell::new(<span class="hljs-number">0</span>) })
        .resource(<span class="hljs-string">"/"</span>, |r| r.method(http::Method::GET).f(index))
}).bind(<span class="hljs-string">"127.0.0.1:8080"</span>)
    .unwrap()
    .run()
</div></code></pre>
<br>
<h2 id="%E7%BB%93%E5%90%88%E4%B8%8D%E5%90%8C%E7%8A%B6%E6%80%81%E7%9A%84%E5%BA%94%E7%94%A8%E7%A8%8B%E5%BA%8F">结合不同状态的应用程序</h2>
<p>将多个应用程序与不同状态组合也是可能的。</p>
<p><a href="https://docs.rs/actix-web/*/actix_web/server/fn.new.html">server::new</a>需要handler具有单一类型。</p>
<p>使用<a href="https://docs.rs/actix-web/*/actix_web/struct.App.html#method.boxed">App::boxed</a>方法可以轻松解决此限制，该方法可将App转换为boxed trait object。</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">State1</span></span>;
<span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">State2</span></span>;

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    server::new(|| {
        <span class="hljs-built_in">vec!</span>[
            App::with_state(State1)
                .prefix(<span class="hljs-string">"/app1"</span>)
                .resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>()))
                .boxed(),
            App::with_state(State2)
                .prefix(<span class="hljs-string">"/app2"</span>)
                .resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>()))
                .boxed(),
                ]
    }).bind(<span class="hljs-string">"127.0.0.1:8080"</span>)
        .unwrap()
        .run()
}
</div></code></pre>
<br>
<h2 id="%E4%BD%BF%E7%94%A8%E5%BA%94%E7%94%A8%E7%A8%8B%E5%BA%8F%E5%89%8D%E7%BC%80%E6%9D%A5%E7%BB%84%E5%90%88%E5%BA%94%E7%94%A8%E7%A8%8B%E5%BA%8F">使用应用程序前缀来组合应用程序</h2>
<p>该App::prefix()方法允许设置特定的应用程序前缀。此前缀表示将添加到所有资源模式的资源前缀通过资源配置。 这可以用来帮助挂载一组路由在不同的
地点比所包含的可调用作者的意图仍保持原样资源名称。</p>
<p>例如：</p>
<pre class="hljs"><code><div><span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">show_users</span></span>(req: HttpRequest) -&gt; HttpResponse {
    <span class="hljs-built_in">unimplemented!</span>()
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    App::new()
        .prefix(<span class="hljs-string">"/users"</span>)
        .resource(<span class="hljs-string">"/show"</span>, |r| r.f(show_users))
        .finish();
}
</div></code></pre>
<p>在上面的示例中，<code>show_users</code>路由将具有/users/show的有效路由模式， 而不是/ show，因为应用程序的前缀参数将预先添加到该模式。只有当URL路径为/users/show，并且HttpRequest.url_for()路由名称show_users调用该函数时，路由才会匹配，它将生成具有相同路径的URL。</p>
<br>
<h2 id="%E5%BA%94%E7%94%A8%E7%A8%8B%E5%BA%8F%E8%B0%93%E8%AF%8D%E5%92%8C%E8%99%9A%E6%8B%9F%E4%B8%BB%E6%9C%BA">应用程序谓词和虚拟主机</h2>
<p>您可以将谓词看作一个接受请求对象引用并返回true或false的简单函数。形式上，谓词是实现<a href="https://docs.rs/actix-web/0.6.10/actix_web/pred/trait.Predicate.html"><code>Predicate</code></a> trait的任何对象 。Actix提供了几个谓词，你可以检查 API文档的<a href="https://docs.rs/actix-web/0.6.10/actix_web/pred/index.html#functions">functions section</a>部分。</p>
<p>任何这些谓词都可以用于App::filter()方法。提供的谓词之一是Host，它可以根据请求的主机信息用作应用程序的过滤器。</p>
<pre class="hljs"><code><div><span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-keyword">let</span> server = server::new(|| {
        <span class="hljs-built_in">vec!</span>[
            App::new()
                .filter(pred::Host(<span class="hljs-string">"www.rust-lang.org"</span>))
                .resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>())),
            App::new()
                .filter(pred::Host(<span class="hljs-string">"users.rust-lang.org"</span>))
                .resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>())),
            App::new().resource(<span class="hljs-string">"/"</span>, |r| r.f(|r| HttpResponse::<span class="hljs-literal">Ok</span>())),
        ]
    });

    server.run();
}
</div></code></pre>
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
  name: 'application',
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