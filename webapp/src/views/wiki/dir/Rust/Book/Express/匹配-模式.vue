<template>
  <div  id="pattern">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
<h2 id="%E5%8C%B9%E9%85%8D">匹配</h2>
<p>一个简单的[<code>if</code>](If <a href="http://xn--If-8j6c183s.md">If语句.md</a>)/<code>else</code>往往是不够的，因为你可能有两个或更多个选项。这样<code>else</code>也会变得异常复杂。Rust 有一个<code>match</code>关键字，它可以让你有效的取代复杂的<code>if</code>/<code>else</code>组。看看下面的代码：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">match</span> x {
    <span class="hljs-number">1</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"one"</span>),
    <span class="hljs-number">2</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"two"</span>),
    <span class="hljs-number">3</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"three"</span>),
    <span class="hljs-number">4</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"four"</span>),
    <span class="hljs-number">5</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"five"</span>),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"something else"</span>),
}
</div></code></pre>
<p><code>match</code>使用一个表达式然后基于它的值分支。每个分支都是<code>val =&gt; expression</code>这种形式。当匹配到一个分支，它的表达式将被执行。<code>match</code>属于“模式匹配”的范畴，<code>match</code>是它的一个实现。有[一个整个关于模式的部分](Patterns <a href="http://xn--w0t69v.md">模式.md</a>)讲到了所有可能的模式。</p>
<p>match 中，若 回傳空值，不能直接回傳 None 而要回傳空字串，這是因為 Rust 要求回傳的型別需一致。若很確定該鍵/值對的確存在，可參考以下方式取值：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">use</span> std::collections::HashMap;
 
<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">main</span></span>() {
    <span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> hash = HashMap::new();
 
    hash.insert(<span class="hljs-string">"one"</span>, <span class="hljs-string">"eins"</span>);
    hash.insert(<span class="hljs-string">"two"</span>, <span class="hljs-string">"zwei"</span>);
    hash.insert(<span class="hljs-string">"three"</span>, <span class="hljs-string">"drei"</span>);
 
    <span class="hljs-keyword">let</span> data = *(hash.get(<span class="hljs-string">"one"</span>).unwrap());
    <span class="hljs-built_in">assert_eq!</span>(data, <span class="hljs-string">"eins"</span>);
}

</div></code></pre>
<p>那么这有什么巨大的优势呢？这确实有优势。第一，<code>match</code>强制<em>穷尽性检查</em>（<em>exhaustiveness checking</em>）。你看到了最后那个下划线开头的分支了吗？如果去掉它，Rust 将会给我们一个错误：</p>
<pre><code class="language-text">error: non-exhaustive patterns: `_` not covered
</code></pre>
<p>Rust 试图告诉我们忘记了一个值。编译器从<code>x</code>推断它可以是任何正的 32 位整型值；例如从 1 到 2,147,483,647。<code>_</code>就像一个<em>匹配所有</em>的分支，它会捕获所有没有被<code>match</code>分支捕获的所有可能值。如你所见，在上个例子中，我们提供了 1 到 5 的<code>mtach</code>分支，如果<code>x</code>是 6 或者其他值，那么它会被<code>_</code>捕获。</p>
<p><code>match</code>也是一个表达式，也就是说它可以用在<code>let</code>绑定的右侧或者其它直接用到表达式的地方：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">let</span> number = <span class="hljs-keyword">match</span> x {
    <span class="hljs-number">1</span> =&gt; <span class="hljs-string">"one"</span>,
    <span class="hljs-number">2</span> =&gt; <span class="hljs-string">"two"</span>,
    <span class="hljs-number">3</span> =&gt; <span class="hljs-string">"three"</span>,
    <span class="hljs-number">4</span> =&gt; <span class="hljs-string">"four"</span>,
    <span class="hljs-number">5</span> =&gt; <span class="hljs-string">"five"</span>,
    _ =&gt; <span class="hljs-string">"something else"</span>,
};
</div></code></pre>
<p>有时，这是一个把一种类型的数据转换为另一个类型的好方法。</p>
<h2 id="%E5%8C%B9%E9%85%8D%E6%9E%9A%E4%B8%BE%EF%BC%88matching-on-enums%EF%BC%89">匹配枚举（Matching on enums）</h2>
<p><code>match</code>的另一个重要的作用是处理枚举的可能变量：</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">enum</span> <span class="hljs-title">Message</span></span> {
    Quit,
    ChangeColor(<span class="hljs-built_in">i32</span>, <span class="hljs-built_in">i32</span>, <span class="hljs-built_in">i32</span>),
    Move { x: <span class="hljs-built_in">i32</span>, y: <span class="hljs-built_in">i32</span> },
    Write(<span class="hljs-built_in">String</span>),
}

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">quit</span></span>() { <span class="hljs-comment">/* ... */</span> }
<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">change_color</span></span>(r: <span class="hljs-built_in">i32</span>, g: <span class="hljs-built_in">i32</span>, b: <span class="hljs-built_in">i32</span>) { <span class="hljs-comment">/* ... */</span> }
<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">move_cursor</span></span>(x: <span class="hljs-built_in">i32</span>, y: <span class="hljs-built_in">i32</span>) { <span class="hljs-comment">/* ... */</span> }

