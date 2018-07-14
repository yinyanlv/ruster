<template>
  <div  id="cargoset">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
<h2 id="cargo">Cargo</h2>
<ul>
<li>Rust's package manager &amp; build tool</li>
<li>Create a new project:
<ul>
<li><code>cargo new project_name</code> (library)</li>
<li><code>cargo new project_name --bin</code> (executable)</li>
</ul>
</li>
<li>Build your project: <code>cargo build</code></li>
<li>Run your tests: <code>cargo test</code>
<ul>
<li>These get tedious to type, so shell alias to your heart's content,
e.g., <code>cargob</code>/<code>cb</code> and <code>cargot</code>/<code>ct</code></li>
</ul>
</li>
<li>Magic, right? How does this work?</li>
</ul>
<hr>
<h3 id="cargotoml">Cargo.toml</h3>
<ul>
<li>Cargo uses the <code>Cargo.toml</code> file to declare and manage dependencies and
project metadata.
<ul>
<li>TOML is a simple format similar to INI.</li>
</ul>
</li>
<li>More in your first homework assignments.</li>
</ul>
<pre class="hljs"><code><div><span class="hljs-section">[package]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"Rust"</span>
<span class="hljs-attr">version</span> = <span class="hljs-string">"0.1.0"</span>
<span class="hljs-attr">authors</span> = [<span class="hljs-string">"Ferris &lt;cis198@seas.upenn.edu&gt;"</span>]
<span class="hljs-section">
[dependencies]</span>
<span class="hljs-attr">uuid</span> = <span class="hljs-string">"0.1"</span>
<span class="hljs-attr">rand</span> = <span class="hljs-string">"0.3"</span>
<span class="hljs-section">
[profile.release]</span>
<span class="hljs-attr">opt-level</span> = <span class="hljs-number">3</span>
<span class="hljs-attr">debug</span> = <span class="hljs-literal">false</span>
</div></code></pre>
<hr>
<h3 id="cargo-test"><code>cargo test</code></h3>
<ul>
<li>A test is any function annotated with <code>#[test]</code>.</li>
<li><code>cargo test</code> will run all annotated functions in your project.</li>
<li>Any function which executes without crashing (<code>panic!</code>ing) succeeds.</li>
<li>Use <code>assert!</code> (or <code>assert_eq!</code>) to check conditions (and <code>panic!</code> on failure)</li>
<li>You'll use this in HW01.</li>
</ul>
<pre class="hljs"><code><div><span class="hljs-meta">#[test]</span>
<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">it_works</span></span>() {
    <span class="hljs-comment">// ...</span>
}
</div></code></pre>
<hr>
<h3 id="cargo-check"><code>cargo check</code></h3>
<ul>
<li>Not available by default!</li>
<li>Run <code>cargo install cargo-check</code> to install it.</li>
<li>Functionally the same as <code>cargo build</code>, but doesn't actually generate any code.
<ul>
<li>=&gt; Faster!</li>
</ul>
</li>
</ul>
<p>本章将深入探讨cargo的一些细节问题，这包括以下几个方面：</p>
<ul>
<li>基于语义化版本的项目版本声明与管理</li>
<li>cargo的toml描述文件配置字段详细参考</li>
</ul>
<h1 id="%E5%9F%BA%E4%BA%8E%E8%AF%AD%E4%B9%89%E5%8C%96%E7%89%88%E6%9C%AC%E7%9A%84%E9%A1%B9%E7%9B%AE%E7%89%88%E6%9C%AC%E5%A3%B0%E6%98%8E%E4%B8%8E%E7%AE%A1%E7%90%86">基于语义化版本的项目版本声明与管理</h1>
<p>我们在使用toml描述文件对项目进行配置时，经常会遇到项目版本声明及管理的问题，比如：</p>
<pre class="hljs"><code><div><span class="hljs-section">[package]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"libevent_sys"</span>
<span class="hljs-attr">version</span> = <span class="hljs-string">"0.1.0"</span>
<span class="hljs-section">
[dependencies]</span>
<span class="hljs-attr">libc</span> = <span class="hljs-string">"0.2"</span>

