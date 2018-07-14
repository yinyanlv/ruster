<template>
  <div  id="databases">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">

<h1 id="diesel">Diesel</h1>
<p>目前，Diesel 1.0不支持异步操作，但可以将actix同步actor系统用作数据库接口API。从技术上讲，同步actor是worker风格的actor。
多个同步actors可以并行运行并处理来自同一队列的消息。同步actors以mpsc模式工作。</p>
<p>我们来创建一个简单的数据库api，它可以将一个新的 user row插入到SQLite表中。我们必须定义一个同步actor和该actor将使用的连接。其他数据库可以使用相同的方法。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix::prelude::*;

<span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">DbExecutor</span></span>(SqliteConnection);

<span class="hljs-keyword">impl</span> Actor <span class="hljs-keyword">for</span> DbExecutor {
    <span class="hljs-class"><span class="hljs-keyword">type</span> <span class="hljs-title">Context</span></span> = SyncContext&lt;<span class="hljs-keyword">Self</span>&gt;;
}
</div></code></pre>
<p>这是我们actor的定义。现在，我们必须定义创建用户消息和响应。</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">CreateUser</span></span> {
    name: <span class="hljs-built_in">String</span>,
}

<span class="hljs-keyword">impl</span> Message <span class="hljs-keyword">for</span> CreateUser {
    <span class="hljs-class"><span class="hljs-keyword">type</span> <span class="hljs-title">Result</span></span> = <span class="hljs-built_in">Result</span>&lt;User, Error&gt;;
}
</div></code></pre>
<p>我们可以向演员发送<code>CreateUser</code>消息<code>DbExecutor</code> actor，因此我们将收到一个 <code>User</code>实例。接下来，我们必须为此消息定义处理程序实现。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">impl</span> Handler&lt;CreateUser&gt; <span class="hljs-keyword">for</span> DbExecutor {
    <span class="hljs-class"><span class="hljs-keyword">type</span> <span class="hljs-title">Result</span></span> = <span class="hljs-built_in">Result</span>&lt;User, Error&gt;;

    <span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">handle</span></span>(&amp;<span class="hljs-keyword">mut</span> <span class="hljs-keyword">self</span>, msg: CreateUser, _: &amp;<span class="hljs-keyword">mut</span> Self::Context) -&gt; Self::<span class="hljs-built_in">Result</span>
    {
        <span class="hljs-keyword">use</span> self::schema::users::dsl::*;

        <span class="hljs-comment">// Create insertion model</span>
        <span class="hljs-keyword">let</span> uuid = <span class="hljs-built_in">format!</span>(<span class="hljs-string">"{}"</span>, uuid::Uuid::new_v4());
        <span class="hljs-keyword">let</span> new_user = models::NewUser {
            id: &amp;uuid,
            name: &amp;msg.name,
        };

        <span class="hljs-comment">// normal diesel operations</span>
        diesel::insert_into(users)
            .values(&amp;new_user)
            .execute(&amp;<span class="hljs-keyword">self</span>.<span class="hljs-number">0</span>)
            .expect(<span class="hljs-string">"Error inserting person"</span>);

        <span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> items = users
            .filter(id.eq(&amp;uuid))
            .load::&lt;models::User&gt;(&amp;<span class="hljs-keyword">self</span>.<span class="hljs-number">0</span>)
            .expect(<span class="hljs-string">"Error loading person"</span>);

        <span class="hljs-literal">Ok</span>(items.pop().unwrap())
    }
}
</div></code></pre>
<p>仅此而已！现在，我们可以使用来在于任何http处理程序或中间件的DbExecutor actor。我们需要的只是启动DbExecutor actors并将地址存储在http处理程序可以访问的状态中。</p>
<pre class="hljs"><code><div><span class="hljs-comment">/// This is state where we will store *DbExecutor* address.</span>
<span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">State</span></span> {
    db: Addr&lt;Syn, DbExecutor&gt;,
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-keyword">let</span> sys = actix::System::new(<span class="hljs-string">"diesel-example"</span>);

    <span class="hljs-comment">// Start 3 parallel db executors</span>
    <span class="hljs-keyword">let</span> addr = SyncArbiter::start(<span class="hljs-number">3</span>, || {
        DbExecutor(SqliteConnection::establish(<span class="hljs-string">"test.db"</span>).unwrap())
    });

    <span class="hljs-comment">// Start http server</span>
    HttpServer::new(<span class="hljs-keyword">move</span> || {
        App::with_state(State{db: addr.clone()})
            .resource(<span class="hljs-string">"/{name}"</span>, |r| r.method(Method::GET).a(index))})
        .bind(<span class="hljs-string">"127.0.0.1:8080"</span>).unwrap()
        .start().unwrap();

    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Started http server: 127.0.0.1:8080"</span>);
    <span class="hljs-keyword">let</span> _ = sys.run();
}
</div></code></pre>
<p>我们将在请求处理程序中使用该地址。处理程序返回future对象; 因此，我们异步接收响应消息。<code>Route::a()</code>必须用于异步处理注册。</p>
<pre class="hljs"><code><div><span class="hljs-comment">/// Async handler</span>
<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(req: HttpRequest&lt;State&gt;) -&gt; <span class="hljs-built_in">Box</span>&lt;Future&lt;Item=HttpResponse, Error=Error&gt;&gt; {
    <span class="hljs-keyword">let</span> name = &amp;req.match_info()[<span class="hljs-string">"name"</span>];

    <span class="hljs-comment">// Send message to `DbExecutor` actor</span>
    req.state().db.send(CreateUser{name: name.to_owned()})
        .from_err()
        .and_then(|res| {
            <span class="hljs-keyword">match</span> res {
                <span class="hljs-literal">Ok</span>(user) =&gt; <span class="hljs-literal">Ok</span>(HttpResponse::<span class="hljs-literal">Ok</span>().json(user)),
                <span class="hljs-literal">Err</span>(_) =&gt; <span class="hljs-literal">Ok</span>(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}
</div></code></pre>
<p><a href="https://github.com/actix/examples/tree/master/diesel/">diesel directory</a>提供了一个完整的示例。</p>
<p>有关同步actors的更多信息可以在<a href="https://docs.rs/actix/0.5.0/actix/sync/index.html">actix documentation</a>文档中找到 。</p>

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
  name: 'databases',
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