<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">process_message</span></span>(msg: Message) {
    <span class="hljs-keyword">match</span> msg {
        Message::Quit =&gt; quit(),
        Message::ChangeColor(r, g, b) =&gt; change_color(r, g, b),
        Message::Move { x: x, y: y } =&gt; move_cursor(x, y),
        Message::Write(s) =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, s),
    };
}
</div></code></pre>
<p>再一次，Rust编译器检查穷尽性，所以它要求对每一个枚举的变量都有一个匹配分支。如果你忽略了一个，除非你用<code>_</code>否则它会给你一个编译时错误。</p>
<p>与之前的<code>match</code>的作用不同，你不能用常规的<code>if</code>语句来做这些。你可以使用[if let](if <a href="http://let.md">let.md</a>)语句，它可以被看作是一个<code>match</code>的简略形式。</p>
<h1 id="%E6%A8%A1%E5%BC%8F">模式</h1>
<p>模式在Rust中十分常见。我们在[变量绑定](Variable Bindings <a href="http://xn--vor83py25aw4p.md">变量绑定.md</a>)，[匹配语句](Match <a href="http://xn--ckrq85l.md">匹配.md</a>)和其它一些地方使用它们。让我们开始一个快速的关于模式可以干什么的教程！</p>
<p>快速回顾：你可以直接匹配常量，并且<code>_</code>作为“任何”类型：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">1</span>;

<span class="hljs-keyword">match</span> x {
    <span class="hljs-number">1</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"one"</span>),
    <span class="hljs-number">2</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"two"</span>),
    <span class="hljs-number">3</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"three"</span>),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"anything"</span>),
}
</div></code></pre>
<p>这会打印出<code>one</code>。</p>
<p>有一个模式的陷阱：就像任何引入一个新绑定的语句，他们会引入隐藏。例如：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">1</span>;
<span class="hljs-keyword">let</span> c = <span class="hljs-string">'c'</span>;

<span class="hljs-keyword">match</span> c {
    x =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x: {} c: {}"</span>, x, c),
}

<span class="hljs-built_in">println!</span>(<span class="hljs-string">"x: {}"</span>, x)
</div></code></pre>
<p>这会打印：</p>
<pre><code class="language-text">x: c c: c
x: 1
</code></pre>
<p>换句话说，<code>x =&gt;</code>匹配到了模式并引入了一个叫做<code>x</code>的新绑定。这个新绑定的作用域是匹配分支并拥有<code>c</code>的值。注意匹配作用域外的<code>x</code>的值对内部的<code>x</code>的值并无影响。因为我们已经有了一个<code>x</code>，新的<code>x</code>隐藏了它。</p>
<h2 id="%E5%A4%9A%E9%87%8D%E6%A8%A1%E5%BC%8F%EF%BC%88multiple-patterns%EF%BC%89">多重模式（Multiple patterns）</h2>
<p>你可以使用<code>|</code>匹配多个模式：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">1</span>;

