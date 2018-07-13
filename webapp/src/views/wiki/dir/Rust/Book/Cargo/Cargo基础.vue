<template>
  <div  id="Cargo基础">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
                 <h1 id="cargo%E5%85%A5%E9%97%A8">cargo入门</h1>
<p>首先，当然还是废话，要使用cargo，自然首先要安装cargo。安装cargo有三种方法，前两种方法请参见rust的安装方法，因为cargo工具是官方正统出身，当然包含在官方的分发包中。第三种方法即从<a href="https://github.com/rust-lang/cargo"><code>cargo</code></a>项目的源码仓库进行构建。Oh，My God。的确是废话。</p>
<p>好了，假设各位已经安装好了cargo，大家和我一起学一下起手式。当然了，猿的世界，起手式一般都千篇一律——那就是<code>hello world</code>大法。
在终端中输入</p>
<pre class="hljs"><code><div>$ cargo new hello_world --bin
</div></code></pre>
<p>上述命令使用<strong>cargo new</strong>在当前目录下新建了基于cargo项目管理的rust项目，项目名称为hello_world，--bin表示该项目将生成可执行文件。具体生成的项目目录结构如下：</p>
<pre class="hljs"><code><div>$ <span class="hljs-built_in">cd</span> hello_world
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
</div></code></pre>
<p>大家可以在终端中输入上述命令，敲出回车键之后即可看到上述结果，或者直接去编辑器或文件管理器中去观察即可。
打开main.rs文件，可以看到，cargo new命令为我们自动生成了hello_world运行所必须的所有代码：</p>
<pre class="hljs"><code><div><span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Hello, world!"</span>);
}
</div></code></pre>
<p>好了，心急的猿们可能已经迫不及待的脱裤子了，好吧，我们先来构建并看看cargo有多神奇，在终端中输入：</p>
<pre class="hljs"><code><div>$ cargo build
</div></code></pre>
<p>稍等片刻，cargo会自动为我们构建好高清应用所需的一切，对于这个起手式来说，缓冲不会超过5秒，12秒88的选手要憋住了。</p>
<pre class="hljs"><code><div>$ cargo run
    Running `target/debug/hello_world`
Hello, world!