</div></code></pre>
<p>这里package段落中的version字段的值，以及dependencies段落中的libc字段的值，这些值的写法，都涉及到语义化版本控制的问题。语义化版本控制是用一组简单的规则及条件来约束版本号的配置和增长。这些规则是根据（但不局限于）已经被各种封闭、开放源码软件所广泛使用的惯例所设计。简单来说，语义化版本控制遵循下面这些规则：</p>
<ul>
<li>版本格式：主版本号.次版本号.修订号，版本号递增规则如下：</li>
</ul>
<ol>
<li>主版本号：当你做了不兼容的 API 修改，</li>
<li>次版本号：当你做了向下兼容的功能性新增，</li>
<li>修订号：当你做了向下兼容的问题修正。</li>
</ol>
<ul>
<li>先行版本号及版本编译信息可以加到“主版本号.次版本号.修订号”的后面，作为延伸。</li>
</ul>
<p>关于语义化版本控制的具体细节问题，大家可以参考<a href="http://semver.org/lang/zh-CN/">这里</a>，我不再赘述。</p>
<h1 id="cargo%E7%9A%84toml%E6%8F%8F%E8%BF%B0%E6%96%87%E4%BB%B6%E9%85%8D%E7%BD%AE%E5%AD%97%E6%AE%B5%E8%AF%A6%E7%BB%86%E5%8F%82%E8%80%83">cargo的toml描述文件配置字段详细参考</h1>
<h2 id="package%E6%AE%B5%E8%90%BD">[package]段落</h2>
<p>啥也不多说了，直接上例子，大家注意我在例子中的中文解释，个人觉得这样比较一目了然：</p>
<pre class="hljs"><code><div><span class="hljs-section">[package]</span>
 <span class="hljs-comment"># 软件包名称，如果需要在别的地方引用此软件包，请用此名称。</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"hello_world"</span>

<span class="hljs-comment"># 当前版本号，这里遵循semver标准，也就是语义化版本控制标准。</span>
<span class="hljs-attr">version</span> = <span class="hljs-string">"0.1.0"</span>    # the current version, obeying semver

<span class="hljs-comment"># 软件所有作者列表</span>
<span class="hljs-attr">authors</span> = [<span class="hljs-string">"you@example.com"</span>]

<span class="hljs-comment"># 非常有用的一个字段，如果要自定义自己的构建工作流，</span>
<span class="hljs-comment"># 尤其是要调用外部工具来构建其他本地语言（C、C++、D等）开发的软件包时。</span>
<span class="hljs-comment"># 这时，自定义的构建流程可以使用rust语言，写在"build.rs"文件中。</span>
<span class="hljs-attr">build</span> = <span class="hljs-string">"build.rs"</span>

<span class="hljs-comment"># 显式声明软件包文件夹内哪些文件被排除在项目的构建流程之外，</span>
<span class="hljs-comment"># 哪些文件包含在项目的构建流程中</span>
<span class="hljs-attr">exclude</span> = [<span class="hljs-string">"build/**/*.o"</span>, <span class="hljs-string">"doc/**/*.html"</span>]
<span class="hljs-attr">include</span> = [<span class="hljs-string">"src/**/*"</span>, <span class="hljs-string">"Cargo.toml"</span>]

<span class="hljs-comment"># 当软件包在向公共仓库发布时出现错误时，使能此字段可以阻止此错误。</span>
<span class="hljs-attr">publish</span> = <span class="hljs-literal">false</span>

<span class="hljs-comment"># 关于软件包的一个简短介绍。</span>
<span class="hljs-attr">description</span> = <span class="hljs-string">"..."</span>

<span class="hljs-comment"># 下面这些字段标明了软件包仓库的更多信息</span>
<span class="hljs-attr">documentation</span> = <span class="hljs-string">"..."</span>
<span class="hljs-attr">homepage</span> = <span class="hljs-string">"..."</span>
<span class="hljs-attr">repository</span> = <span class="hljs-string">"..."</span>

<span class="hljs-comment"># 顾名思义，此字段指向的文件就是传说中的ReadMe，</span>
<span class="hljs-comment"># 并且，此文件的内容最终会保存在注册表数据库中。</span>
<span class="hljs-attr">readme</span> = <span class="hljs-string">"..."</span>

<span class="hljs-comment"># 用于分类和检索的关键词。</span>
<span class="hljs-attr">keywords</span> = [<span class="hljs-string">"..."</span>, <span class="hljs-string">"..."</span>]

<span class="hljs-comment"># 软件包的许可证，必须是cargo仓库已列出的已知的标准许可证。</span>
<span class="hljs-attr">license</span> = <span class="hljs-string">"..."</span>