<span class="hljs-keyword">match</span> x {
    <span class="hljs-number">1</span> | <span class="hljs-number">2</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"one or two"</span>),
    <span class="hljs-number">3</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"three"</span>),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"anything"</span>),
}
</div></code></pre>
<p>这会输出<code>one or two</code>。</p>
<h2 id="%E8%A7%A3%E6%9E%84%EF%BC%88destructuring%EF%BC%89">解构（Destructuring）</h2>
<p>如果你有一个复合数据类型，例如一个[结构体](Structs <a href="http://xn--tqqq30dppq.md">结构体.md</a>)，你可以在模式中解构它：</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">Point</span></span> {
    x: <span class="hljs-built_in">i32</span>,
    y: <span class="hljs-built_in">i32</span>,
}

<span class="hljs-keyword">let</span> origin = Point { x: <span class="hljs-number">0</span>, y: <span class="hljs-number">0</span> };

<span class="hljs-keyword">match</span> origin {
    Point { x, y } =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"({},{})"</span>, x, y),
}
</div></code></pre>
<p>我们可以用<code>:</code>来给出一个不同的名字：</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">Point</span></span> {
    x: <span class="hljs-built_in">i32</span>,
    y: <span class="hljs-built_in">i32</span>,
}

<span class="hljs-keyword">let</span> origin = Point { x: <span class="hljs-number">0</span>, y: <span class="hljs-number">0</span> };

<span class="hljs-keyword">match</span> origin {
    Point { x: x1, y: y1 } =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"({},{})"</span>, x1, y1),
}
</div></code></pre>
<p>如果你只关心部分值，我们不需要给它们都命名：</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">Point</span></span> {
    x: <span class="hljs-built_in">i32</span>,
    y: <span class="hljs-built_in">i32</span>,
}

<span class="hljs-keyword">let</span> origin = Point { x: <span class="hljs-number">0</span>, y: <span class="hljs-number">0</span> };

<span class="hljs-keyword">match</span> origin {
    Point { x, .. } =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x is {}"</span>, x),
}
</div></code></pre>
<p>这会输出<code>x is 0</code>。</p>
<p>你可以对任何成员进行这样的匹配，不仅仅是第一个：</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">Point</span></span> {
    x: <span class="hljs-built_in">i32</span>,
    y: <span class="hljs-built_in">i32</span>,
}

<span class="hljs-keyword">let</span> origin = Point { x: <span class="hljs-number">0</span>, y: <span class="hljs-number">0</span> };

<span class="hljs-keyword">match</span> origin {
    Point { y, .. } =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"y is {}"</span>, y),
}
</div></code></pre>
<p>这会输出<code>y is 0</code>。</p>
<p>这种“解构”行为可以用在任何复合数据类型上，例如[元组](Primitive Types <a href="http://xn--8mr24g9v7alrg.md#%E5%85%83%E7%BB%84%EF%BC%88tuples%EF%BC%89">原生类型.md#元组（tuples）</a>)和[枚举](Enums <a href="http://xn--wiq013d.md">枚举.md</a>)</p>
<h2 id="%E5%BF%BD%E7%95%A5%E7%BB%91%E5%AE%9A%EF%BC%88ignoring-bindings%EF%BC%89">忽略绑定（Ignoring bindings）</h2>
<p>你可以在模式中使用<code>_</code>来忽视它的类型和值。例如，这是一个<code>Result&lt;T, E&gt;</code>的<code>match</code>：</p>
<pre class="hljs"><code><div># <span class="hljs-keyword">let</span> some_value: <span class="hljs-built_in">Result</span>&lt;<span class="hljs-built_in">i32</span>, &amp;<span class="hljs-symbol">'static</span> <span class="hljs-built_in">str</span>&gt; = <span class="hljs-literal">Err</span>(<span class="hljs-string">"There was an error"</span>);
<span class="hljs-keyword">match</span> some_value {
    <span class="hljs-literal">Ok</span>(value) =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"got a value: {}"</span>, value),
    <span class="hljs-literal">Err</span>(_) =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"an error occurred"</span>),
}
</div></code></pre>
<p>在第一个分支，我们绑定了<code>Ok</code>变量中的值为<code>value</code>，不过在<code>Err</code>分支，我们用<code>_</code>来忽视特定的错误，而只是打印了一个通用的错误信息。</p>
<p><code>_</code>在任何创建绑定的模式中都有效。这在忽略一个大大结构体的部分字段时很有用：</p>
<pre class="hljs"><code><div><span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">coordinate</span></span>() -&gt; (<span class="hljs-built_in">i32</span>, <span class="hljs-built_in">i32</span>, <span class="hljs-built_in">i32</span>) {
    <span class="hljs-comment">// generate and return some sort of triple tuple</span>
