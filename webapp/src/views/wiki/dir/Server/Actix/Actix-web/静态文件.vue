<template>
  <div  id="staticfile">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">

<h1 id="%E5%8D%95%E6%96%87%E4%BB%B6">单文件</h1>
<p>可以使用自定义路径模式和NamedFile来提供静态文件服务。要匹配路径尾部，我们可以使用[.*]正则表达式。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> std::path::PathBuf;
<span class="hljs-keyword">use</span> actix_web::{App, HttpRequest, <span class="hljs-built_in">Result</span>, http::Method, fs::NamedFile};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">index</span></span>(req: HttpRequest) -&gt; <span class="hljs-built_in">Result</span>&lt;NamedFile&gt; {
    <span class="hljs-keyword">let</span> path: PathBuf = req.match_info().query(<span class="hljs-string">"tail"</span>)?;
    <span class="hljs-literal">Ok</span>(NamedFile::open(path)?)
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    App::new()
        .resource(<span class="hljs-string">r"/a/{tail:.*}"</span>, |r| r.method(Method::GET).f(index))
        .finish();
}
</div></code></pre>
<h1 id="%E7%9B%AE%E5%BD%95">目录</h1>
<p>StaticFiles可以用作特定目录和子目录文件服务。 StaticFiles必须注册一个App::handler()方法，否则它将无法服务子路径。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> actix_web::{App, fs};

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    App::new()
        .handler(
            <span class="hljs-string">"/static"</span>,
            fs::StaticFiles::new(<span class="hljs-string">"."</span>)
                .show_files_listing())
        .finish();
}
</div></code></pre>
<p>该参数是根目录。默认情况下，子目录的文件列表被禁用。尝试加载目录列表将返回404 Not Found响应。要启用文件列表，请使用<a href="https://actix.rs/actix-web/actix_web/fs/struct.StaticFiles.html#method.show_files_listing">* StaticFiles :: show_files_listing（）*</a> 方法。</p>
<p>与其显示目录的文件列表，宁一种方法是重定向到特定的index文件。使用<a href="https://actix.rs/actix-web/actix_web/fs/struct.StaticFiles.html#method.index_file"><em>StaticFiles::index_file()</em></a>方法来配置此重定向。</p>

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
  name: 'staticfile',
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