<span class="hljs-comment"># 软件包的非标许可证书对应的文件路径。</span>
<span class="hljs-attr">license-file</span> = <span class="hljs-string">"..."</span>
</div></code></pre>
<h2 id="%E4%BE%9D%E8%B5%96%E7%9A%84%E8%AF%A6%E7%BB%86%E9%85%8D%E7%BD%AE">依赖的详细配置</h2>
<p>最直接的方式在之前第五章探讨过，这里不在赘述，例如这样：</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies]</span>
<span class="hljs-attr">hammer</span> = <span class="hljs-string">"0.5.0"</span>
<span class="hljs-attr">color</span> = <span class="hljs-string">"&gt; 0.6.0, &lt; 0.8.0"</span>
</div></code></pre>
<p>与平台相关的依赖定义格式不变，不同的是需要定义在[target]字段下。例如：</p>
<pre class="hljs"><code><div><span class="hljs-comment"># 注意，此处的cfg可以使用not、any、all等操作符任意组合键值对。</span>
<span class="hljs-comment"># 并且此用法仅支持cargo 0.9.0（rust 1.8.0）以上版本。</span>
<span class="hljs-comment"># 如果是windows平台，则需要此依赖。</span>
<span class="hljs-section">[target.'cfg(windows)'.dependencies]</span>
<span class="hljs-attr">winhttp</span> = <span class="hljs-string">"0.4.0"</span>
<span class="hljs-section">
[target.'cfg(unix)'.dependencies]</span>
<span class="hljs-attr">openssl</span> = <span class="hljs-string">"1.0.1"</span>

<span class="hljs-comment">#如果是32位平台，则需要此依赖。</span>
<span class="hljs-section">[target.'cfg(target_pointer_width = "32")'.dependencies]</span>
<span class="hljs-attr">native</span> = { path = <span class="hljs-string">"native/i686"</span> }
<span class="hljs-section">
[target.'cfg(target_pointer_width = "64")'.dependencies]</span>
<span class="hljs-attr">native</span> = { path = <span class="hljs-string">"native/i686"</span> }

<span class="hljs-comment"># 另一种写法就是列出平台的全称描述</span>
<span class="hljs-section">[target.x86_64-pc-windows-gnu.dependencies]</span>
<span class="hljs-attr">winhttp</span> = <span class="hljs-string">"0.4.0"</span>
<span class="hljs-section">[target.i686-unknown-linux-gnu.dependencies]</span>
<span class="hljs-attr">openssl</span> = <span class="hljs-string">"1.0.1"</span>

<span class="hljs-comment"># 如果使用自定义平台，请将自定义平台文件的完整路径用双引号包含</span>
<span class="hljs-section">[target."x86_64/windows.json".dependencies]</span>
<span class="hljs-attr">winhttp</span> = <span class="hljs-string">"0.4.0"</span>
<span class="hljs-section">[target."i686/linux.json".dependencies]</span>
<span class="hljs-attr">openssl</span> = <span class="hljs-string">"1.0.1"</span>
<span class="hljs-attr">native</span> = { path = <span class="hljs-string">"native/i686"</span> }
<span class="hljs-attr">openssl</span> = <span class="hljs-string">"1.0.1"</span>
<span class="hljs-attr">native</span> = { path = <span class="hljs-string">"native/x86_64"</span> }

<span class="hljs-comment"># [dev-dependencies]段落的格式等同于[dependencies]段落，</span>
<span class="hljs-comment"># 不同之处在于，[dependencies]段落声明的依赖用于构建软件包，</span>
<span class="hljs-comment"># 而[dev-dependencies]段落声明的依赖仅用于构建测试和性能评估。</span>
<span class="hljs-comment"># 此外，[dev-dependencies]段落声明的依赖不会传递给其他依赖本软件包的项目</span>
<span class="hljs-section">[dev-dependencies]</span>
<span class="hljs-attr">iron</span> = <span class="hljs-string">"0.2"</span>

</div></code></pre>
<h2 id="%E8%87%AA%E5%AE%9A%E4%B9%89%E7%BC%96%E8%AF%91%E5%99%A8%E8%B0%83%E7%94%A8%E6%96%B9%E5%BC%8F%E6%A8%A1%E6%9D%BF%E8%AF%A6%E7%BB%86%E5%8F%82%E6%95%B0">自定义编译器调用方式模板详细参数</h2>
<p>cargo内置五种编译器调用模板，分别为dev、release、test、bench、doc，分别用于定义不同类型生成目标时的编译器参数，如果我们自己想改变这些编译模板，可以自己定义相应字段的值，例如（注意：下述例子中列出的值均为此模板字段对应的系统默认值）：</p>
<pre class="hljs"><code><div><span class="hljs-comment"># 开发模板, 对应`cargo build`命令</span>
<span class="hljs-section">[profile.dev]</span>
<span class="hljs-attr">opt-level</span> = <span class="hljs-number">0</span>  # 控制编译器的 --opt-level 参数，也就是优化参数
<span class="hljs-attr">debug</span> = <span class="hljs-literal">true</span>   # 控制编译器是否开启 `-g` 参数
<span class="hljs-attr">rpath</span> = <span class="hljs-literal">false</span>  # 控制编译器的 `-C rpath` 参数
<span class="hljs-attr">lto</span> = <span class="hljs-literal">false</span>    # 控制`-C lto` 参数，此参数影响可执行文件和静态库的生成，
<span class="hljs-attr">debug-assertions</span> = <span class="hljs-literal">true</span>  # 控制调试断言是否开启
<span class="hljs-attr">codegen-units</span> = <span class="hljs-number">1</span> # 控制编译器的 `-C codegen-units` 参数。注意，当`lto = <span class="hljs-literal">true</span>`时，此字段值被忽略