# (<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>)
}

<span class="hljs-keyword">let</span> (x, _, z) = coordinate();
</div></code></pre>
<p>这里，我们绑定元组第一个和最后一个元素为<code>x</code>和<code>z</code>，不过省略了中间的元素。</p>
<p>相似的，你可以在模式中用<code>..</code>来忽略多个值。</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">enum</span> <span class="hljs-title">OptionalTuple</span></span> {
    Value(<span class="hljs-built_in">i32</span>, <span class="hljs-built_in">i32</span>, <span class="hljs-built_in">i32</span>),
    Missing,
}

<span class="hljs-keyword">let</span> x = OptionalTuple::Value(<span class="hljs-number">5</span>, -<span class="hljs-number">2</span>, <span class="hljs-number">3</span>);

<span class="hljs-keyword">match</span> x {
    OptionalTuple::Value(..) =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Got a tuple!"</span>),
    OptionalTuple::Missing =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"No such luck."</span>),
}
</div></code></pre>
<p>这会打印<code>Got a tuple!</code>。</p>
<h2 id="ref%E5%92%8Cref-mut"><code>ref</code>和<code>ref mut</code></h2>
<p>模式匹配命中的时候，未实现<code>Copy</code>的类型会被默认的move掉，因此，原owner就不再持有其所有权。但是有些时候，我们只想要从中拿到一个变量的（可变）引用，而不想将其move出作用域，用<code>ref</code>或者<code>ref mut</code>。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">match</span> x {
    <span class="hljs-keyword">ref</span> r =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Got a reference to {}"</span>, r),
}
</div></code></pre>
<p>这会输出<code>Got a reference to 5</code>。</p>
<p>这里，<code>match</code>中的<code>r</code>是<code>&amp;i32</code>类型的。换句话说，<code>ref</code>关键字创建了一个在模式中使用的引用。如果你需要一个可变引用，<code>ref mut</code>同样可以做到：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">match</span> x {
    <span class="hljs-keyword">ref</span> <span class="hljs-keyword">mut</span> mr =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Got a mutable reference to {}"</span>, mr),
}
</div></code></pre>
<h2 id="%E8%8C%83%E5%9B%B4%EF%BC%88ranges%EF%BC%89">范围（Ranges）</h2>
<p>你可以用<code>...</code>匹配一个范围的值：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">1</span>;

<span class="hljs-keyword">match</span> x {
    <span class="hljs-number">1</span> ... <span class="hljs-number">5</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"one through five"</span>),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"anything"</span>),
}
</div></code></pre>
<p>这会输出<code>one through five</code>。</p>
<p>范围经常用在整数和<code>char</code>上。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = '💅';

<span class="hljs-keyword">match</span> x {
    <span class="hljs-string">'a'</span> ... <span class="hljs-string">'j'</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"early letter"</span>),
    <span class="hljs-string">'k'</span> ... <span class="hljs-string">'z'</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"late letter"</span>),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"something else"</span>),
}
</div></code></pre>
<p>这会输出<code>something else</code>。</p>
<h2 id="%E7%BB%91%E5%AE%9A">绑定</h2>
<p>你可以使用<code>@</code>把值绑定到名字上：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">1</span>;