</div></code></pre>
<p>看到了什么，看到了什么，吓尿了有木有，吓尿了有木有。好了，cargo就是这么简单。</p>
<p>当然了，说cargo美，并不仅仅是简单这么简单，cargo虽然简单，但是很强大。有多么强大？？可以说，基本上rust开发管理中所需的手段，cargo都有。很小很强大，既强又有节操，不带马，学习曲线几乎为零。</p>
<h1 id="%E5%9F%BA%E4%BA%8Ecargo%E7%9A%84rust%E9%A1%B9%E7%9B%AE%E7%BB%84%E7%BB%87%E7%BB%93%E6%9E%84">基于cargo的rust项目组织结构</h1>
<p>这次不说废话了，先上高清无马图：</p>
<p>对上述cargo默认的项目结构解释如下：</p>
<h5 id="cargotoml%E5%92%8Ccargolock%E6%96%87%E4%BB%B6%E6%80%BB%E6%98%AF%E4%BD%8D%E4%BA%8E%E9%A1%B9%E7%9B%AE%E6%A0%B9%E7%9B%AE%E5%BD%95%E4%B8%8B%E3%80%82"><code>cargo.toml</code>和<code>cargo.lock</code>文件总是位于项目根目录下。</h5>
<h5 id="%E6%BA%90%E4%BB%A3%E7%A0%81%E4%BD%8D%E4%BA%8Esrc%E7%9B%AE%E5%BD%95%E4%B8%8B%E3%80%82">源代码位于<code>src</code>目录下。</h5>
<h5 id="%E9%BB%98%E8%AE%A4%E7%9A%84%E5%BA%93%E5%85%A5%E5%8F%A3%E6%96%87%E4%BB%B6%E6%98%AFsrclibrs%E3%80%82">默认的库入口文件是<code>src/lib.rs</code>。</h5>
<h5 id="%E9%BB%98%E8%AE%A4%E7%9A%84%E5%8F%AF%E6%89%A7%E8%A1%8C%E7%A8%8B%E5%BA%8F%E5%85%A5%E5%8F%A3%E6%96%87%E4%BB%B6%E6%98%AFsrcmainrs%E3%80%82">默认的可执行程序入口文件是<code>src/main.rs</code>。</h5>
<h5 id="%E5%85%B6%E4%BB%96%E5%8F%AF%E9%80%89%E7%9A%84%E5%8F%AF%E6%89%A7%E8%A1%8C%E6%96%87%E4%BB%B6%E4%BD%8D%E4%BA%8Esrcbinrs%E8%BF%99%E9%87%8C%E6%AF%8F%E4%B8%80%E4%B8%AArs%E6%96%87%E4%BB%B6%E5%9D%87%E5%AF%B9%E5%BA%94%E4%B8%80%E4%B8%AA%E5%8F%AF%E6%89%A7%E8%A1%8C%E6%96%87%E4%BB%B6%E3%80%82">其他可选的可执行文件位于<code>src/bin/*.rs</code>(这里每一个rs文件均对应一个可执行文件)。</h5>
<h5 id="%E5%A4%96%E9%83%A8%E6%B5%8B%E8%AF%95%E6%BA%90%E4%BB%A3%E7%A0%81%E6%96%87%E4%BB%B6%E4%BD%8D%E4%BA%8Etests%E7%9B%AE%E5%BD%95%E4%B8%8B%E3%80%82">外部测试源代码文件位于<code>tests</code>目录下。</h5>
<h5 id="%E7%A4%BA%E4%BE%8B%E7%A8%8B%E5%BA%8F%E6%BA%90%E4%BB%A3%E7%A0%81%E6%96%87%E4%BB%B6%E4%BD%8D%E4%BA%8Eexamples%E3%80%82">示例程序源代码文件位于<code>examples</code>。</h5>
<h5 id="%E5%9F%BA%E5%87%86%E6%B5%8B%E8%AF%95%E6%BA%90%E4%BB%A3%E7%A0%81%E6%96%87%E4%BB%B6%E4%BD%8D%E4%BA%8Ebenches%E7%9B%AE%E5%BD%95%E4%B8%8B%E3%80%82">基准测试源代码文件位于<code>benches</code>目录下。</h5>
<p>好了，大家一定谨记这些默认规则，最好按照这种模式来组织自己的rust项目。</p>
<h1 id="cargotoml%E5%92%8Ccargolock">cargo.toml和cargo.lock</h1>
<p><code>cargo.toml</code>和<code>cargo.lock</code>是cargo项目代码管理的核心两个文件，cargo工具的所有活动均基于这两个文件。</p>
<p><code>cargo.toml</code>是cargo特有的项目数据描述文件，对于猿们而言，<code>cargo.toml</code>文件存储了项目的所有信息，它直接面向rust猿，猿们如果想让自己的rust项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建'cargo.toml'。</p>
<p>而<code>cargo.lock</code>文件则不直接面向猿，猿们也不需要直接去修改这个文件。lock文件是cargo工具根据同一项目的toml文件生成的项目依赖详细清单文件，所以我们一般不用不管他，只需要对着<code>cargo.toml</code>文件撸就行了。</p>
<pre class="hljs"><code><div><span class="hljs-section">[package]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"hello_world"</span>
<span class="hljs-attr">version</span> = <span class="hljs-string">"0.1.0"</span>
<span class="hljs-attr">authors</span> = [<span class="hljs-string">"fuying"</span>]
<span class="hljs-section">
[dependencies]</span>
</div></code></pre>
<p>toml文件是由诸如[package]或[dependencies]这样的段落组成，每一个段落又由多个字段组成，这些段落和字段就描述了项目组织的基本信息，例如上述toml文件中的[package]段落描述了<code>hello_world</code>项目本身的一些信息，包括项目名称（对应于name字段）、项目版本（对应于version字段）、作者列表（对应于authors字段）等；[dependencies]段落描述了<code>hello_world</code>项目的依赖项目有哪些。</p>
<p>下面我们来看看toml描述文件中常用段落和字段的意义。</p>
<h1 id="package%E6%AE%B5%E8%90%BD">package段落</h1>
<p>[package]段落描述了软件开发者对本项目的各种元数据描述信息，例如[name]字段定义了项目的名称，[version]字段定义了项目的当前版本，[authors]定义了该项目的所有作者，当然，[package]段落不仅仅包含这些字段，[package]段落的其他可选字段详见cargo参数配置章节。</p>
<h1 id="%E5%AE%9A%E4%B9%89%E9%A1%B9%E7%9B%AE%E4%BE%9D%E8%B5%96">定义项目依赖</h1>
<p>使用cargo工具的最大优势就在于，能够对该项目的各种依赖项进行方便、统一和灵活的管理。这也是使用cargo对rust 的项目进行管理的重要目标之一。在cargo的toml文件描述中，主要通过各种依赖段落来描述该项目的各种依赖项。toml中常用的依赖段落包括一下几种：</p>
<ul>
<li><a href="http://xn--rustcrates-cp3p6z163gtukb1ksy2a.io">基于rust官方仓库crates.io</a>，通过版本说明来描述：</li>
<li>基于项目源代码的git仓库地址，通过URL来描述：</li>
<li>基于本地项目的绝对路径或者相对路径，通过类Unix模式的路径来描述：
这三种形式具体写法如下：</li>
</ul>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies]</span>
<span class="hljs-attr">typemap</span> = <span class="hljs-string">"0.3"</span>
<span class="hljs-attr">plugin</span> = <span class="hljs-string">"0.2*"</span>
<span class="hljs-attr">hammer</span> = { version = <span class="hljs-string">"0.5.0"</span>}
<span class="hljs-attr">color</span> = { git = <span class="hljs-string">"https://github.com/bjz/color-rs"</span> }
<span class="hljs-attr">geometry</span> = { path = <span class="hljs-string">"crates/geometry"</span> }
</div></code></pre>
<p>上述例子中，2-4行为方法一的写法，第5行为方法二的写法，第6行为方法三的写法。
这三种写法各有用处，如果项目需要使用crates.io官方仓库来管理项目依赖项，推荐使用第一种方法。如果项目开发者更倾向于使用git仓库中最新的源码，可以使用方法二。方法二也经常用于当官方仓库的依赖项编译不通过时的备选方案。方法三主要用于源代码位于本地的依赖项。</p>
<h1 id="%E5%AE%9A%E4%B9%89%E9%9B%86%E6%88%90%E6%B5%8B%E8%AF%95%E7%94%A8%E4%BE%8B">定义集成测试用例</h1>
<p>cargo另一个重要的功能，即将软件开发过程中必要且非常重要的测试环节进行集成，并通过代码属性声明或者toml文件描述来对测试进行管理。其中，单元测试主要通过在项目代码的测试代码部分前用<code>#[test]</code>属性来描述，而集成测试，则一般都会通过toml文件中的[[test]]段落进行描述。
例如，假设集成测试文件均位于tests文件夹下，则toml可以这样来写：</p>
<pre class="hljs"><code><div><span class="hljs-section">[[test]]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"testinit"</span>
<span class="hljs-attr">path</span> = <span class="hljs-string">"tests/testinit.rs"</span>
<span class="hljs-section">
[[test]]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"testtime"</span>
<span class="hljs-attr">path</span> = <span class="hljs-string">"tests/testtime.rs"</span>
</div></code></pre>
<p>上述例子中，name字段定义了集成测试的名称，path字段定义了集成测试文件相对于本toml文件的路径。
看看，定义集成测试就是如此简单。
需要注意的是:</p>
<ul>
<li>如果没有在Cargo.toml里定义集成测试的入口，那么tests目录(不包括子目录)下的每个rs文件被当作集成测试入口.</li>
<li>如果在Cargo.toml里定义了集成测试入口，那么定义的那些rs就是入口，不再默认指定任何集成测试入口.</li>
</ul>
<h1 id="%E5%AE%9A%E4%B9%89%E9%A1%B9%E7%9B%AE%E7%A4%BA%E4%BE%8B%E5%92%8C%E5%8F%AF%E6%89%A7%E8%A1%8C%E7%A8%8B%E5%BA%8F">定义项目示例和可执行程序</h1>
<p><strong>上面我们介绍了cargo项目管理中常用的三个功能，还有两个经常使用的功能：example用例的描述以及bin用例的描述。其描述方法和test用例描述方法类似。不过，这时候段落名称'[[test]]'分别替换为：'[[example]]'或者'[[bin]]'。例如：</strong></p>
<pre class="hljs"><code><div><span class="hljs-section">[[example]]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"timeout"</span>
<span class="hljs-attr">path</span> = <span class="hljs-string">"examples/timeout.rs"</span>
<span class="hljs-section">
[[bin]]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"bin1"</span>
<span class="hljs-attr">path</span> = <span class="hljs-string">"bin/bin1.rs"</span>

</div></code></pre>
<p>对于'[[example]]'和'[[bin]]'段落中声明的examples和bins，需要通过'cargo run --example NAME'或者'cargo run --bin NAME'来运行，其中NAME对应于你在name字段中定义的名称。</p>
<h1 id="%E6%9E%84%E5%BB%BA%E3%80%81%E6%B8%85%E7%90%86%E3%80%81%E6%9B%B4%E6%96%B0%E4%BB%A5%E5%8F%8A%E5%AE%89%E8%A3%85">构建、清理、更新以及安装</h1>
<p>领会了toml描述文件的写法，是一个重要的方面。另一个重要的方面，就是cargo工具本身为我们程序猿提供的各种好用的工具。如果大家感兴趣，自己在终端中输入'cargo --help'查看即可。其中开发时最常用的命令就是'cargo build'，用于构建项目。此外，'cargo clean'命令可以清理target文件夹中的所有内容；'cargo update'根据toml描述文件重新检索并更新各种依赖项的信息，并写入lock文件，例如依赖项版本的更新变化等等；'cargo install'可用于实际的生产部署。这些命令在实际的开发部署中均是非常有用的。</p>
<p><strong>cargo更多详细用法请参见<a href="../cargo-detailed-cfg/cargo-detailed-cfg.md">'28. cargo参数配置'</a></strong></p>

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
  name: 'Cargo基础',
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