<span class="hljs-comment"># 发布模板, 对应`cargo build --release`命令</span>
<span class="hljs-section">[profile.release]</span>
<span class="hljs-attr">opt-level</span> = <span class="hljs-number">3</span>
<span class="hljs-attr">debug</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">rpath</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">lto</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">debug-assertions</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">codegen-units</span> = <span class="hljs-number">1</span>

<span class="hljs-comment"># 测试模板，对应`cargo test`命令</span>
<span class="hljs-section">[profile.test]</span>
<span class="hljs-attr">opt-level</span> = <span class="hljs-number">0</span>
<span class="hljs-attr">debug</span> = <span class="hljs-literal">true</span>
<span class="hljs-attr">rpath</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">lto</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">debug-assertions</span> = <span class="hljs-literal">true</span>
<span class="hljs-attr">codegen-units</span> = <span class="hljs-number">1</span>

<span class="hljs-comment"># 性能评估模板，对应`cargo bench`命令</span>
<span class="hljs-section">[profile.bench]</span>
<span class="hljs-attr">opt-level</span> = <span class="hljs-number">3</span>
<span class="hljs-attr">debug</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">rpath</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">lto</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">debug-assertions</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">codegen-units</span> = <span class="hljs-number">1</span>

<span class="hljs-comment"># 文档模板，对应`cargo doc`命令</span>
<span class="hljs-section">[profile.doc]</span>
<span class="hljs-attr">opt-level</span> = <span class="hljs-number">0</span>
<span class="hljs-attr">debug</span> = <span class="hljs-literal">true</span>
<span class="hljs-attr">rpath</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">lto</span> = <span class="hljs-literal">false</span>
<span class="hljs-attr">debug-assertions</span> = <span class="hljs-literal">true</span>
<span class="hljs-attr">codegen-units</span> = <span class="hljs-number">1</span>

</div></code></pre>
<p>需要注意的是，当调用编译器时，只有位于调用最顶层的软件包的模板文件有效，其他的子软件包或者依赖软件包的模板定义将被顶层软件包的模板覆盖。</p>
<h2 id="features%E6%AE%B5%E8%90%BD">[features]段落</h2>
<p>[features]段落中的字段被用于条件编译选项或者是可选依赖。例如：</p>
<pre class="hljs"><code><div><span class="hljs-section">[package]</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"awesome"</span>
<span class="hljs-section">
[features]</span>
<span class="hljs-comment"># 此字段设置了可选依赖的默认选择列表，</span>
<span class="hljs-comment"># 注意这里的"session"并非一个软件包名称，</span>
<span class="hljs-comment"># 而是另一个featrue字段session</span>
<span class="hljs-attr">default</span> = [<span class="hljs-string">"jquery"</span>, <span class="hljs-string">"uglifier"</span>, <span class="hljs-string">"session"</span>]

<span class="hljs-comment"># 类似这样的值为空的feature一般用于条件编译，</span>
<span class="hljs-comment"># 类似于`#[cfg(feature = "go-faster")]`。</span>
<span class="hljs-attr">go-faster</span> = []

<span class="hljs-comment"># 此feature依赖于bcrypt软件包，</span>
<span class="hljs-comment"># 这样封装的好处是未来可以对secure-password此feature增加可选项目。</span>
<span class="hljs-attr">secure-password</span> = [<span class="hljs-string">"bcrypt"</span>]

<span class="hljs-comment"># 此处的session字段导入了cookie软件包中的feature段落中的session字段</span>
<span class="hljs-attr">session</span> = [<span class="hljs-string">"cookie/session"</span>]
<span class="hljs-section">
[dependencies]</span>
<span class="hljs-comment"># 必要的依赖</span>
<span class="hljs-attr">cookie</span> = <span class="hljs-string">"1.2.0"</span>
<span class="hljs-attr">oauth</span> = <span class="hljs-string">"1.1.0"</span>
<span class="hljs-attr">route-recognizer</span> = <span class="hljs-string">"=2.1.0"</span>