<span class="hljs-keyword">match</span> x {
    e @ <span class="hljs-number">1</span> ... <span class="hljs-number">5</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"got a range element {}"</span>, e),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"anything"</span>),
}
</div></code></pre>
<p>这会输出<code>got a range element 1</code>。在你想对一个复杂数据结构进行部分匹配的时候，这个特性十分有用：</p>
<pre class="hljs"><code><div><span class="hljs-meta">#[derive(Debug)]</span>
<span class="hljs-class"><span class="hljs-keyword">struct</span> <span class="hljs-title">Person</span></span> {
    name: <span class="hljs-built_in">Option</span>&lt;<span class="hljs-built_in">String</span>&gt;,
}

<span class="hljs-keyword">let</span> name = <span class="hljs-string">"Steve"</span>.to_string();
<span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> x: <span class="hljs-built_in">Option</span>&lt;Person&gt; = <span class="hljs-literal">Some</span>(Person { name: <span class="hljs-literal">Some</span>(name) });
<span class="hljs-keyword">match</span> x {
    <span class="hljs-literal">Some</span>(Person { name: <span class="hljs-keyword">ref</span> a @ <span class="hljs-literal">Some</span>(_), .. }) =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{:?}"</span>, a),
    _ =&gt; {}
}
</div></code></pre>
<p>这会输出 <code>Some(&quot;Steve&quot;)</code>，因为我们把Person里面的<code>name</code>绑定到<code>a</code>。</p>
<p>如果你在使用<code>|</code>的同时也使用了<code>@</code>，你需要确保名字在每个模式的每一部分都绑定名字：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">match</span> x {
    e @ <span class="hljs-number">1</span> ... <span class="hljs-number">5</span> | e @ <span class="hljs-number">8</span> ... <span class="hljs-number">10</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"got a range element {}"</span>, e),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"anything"</span>),
}
</div></code></pre>
<h2 id="%E5%AE%88%E5%8D%AB%EF%BC%88guards%EF%BC%89">守卫（Guards）</h2>
<p>你可以用<code>if</code>来引入<em>匹配守卫</em>（<em>match guards</em>）：</p>
<pre class="hljs"><code><div><span class="hljs-class"><span class="hljs-keyword">enum</span> <span class="hljs-title">OptionalInt</span></span> {
    Value(<span class="hljs-built_in">i32</span>),
    Missing,
}

<span class="hljs-keyword">let</span> x = OptionalInt::Value(<span class="hljs-number">5</span>);

<span class="hljs-keyword">match</span> x {
    OptionalInt::Value(i) <span class="hljs-keyword">if</span> i &gt; <span class="hljs-number">5</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Got an int bigger than five!"</span>),
    OptionalInt::Value(..) =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Got an int!"</span>),
    OptionalInt::Missing =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"No such luck."</span>),
}
</div></code></pre>
<p>这会输出<code>Got an int!</code>。</p>
<p>如果你在<code>if</code>中使用多重模式，<code>if</code>条件将适用于所有模式：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">4</span>;
<span class="hljs-keyword">let</span> y = <span class="hljs-literal">false</span>;

