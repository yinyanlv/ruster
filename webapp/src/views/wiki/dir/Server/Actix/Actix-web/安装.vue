<template>
  <div  id="installation">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
<h1 id="%E5%AE%89%E8%A3%85-rust">安装 Rust</h1>
<p>既然actix-web是一个Rust框架，你需要Rust来使用它。如果您还没有它，我们建议您使用rustup来管理您的Rust安装。该<a href="https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html">官方Rust指南</a> 有精彩的入门部分。</p>
<p>我们目前至少需要Rust 1.24，因此请确保您运行的rustup update 是最新且最好的Rust版本。特别是本指南将假定您实际运行Rust 1.26或更高版本。</p>
<h1 id="%E5%AE%89%E8%A3%85-actix-web">安装 <code>actix-web</code></h1>
<p>感谢Rust的cargo包管理，您不需要明确安装 actix-web。只需要声明依赖就行了。对于想要使用actix-web的开发版本的情况，您可以直接依赖git存储库。</p>
<p>Release版本：</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies]</span>
<span class="hljs-attr">actix-web</span> = <span class="hljs-string">"0.6"</span>
</div></code></pre>
<p>Development 版本：</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies]</span>
<span class="hljs-attr">actix-web</span> = { git = <span class="hljs-string">"https://github.com/actix/actix-web"</span> }
</div></code></pre>
<h1 id="%E6%B7%B1%E5%85%A5">深入</h1>
<p>在这里有两条你可以选择的路径。你可以沿着指南或者如果你非常不耐烦，你可能想看看我们 广泛的<a href="https://github.com/actix/examples">示例库</a>并运行包含的示例。这里举例说明如何运行包含的basics 示例：</p>
<pre><code>git clone https://github.com/actix/examples
cd examples/basics
cargo run
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
  name: 'installation',
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