<span class="hljs-comment"># 可选依赖</span>
<span class="hljs-attr">jquery</span> = { version = <span class="hljs-string">"1.0.2"</span>, optional = <span class="hljs-literal">true</span> }
<span class="hljs-attr">uglifier</span> = { version = <span class="hljs-string">"1.5.3"</span>, optional = <span class="hljs-literal">true</span> }
<span class="hljs-attr">bcrypt</span> = { version = <span class="hljs-string">"*"</span>, optional = <span class="hljs-literal">true</span> }
<span class="hljs-attr">civet</span> = { version = <span class="hljs-string">"*"</span>, optional = <span class="hljs-literal">true</span> }
</div></code></pre>
<p>如果其他软件包要依赖使用上述awesome软件包，可以在其描述文件中这样写：</p>
<pre class="hljs"><code><div><span class="hljs-section">[dependencies.awesome]</span>
<span class="hljs-attr">version</span> = <span class="hljs-string">"1.3.5"</span>
<span class="hljs-attr">default-features</span> = <span class="hljs-literal">false</span> # 禁用awesome 的默认features
<span class="hljs-attr">features</span> = [<span class="hljs-string">"secure-password"</span>, <span class="hljs-string">"civet"</span>] # 使用此处列举的各项features
</div></code></pre>
<p>使用features时需要遵循以下规则：</p>
<ul>
<li>feature名称在本描述文件中不能与出现的软件包名称冲突</li>
<li>除了default feature，其他所有的features均是可选的</li>
<li>features不能相互循环包含</li>
<li>开发依赖包不能包含在内</li>
<li>features组只能依赖于可选软件包</li>
</ul>
<p>features的一个重要用途就是，当开发者需要对软件包进行最终的发布时，在进行构建时可以声明暴露给终端用户的features，这可以通过下述命令实现：</p>
<pre><code>$ cargo build --release --features "shumway pdf"
</code></pre>
<h2 id="%E5%85%B3%E4%BA%8E%E6%B5%8B%E8%AF%95">关于测试</h2>
<p>当运行cargo test命令时，cargo将会按做以下事情：</p>
<ul>
<li>编译并运行软件包源代码中被#[cfg(test)] 所标志的单元测试</li>
<li>编译并运行文档测试</li>
<li>编译并运行集成测试</li>
<li>编译examples</li>
</ul>
<h2 id="%E9%85%8D%E7%BD%AE%E6%9E%84%E5%BB%BA%E7%9B%AE%E6%A0%87">配置构建目标</h2>
<p>所有的诸如[[bin]], [lib], [[bench]], [[test]]以及 [[example]]等字段，均提供了类似的配置，以说明构建目标应该怎样被构建。例如（下述例子中[lib]段落中各字段值均为默认值）：</p>
<pre class="hljs"><code><div><span class="hljs-section">[lib]</span>
<span class="hljs-comment"># 库名称，默认与项目名称相同</span>
<span class="hljs-attr">name</span> = <span class="hljs-string">"foo"</span>

<span class="hljs-comment"># 此选项仅用于[lib]段落，其决定构建目标的构建方式，</span>
<span class="hljs-comment"># 可以取dylib, rlib, staticlib 三种值之一，表示生成动态库、r库或者静态库。</span>
<span class="hljs-attr">crate-type</span> = [<span class="hljs-string">"dylib"</span>]

<span class="hljs-comment"># path字段声明了此构建目标相对于cargo.toml文件的相对路径</span>
<span class="hljs-attr">path</span> = <span class="hljs-string">"src/lib.rs"</span>

<span class="hljs-comment"># 单元测试开关选项</span>
<span class="hljs-attr">test</span> = <span class="hljs-literal">true</span>

<span class="hljs-comment"># 文档测试开关选项</span>
<span class="hljs-attr">doctest</span> = <span class="hljs-literal">true</span>

<span class="hljs-comment"># 性能评估开关选项</span>
<span class="hljs-attr">bench</span> = <span class="hljs-literal">true</span>

<span class="hljs-comment"># 文档生成开关选项</span>
<span class="hljs-attr">doc</span> = <span class="hljs-literal">true</span>

<span class="hljs-comment"># 是否构建为编译器插件的开关选项</span>
<span class="hljs-attr">plugin</span> = <span class="hljs-literal">false</span>

<span class="hljs-comment"># 如果设置为false，`cargo test`将会忽略传递给rustc的--test参数。</span>
<span class="hljs-attr">harness</span> = <span class="hljs-literal">true</span>
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
  name: 'cargoset',
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