<span class="hljs-keyword">match</span> x {
    <span class="hljs-number">4</span> | <span class="hljs-number">5</span> <span class="hljs-keyword">if</span> y =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"yes"</span>),
    _ =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"no"</span>),
}
</div></code></pre>
<p>这会打印<code>no</code>，因为<code>if</code>适用于整个<code>4 | 5</code>，而不仅仅是<code>5</code>，换句话说，<code>if</code>语句的优先级是这样的：</p>
<pre><code class="language-text">(4 | 5) if y => ...
</code></pre>
<p>而不是这样：</p>
<pre><code class="language-text">4 | (5 if y) => ...
</code></pre>
<h2 id="%E6%B7%B7%E5%90%88%E4%B8%8E%E5%8C%B9%E9%85%8D%EF%BC%88mix-and-match%EF%BC%89">混合与匹配（Mix and Match）</h2>
<p>(口哨)！根据你的需求，你可以对上面的多种匹配方法进行组合：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">match</span> x {
    Foo { x: <span class="hljs-literal">Some</span>(<span class="hljs-keyword">ref</span> name), y: <span class="hljs-literal">None</span> } =&gt; ...
}
</div></code></pre>
<p>模式十分强大。好好使用它们。</p>
<h1 id="if-let">if let</h1>
<p><code>if let</code>允许你合并<code>if</code>和<code>let</code>来减少特定类型模式匹配的开销。</p>
<p>例如，让我们假设我们有一些<code>Option&lt;T&gt;</code>。我们想让它是<code>Some&lt;T&gt;</code>时在其上调用一个函数，而它是<code>None</code>时什么也不做。这看起来像：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> option = <span class="hljs-literal">Some</span>(<span class="hljs-number">5</span>);
<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">foo</span></span>(x: <span class="hljs-built_in">i32</span>) { }
<span class="hljs-keyword">match</span> option {
    <span class="hljs-literal">Some</span>(x) =&gt; { foo(x) },
    <span class="hljs-literal">None</span> =&gt; {},
}
</div></code></pre>
<p>我们并不一定要在这使用<code>match</code>，例如，我们可以使用<code>if</code>：</p>
<pre class="hljs"><code><div># <span class="hljs-keyword">let</span> option = <span class="hljs-literal">Some</span>(<span class="hljs-number">5</span>);
# <span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">foo</span></span>(x: <span class="hljs-built_in">i32</span>) { }
<span class="hljs-keyword">if</span> option.is_some() {
    <span class="hljs-keyword">let</span> x = option.unwrap();
    foo(x);
}
</div></code></pre>
<p>这两种选项都不是特别吸引人。我们可以使用<code>if let</code>来优雅地完成相同的功能：</p>
<pre class="hljs"><code><div># <span class="hljs-keyword">let</span> option = <span class="hljs-literal">Some</span>(<span class="hljs-number">5</span>);
# <span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">foo</span></span>(x: <span class="hljs-built_in">i32</span>) { }
<span class="hljs-keyword">if</span> <span class="hljs-keyword">let</span> <span class="hljs-literal">Some</span>(x) = option {
    foo(x);
}
</div></code></pre>
<p>如果一个[模式](Patterns <a href="http://xn--w0t69v.md">模式.md</a>)匹配成功，它绑定任何值的合适的部分到模式的标识符中，并计算这个表达式。如果模式不匹配，啥也不会发生。</p>
<p>如果你想在模式不匹配时做点其他的，你可以使用<code>else</code>：</p>
<pre class="hljs"><code><div># <span class="hljs-keyword">let</span> option = <span class="hljs-literal">Some</span>(<span class="hljs-number">5</span>);
# <span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">foo</span></span>(x: <span class="hljs-built_in">i32</span>) { }
# <span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">bar</span></span>() { }
<span class="hljs-keyword">if</span> <span class="hljs-keyword">let</span> <span class="hljs-literal">Some</span>(x) = option {
    foo(x);
} <span class="hljs-keyword">else</span> {
    bar();
}
</div></code></pre>
<h2 id="while-let"><code>while let</code></h2>
<p>类似的，当你想一直循环，直到一个值匹配到特定的模式的时候，你可以选择使用<code>while let</code>。使用<code>while let</code>可以把类似这样的代码：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> v = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">3</span>, <span class="hljs-number">5</span>, <span class="hljs-number">7</span>, <span class="hljs-number">11</span>];
<span class="hljs-keyword">loop</span> {
    <span class="hljs-keyword">match</span> v.pop() {
        <span class="hljs-literal">Some</span>(x) =&gt;  <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x),
        <span class="hljs-literal">None</span> =&gt; <span class="hljs-keyword">break</span>,
    }
}
</div></code></pre>
<p>变成这样的代码：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> v = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">3</span>, <span class="hljs-number">5</span>, <span class="hljs-number">7</span>, <span class="hljs-number">11</span>];
<span class="hljs-keyword">while</span> <span class="hljs-keyword">let</span> <span class="hljs-literal">Some</span>(x) = v.pop() {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x);
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
  name: 'pattern',
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