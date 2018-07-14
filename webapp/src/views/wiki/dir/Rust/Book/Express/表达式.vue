<template>
  <div  id="express">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
                  <h3 id="%E8%A1%A8%E8%BE%BE%E5%BC%8F-%E8%AF%AD%E5%8F%A5">表达式-语句</h3>
<p>Rust 主要是一个基于表达式的语言。只有两种语句，其它的一切都是表达式。</p>
<p>表达式返回一个值，而语句不是。这就是为什么这里我们以“不是所有控制路径都返回一个值”。Rust 中有两种类型的语句：“声明语句”和“表达式语句”。其余的一切是表达式。</p>
<p>在 Rust 中，使用<code>let</code>引入一个绑定并<em>不是</em>一个表达式。下面的代码会产生一个编译时错误：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = (<span class="hljs-keyword">let</span> y = <span class="hljs-number">5</span>); <span class="hljs-comment">// expected identifier, found keyword `let`</span>
</div></code></pre>
<p>编译器告诉我们这里它期望看到表达式的开头，而<code>let</code>只能开始一个语句，不是一个表达式。</p>
<p>注意赋值一个已经绑定过的变量（例如，<code>y = 5</code>）仍是一个表达式，即使它的（返回）值并不是特别有用。不像其它语言中赋值语句返回它赋的值（例如，前面例子中的<code>5</code>），在 Rust 中赋值的值是一个空的元组<code>()</code>：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> y = <span class="hljs-number">5</span>;

<span class="hljs-keyword">let</span> x = (y = <span class="hljs-number">6</span>);  <span class="hljs-comment">// x has the value `()`, not `6`</span>
</div></code></pre>
<p>Rust中第二种语句是<strong>表达式语句</strong>。它的目的是把任何表达式变为语句。Rust 语法期望语句后跟其它语句。用分号来分隔各个表达式.</p>
<h2 id="%E6%8E%A7%E5%88%B6%E7%BB%93%E6%9E%84">控制结构</h2>
<ul>
<li>return</li>
<li>if/if let</li>
<li>while/while let</li>
<li>loop</li>
<li>for</li>
<li>label</li>
<li>break 与 continue</li>
<li>match</li>
</ul>
<h3 id="return">return</h3>
<pre class="hljs"><code><div>
<span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">max</span></span>(a: <span class="hljs-built_in">i32</span>, b: <span class="hljs-built_in">i32</span>) -&gt; <span class="hljs-built_in">i32</span> {
    <span class="hljs-keyword">if</span> a &gt; b {
        <span class="hljs-keyword">return</span> a;
    }
    <span class="hljs-keyword">return</span> b;
}
</div></code></pre>
<h3 id="if">If</h3>
<p><code>if</code> 语句是<em>分支</em>这个更加宽泛的概念的一个特定形式。
Rust 的 if 表达式的显著特点是：1,判断条件不用小括号括起来；2,它是表达式，而不是语句。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">if</span> x == <span class="hljs-number">5</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x is five!"</span>);
}
</div></code></pre>
<p>如果<code>if</code>后面的表达式的值为<code>true</code>，这个代码块将被执行。为<code>false</code>则不被执行。</p>
<p>如果你想当值为<code>false</code>时执行些什么，使用<code>else</code>：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">if</span> x == <span class="hljs-number">5</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x is five!"</span>);
} <span class="hljs-keyword">else</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x is not five :("</span>);
}
</div></code></pre>
<p>如果不止一种情况，使用<code>else if</code>：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">if</span> x == <span class="hljs-number">5</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x is five!"</span>);
} <span class="hljs-keyword">else</span> <span class="hljs-keyword">if</span> x == <span class="hljs-number">6</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x is six!"</span>);
} <span class="hljs-keyword">else</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x is not five or six :("</span>);
}
</div></code></pre>
<p>你可以（或许也应该）这么写：</p>

<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">let</span> y = <span class="hljs-keyword">if</span> x == <span class="hljs-number">5</span> { <span class="hljs-number">10</span> } <span class="hljs-keyword">else</span> { <span class="hljs-number">15</span> }; <span class="hljs-comment">// y: i32</span>
</div></code></pre>
<p>这代码可以被执行是因为<code>if</code>是一个表达式。表达式的值是任何被选择的分支的最后一个表达式的值。一个没有<code>else</code>的<code>if</code>总是返回<code>()</code>作为返回值。</p>
<p><strong>if let模式</strong> : match 的简化用法</p>
<pre class="hljs"><code><div>
<span class="hljs-keyword">let</span> dish = (<span class="hljs-string">"Ham"</span>, <span class="hljs-string">"Eggs"</span>);

<span class="hljs-comment">// this body will be skipped because the pattern is refuted</span>
<span class="hljs-keyword">if</span> <span class="hljs-keyword">let</span> (<span class="hljs-string">"Bacon"</span>, b) = dish {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Bacon is served with {}"</span>, b);
} <span class="hljs-keyword">else</span> {
    <span class="hljs-comment">// This block is evaluated instead.</span>
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"No bacon will be served"</span>);
}

<span class="hljs-comment">// this body will execute</span>
<span class="hljs-keyword">if</span> <span class="hljs-keyword">let</span> (<span class="hljs-string">"Ham"</span>, b) = dish {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Ham is served with {}"</span>, b);
}
</div></code></pre>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> x = <span class="hljs-literal">Some</span>(<span class="hljs-number">3</span>);
<span class="hljs-keyword">let</span> a = <span class="hljs-keyword">if</span> <span class="hljs-keyword">let</span> <span class="hljs-literal">Some</span>(<span class="hljs-number">1</span>) = x {
    <span class="hljs-number">1</span>
} <span class="hljs-keyword">else</span> <span class="hljs-keyword">if</span> x == <span class="hljs-literal">Some</span>(<span class="hljs-number">2</span>) {
    <span class="hljs-number">2</span>
} <span class="hljs-keyword">else</span> <span class="hljs-keyword">if</span> <span class="hljs-keyword">let</span> <span class="hljs-literal">Some</span>(y) = x {
    y
} <span class="hljs-keyword">else</span> {
    -<span class="hljs-number">1</span>
};
<span class="hljs-built_in">assert_eq!</span>(a, <span class="hljs-number">3</span>);
</div></code></pre>
<p><strong>Rust 目前提供一些迭代操作。他们是<code>loop</code>，<code>while</code>和<code>for</code>。每种方法都有自己的用途</strong></p>
<h3 id="loop">loop</h3>
<p>无限<code>loop</code>是 Rust 提供的最简单的循环。使用<code>loop</code>关键字。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">loop</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Loop forever!"</span>);
}
</div></code></pre>
<h3 id="while">while</h3>
<p>Rust 也有一个<code>while</code>循环， <code>while</code>循环是当你不确定应该循环多少次时正确的选择。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> x = <span class="hljs-number">5</span>; <span class="hljs-comment">// mut x: i32</span>
<span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> done = <span class="hljs-literal">false</span>; <span class="hljs-comment">// mut done: bool</span>

<span class="hljs-keyword">while</span> !done {
    x += （x - <span class="hljs-number">3</span>）;

    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x);

    <span class="hljs-keyword">if</span> x % <span class="hljs-number">5</span> == <span class="hljs-number">0</span> {
        done = <span class="hljs-literal">true</span>;
    }
}
</div></code></pre>
<p><strong>while let</strong>模式</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> x = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>];

<span class="hljs-keyword">while</span> <span class="hljs-keyword">let</span> <span class="hljs-literal">Some</span>(y) = x.pop() {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"y = {}"</span>, y);
}
</div></code></pre>
<h3 id="for">for</h3>
<p><code>for</code>用来循环一个特定的次数。</p>

<pre class="hljs"><code><div><span class="hljs-keyword">for</span> x <span class="hljs-keyword">in</span> <span class="hljs-number">0</span>..<span class="hljs-number">10</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x); <span class="hljs-comment">// x: i32</span>
}

<span class="hljs-keyword">let</span> v = &amp;[<span class="hljs-string">"apples"</span>, <span class="hljs-string">"cake"</span>, <span class="hljs-string">"coffee"</span>];

<span class="hljs-keyword">for</span> text <span class="hljs-keyword">in</span> v {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"I like {}."</span>, text);
}
</div></code></pre>
<p>更抽象的形式：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">for</span> var <span class="hljs-keyword">in</span> expression {
    code
}
</div></code></pre>
<p>这个表达式是一个[迭代器](Iterators <a href="http://xn--mnq26pdo6c.md">迭代器.md</a>).迭代器返回一系列的元素。<code>0..10</code>表达式取一个开始和结束的位置，然后给出一个含有这之间值得迭代器。它不包括上限值，所以我们的循环会打印<code>0</code>到<code>9</code>。</p>
<h2 id="enumerate%E6%96%B9%E6%B3%95">Enumerate方法</h2>
<p>当你需要记录你已经循环了多少次了的时候，你可以使用<code>.enumerate()</code>函数。</p>
<h3 id="%E5%AF%B9%E8%8C%83%E5%9B%B4%EF%BC%88on-ranges%EF%BC%89%EF%BC%9A">对范围（On ranges）：</h3>
<pre class="hljs"><code><div><span class="hljs-keyword">for</span> (i,j) <span class="hljs-keyword">in</span> (<span class="hljs-number">5</span>..<span class="hljs-number">10</span>).enumerate() {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"i = {} and j = {}"</span>, i, j);
}
</div></code></pre>
<p>输出：</p>
<pre><code class="language-text">i = 0 and j = 5
i = 1 and j = 6
i = 2 and j = 7
i = 3 and j = 8
i = 4 and j = 9
</code></pre>
<h3 id="%E5%AF%B9%E8%BF%AD%E4%BB%A3%E5%99%A8%EF%BC%88on-iterators%EF%BC%89">对迭代器（On iterators）:</h3>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> lines = <span class="hljs-string">"hello\nworld"</span>.lines();
<span class="hljs-keyword">for</span> (linenumber, line) <span class="hljs-keyword">in</span> lines.enumerate() {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}: {}"</span>, linenumber, line);
}
</div></code></pre>
<p>输出：</p>
<pre><code class="language-text">0: hello
1: world
</code></pre>
<h2 id="%E6%8F%90%E6%97%A9%E7%BB%93%E6%9D%9F%E8%BF%AD%E4%BB%A3">提早结束迭代</h2>
<p>让我们再看一眼之前的<code>while</code>循环：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> x = <span class="hljs-number">5</span>;
<span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> done = <span class="hljs-literal">false</span>;

<span class="hljs-keyword">while</span> !done {
    x += x - <span class="hljs-number">3</span>;

    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x);

    <span class="hljs-keyword">if</span> x % <span class="hljs-number">5</span> == <span class="hljs-number">0</span> {
        done = <span class="hljs-literal">true</span>;
    }
}
</div></code></pre>
<p>我们必须使用一个<code>mut</code>布尔型变量绑定，<code>done</code>,来确定何时我们应该推出循环。</p>
<p><strong>Rust 有两个关键字帮助我们来修改迭代：<code>break</code>和<code>continue</code></strong></p>
<ul>
<li>break 用来跳出当前层的循环；</li>
<li>continue 用来执行当前层的下一次迭代。</li>
</ul>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> x = <span class="hljs-number">5</span>;

<span class="hljs-keyword">loop</span> {
    x += x - <span class="hljs-number">3</span>;

    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x);

    <span class="hljs-keyword">if</span> x % <span class="hljs-number">5</span> == <span class="hljs-number">0</span> { <span class="hljs-keyword">break</span>; }
}
</div></code></pre>
<p>现在我们用<code>loop</code>来无限循环，然后用<code>break</code>来提前退出循环。</p>
<p><code>continue</code>比较类似，不过不是退出循环，它直接进行下一次迭代。下面的例子只会打印奇数：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">for</span> x <span class="hljs-keyword">in</span> <span class="hljs-number">0</span>..<span class="hljs-number">10</span> {
    <span class="hljs-keyword">if</span> x % <span class="hljs-number">2</span> == <span class="hljs-number">0</span> { <span class="hljs-keyword">continue</span>; }

    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x);
}
</div></code></pre>
<p><code>break</code>和<code>continue</code>在<code>while</code>循环和<a href="#for"><code>for</code>循环</a>中都有效。</p>
<h2 id="%E5%BE%AA%E7%8E%AF%E6%A0%87%E7%AD%BE%EF%BC%88loop-labels%EF%BC%89">循环标签（Loop labels）</h2>
<p>你也许会遇到这样的情形，当你有嵌套的循环而希望指定你的哪一个<code>break</code>或<code>continue</code>该起作用。就像大多数语言，默认<code>break</code>或<code>continue</code>将会作用于最内层的循环。当你想要一个<code>break</code>或<code>continue</code>作用于一个外层循环，你可以使用标签来指定你的<code>break</code>或<code>continue</code>语句作用的循环。如下代码只会在<code>x</code>和<code>y</code>都为奇数时打印他们：</p>
<pre class="hljs"><code><div><span class="hljs-symbol">'outer</span>: <span class="hljs-keyword">for</span> x <span class="hljs-keyword">in</span> <span class="hljs-number">0</span>..<span class="hljs-number">10</span> {
    <span class="hljs-symbol">'inner</span>: <span class="hljs-keyword">for</span> y <span class="hljs-keyword">in</span> <span class="hljs-number">0</span>..<span class="hljs-number">10</span> {
        <span class="hljs-keyword">if</span> x % <span class="hljs-number">2</span> == <span class="hljs-number">0</span> { <span class="hljs-keyword">continue</span> <span class="hljs-symbol">'outer</span>; } <span class="hljs-comment">// continues the loop over x</span>
        <span class="hljs-keyword">if</span> y % <span class="hljs-number">2</span> == <span class="hljs-number">0</span> { <span class="hljs-keyword">continue</span> <span class="hljs-symbol">'inner</span>; } <span class="hljs-comment">// continues the loop over y</span>
        <span class="hljs-built_in">println!</span>(<span class="hljs-string">"x: {}, y: {}"</span>, x, y);
    }
}
</div></code></pre>
<h2 id="%E8%BF%AD%E4%BB%A3%E5%99%A8">迭代器</h2>
<table>
<thead>
<tr>
<th>Production</th>
<th>Syntax</th>
<th>Type</th>
<th>Range</th>
</tr>
</thead>
<tbody>
<tr>
<td>RangeExpr</td>
<td>start..end</td>
<td>std::ops::Range</td>
<td>start ≤ x &lt; end</td>
</tr>
<tr>
<td>RangeFromExpr</td>
<td>start..</td>
<td>std::ops::RangeFrom</td>
<td>start ≤ x</td>
</tr>
<tr>
<td>RangeToExpr</td>
<td>..end</td>
<td>std::ops::RangeTo</td>
<td>x &lt; end</td>
</tr>
<tr>
<td>RangeFullExpr</td>
<td>..</td>
<td>std::ops::RangeFull</td>
<td>-</td>
</tr>
<tr>
<td>RangeInclusiveExpr</td>
<td>start..=end</td>
<td>std::ops::RangeInclusive</td>
<td>start ≤ x ≤ end</td>
</tr>
<tr>
<td>RangeToInclusiveExpr</td>
<td>..=end</td>
<td>std::ops::RangeToInclusive</td>
<td>x ≤ end</td>
</tr>
</tbody>
</table>
<p>范围（<code>0..10</code>）是“迭代器”。我们可以重复调用迭代器的<code>.next()</code>方法，然后它会给我们一个数据序列。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> <span class="hljs-keyword">mut</span> range = <span class="hljs-number">0</span>..<span class="hljs-number">10</span>;

<span class="hljs-keyword">loop</span> {
    <span class="hljs-keyword">match</span> range.next() {
        <span class="hljs-literal">Some</span>(x) =&gt; {
            <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x);
        },
        <span class="hljs-literal">None</span> =&gt; { <span class="hljs-keyword">break</span> }
    }
}
</div></code></pre>

<p>我们创建了一个<code>range</code>的可变绑定，它是我们的迭代器。我们接着<code>loop</code>，它包含一个<code>match</code>。<code>match</code>用来匹配<code>range.next()</code>的结果，它给我们迭代器的下一个值。<code>next</code>返回一个<code>Option&lt;i32&gt;</code>，在这个例子中，如果有值,它会返回<code>Some(i32)</code>然后当我们循环完毕,就会返回<code>None</code>。如果我们得到<code>Some(i32)</code>，我们就会打印它，如果我们得到<code>None</code>，我们<code>break</code>出循环。</p>
<p>这个代码例子基本上和我们的<code>loop</code>版本一样。<code>for</code>只是<code>loop</code>/<code>match</code>/<code>break</code>结构的简便写法。</p>
<p>然而，<code>for</code>循环并不是唯一使用迭代器的结构。编写你自己的迭代器涉及到实现<code>Iterator</code>特性。然而特性不是本章教程的涉及范围，不过Rust提供了一系列的有用的迭代器帮助我们完成各种任务。但首先注意下<em>范围</em> 的一些局限性。</p>
<p><em>范围</em> 非常原始，我们通常可以用更好的替代方案。考虑下面的 Rust 反模式：用<em>范围</em> 来模拟 C-风格的<code>for</code>循环。比如你想遍历完 vector 的内容。你可能尝试这么写：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> nums = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>];

<span class="hljs-keyword">for</span> i <span class="hljs-keyword">in</span> <span class="hljs-number">0</span>..nums.len() {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, nums[i]);
}
</div></code></pre>
<p>这严格的说比使用现成的迭代器还要糟。你可以直接在 vector 上遍历。所以这么写：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> nums = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>];

<span class="hljs-keyword">for</span> num <span class="hljs-keyword">in</span> &amp;nums {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, num);
}
</div></code></pre>
<p>这么写有两个原因。第一，它更明确的表明了我们的意图。我们迭代整个向量，而不是先迭代向量的索引，再按索引迭代向量。第二，这个版本也更有效率：第一个版本会进行额外的边界检查因为它使用了索引，<code>nums[i]</code>。因为我们利用迭代器获取每个向量元素的引用，第二个例子中并没有边界检查。这在迭代器中非常常见：我们可以忽略不必要的边界检查，不过仍然知道我们是安全的。</p>
<p>这里还有一个细节不是100%清楚的就是<code>println!</code>是如何工作的。<code>num</code>是<code>&amp;i32</code>类型。也就是说，它是一个<code>i32</code>的引用，并不是<code>i32</code>本身。<code>println!</code>为我们处理了解引用，所以我们并没有看到它。下面的代码也能工作：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> nums = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>];

<span class="hljs-keyword">for</span> num <span class="hljs-keyword">in</span> &amp;nums {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, *num);
}
</div></code></pre>
<p>现在我们显式的解引用了<code>num</code>。为什么<code>&amp;nums</code>会给我们一个引用呢？首先，因为我们显式的使用了<code>&amp;</code>。再次，如果它给我们数据，我们就是它的所有者了，这会涉及到生成数据的拷贝然后返回给我们拷贝。通过引用，我们只是借用了一个数据的引用，所以仅仅是传递了一个引用，并不涉及数据的移动。</p>
<p>那么，既然现在我们已经明确了范围通常不是我们需要的，让我们来讨论下你需要什么。</p>
<p>这里涉及到大体上相关的3类事物：迭代器，<em>迭代适配器</em>（<em>iterator adapters</em>）和<em>消费者</em>（<em>consumers</em>）。下面是一些定义：</p>
<ul>
<li><em>迭代器</em> 给你一个值的序列</li>
<li><em>迭代适配器</em> 操作迭代器，产生一个不同输出序列的新迭代器</li>
<li><em>消费者</em> 操作迭代器，产生最终值的集合</li>
</ul>
<p>让我们先看看消费者，因为我们已经见过范围这个迭代器了。</p>
<h2 id="%E6%B6%88%E8%B4%B9%E8%80%85%EF%BC%88consumers%EF%BC%89">消费者（Consumers）</h2>
<p><em>消费者</em> 操作一个迭代器，返回一些值或者几种类型的值。最常见的消费者是<code>collect()</code>。这个代码还不能编译，不过它表明了我们的意图：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> one_to_one_hundred = (<span class="hljs-number">1</span>..<span class="hljs-number">101</span>).collect();
</div></code></pre>
<p>如你所见，我们在迭代器上调用了<code>collect()</code>。<code>collect()</code>从迭代器中取得尽可能多的值，然后返回结果的集合。那么为什么这不能编译呢？因为Rust不能确定你想收集什么类型的值，所以你需要让它知道。下面是一个可以编译的版本：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> one_to_one_hundred = (<span class="hljs-number">1</span>..<span class="hljs-number">101</span>).collect::&lt;<span class="hljs-built_in">Vec</span>&lt;<span class="hljs-built_in">i32</span>&gt;&gt;();
</div></code></pre>
<p>如果你还记得，<code>::&lt;&gt;</code>语法允许我们给出一个类型提示，所以我们可以告诉编译器我们需要一个整型的向量。但是你并不总是需要提供完整的类型。使用<code>_</code>可以让你提供一个部分的提示：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> one_to_one_hundred = (<span class="hljs-number">1</span>..<span class="hljs-number">101</span>).collect::&lt;<span class="hljs-built_in">Vec</span>&lt;_&gt;&gt;();
</div></code></pre>
<p>这是指“请把值收集到<code>Vec&lt;T&gt;</code>，不过自行推断<code>T</code>类型”。为此<code>_</code>有时被称为“类型占位符”。</p>
<p><code>collect()</code>是最常见的消费者，不过这还有其它的消费者。<code>find()</code>就是一个：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> greater_than_forty_two = (<span class="hljs-number">0</span>..<span class="hljs-number">100</span>)
                             .find(|x| *x &gt; <span class="hljs-number">42</span>);

<span class="hljs-keyword">match</span> greater_than_forty_two {
    <span class="hljs-literal">Some</span>(_) =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"Found a match!"</span>),
    <span class="hljs-literal">None</span> =&gt; <span class="hljs-built_in">println!</span>(<span class="hljs-string">"No match found :("</span>),
}
</div></code></pre>
<p><code>find</code>接收一个闭包，然后处理迭代器中每个元素的引用。如果这个元素是我们要找的，那么这个闭包返回<code>true</code>，如果不是就返回<code>false</code>。因为我们可能不能找到任何元素，所以<code>find</code>返回<code>Option</code>而不是元素本身。</p>
<p>另一个重要的消费者是<code>fold</code>。他看起来像这样：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> sum = (<span class="hljs-number">1</span>..<span class="hljs-number">4</span>).fold(<span class="hljs-number">0</span>, |sum, x| sum + x);
</div></code></pre>
<p><code>fold()</code>看起来像这样：<code>fold(base, |accumulator, element| ...)</code>。它需要两个参数：第一个参数叫做<em>基数</em>（<em>base</em>）。第二个是一个闭包，它自己也需要两个参数：第一个叫做<em>累计数</em>（<em>accumulator</em>），第二个叫<em>元素</em>（<em>element</em>）。每次迭代，这个闭包都会被调用，返回值是下一次迭代的累计数。在我们的第一次迭代，基数是累计数。</p>
<p>好吧，这有点混乱。让我们检查一下这个迭代器中所有这些值：</p>
<table>
<thead>
<tr>
<th>基数</th>
<th>累计数</th>
<th>元素</th>
<th>闭包结果</th>
</tr>
</thead>
<tbody>
<tr>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
</tr>
<tr>
<td>0</td>
<td>1</td>
<td>2</td>
<td>3</td>
</tr>
<tr>
<td>0</td>
<td>3</td>
<td>3</td>
<td>6</td>
</tr>
</tbody>
</table>
<p>我们可以使用这些参数调用<code>fold()</code>：</p>

<pre class="hljs"><code><div>#(<span class="hljs-number">1</span>..<span class="hljs-number">4</span>)
.fold(<span class="hljs-number">0</span>, |sum, x| sum + x);
</div></code></pre>
<p>那么，<code>0</code>是我们的基数，<code>sum</code>是累计数，<code>x</code>是元素。在第一次迭代，我们设置<code>sum</code>为<code>0</code>，然后<code>x</code>是<code>nums</code>的第一个元素，<code>1</code>。我们接着把<code>sum</code>和<code>x</code>相加，得到<code>0 + 1 = 1</code>。在我们第二次迭代，<code>sum</code>成为我们的累计值，元素是数组的第二个值，<code>2</code>，<code>1 + 2 = 3</code>，然后它就是最后一次迭代的累计数。在这次迭代中，<code>x</code>是最后的元素，<code>3</code>，那么<code>3 + 3 = 6</code>，就是我们和的最终值。<code>1 + 2 + 3 = 6</code>，这就是我们的结果。</p>
<p>（口哨）。最开始你见到<code>fold</code>的时候可能觉得有点奇怪，不过一旦你习惯了它，你就会在到处都用它。任何时候你有一个列表，然后你需要一个单一的结果，<code>fold</code>就是合适的。</p>
<p>消费者很重要还因为另一个我们没有讨论到的迭代器的属性：惰性。让我们更多的讨论一下迭代器，你就知道为什么消费者重要了。</p>
<h2 id="%E8%BF%AD%E4%BB%A3%E5%99%A8%EF%BC%88iterators%EF%BC%89">迭代器（Iterators）</h2>
<p>正如我们之前说的，迭代器是一个我们可以重复调用它的<code>.next()</code>方法，然后它会给我们一个数据序列的结构。因为你需要调用函数，这意味着迭代器是<em>惰性的</em>（*lazy *）并且不需要预先生成所有的值。例如，下面的代码并没有真正的生成<code>1-99</code>这些数，而是创建了一个值来代表这个序列：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> nums = <span class="hljs-number">1</span>..<span class="hljs-number">100</span>;
</div></code></pre>
<p>因为我们没有用范围做任何事，它并未生成序列。让我们加上消费者：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> nums = (<span class="hljs-number">1</span>..<span class="hljs-number">100</span>).collect::&lt;<span class="hljs-built_in">Vec</span>&lt;<span class="hljs-built_in">i32</span>&gt;&gt;();
</div></code></pre>
<p>现在，<code>collect()</code>会要求范围生成一些值，接着它会开始产生序列。</p>
<p>范围是你会见到的两个基本迭代器之一。另一个是<code>iter()</code>。<code>iter()</code>可以把一个向量转换为一个简单的按顺序给出每个值的迭代器：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> nums = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>];

<span class="hljs-keyword">for</span> num <span class="hljs-keyword">in</span> nums.iter() {
   <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, num);
}
</div></code></pre>
<p>这两个基本迭代器应该能胜任你的工作。还有一些高级迭代器，包括一个是无限的。</p>
<p>关于迭代器的介绍足够了。迭代适配器是关于迭代器最后一个要介绍的内容了。让我们开始吧！</p>
<h2 id="%E8%BF%AD%E4%BB%A3%E9%80%82%E9%85%8D%E5%99%A8%EF%BC%88iterator-adapters%EF%BC%89">迭代适配器（Iterator adapters）</h2>
<p><em>迭代适配器</em>（<em>Iterator adapters</em>）获取一个迭代器然后按某种方法修改它，并产生一个新的迭代器。最简单的是一个是<code>map</code>：</p>
<pre class="hljs"><code><div>(<span class="hljs-number">1</span>..<span class="hljs-number">100</span>).map(|x| x + <span class="hljs-number">1</span>);
</div></code></pre>
<p>在其他迭代器上调用<code>map</code>，然后产生一个新的迭代器，它的每个元素引用被调用了作为参数的闭包。所以它会给我们<code>2-100</code>这些数字。好吧，看起来是这样。如果你编译这个例子，你会得到一个警告：</p>
<pre><code class="language-text">warning: unused result which must be used: iterator adaptors are lazy and
         do nothing unless consumed, #[warn(unused_must_use)] on by default
(1..100).map(|x| x + 1);
 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
</code></pre>
<p>又是惰性！那个闭包永远也不会执行。这个例子也不会打印任何数字：</p>
<pre class="hljs"><code><div>(<span class="hljs-number">1</span>..<span class="hljs-number">100</span>).map(|x| <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x));
</div></code></pre>
<p>如果你尝试在一个迭代器上执行带有副作用的闭包，不如直接使用<code>for</code>。</p>
<p>有大量有趣的迭代适配器。<code>take(n)</code>会返回一个源迭代器下<code>n</code>个元素的新迭代器，注意这对源迭代器没有副作用。让我们试试我们之前的无限迭代器，<code>count()</code>：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">for</span> i <span class="hljs-keyword">in</span> (<span class="hljs-number">1</span>..).take(<span class="hljs-number">5</span>) {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, i);
}
</div></code></pre>
<p>这会打印：</p>
<pre><code class="language-text">1
2
3
4
5
</code></pre>
<p><code>filter()</code>是一个带有一个闭包参数的适配器。这个闭包返回<code>true</code>或<code>false</code>。<code>filter()</code>返回的新迭代器只包含闭包返回<code>true</code>的元素：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">for</span> i <span class="hljs-keyword">in</span> (<span class="hljs-number">1</span>..<span class="hljs-number">100</span>).filter(|&amp;x| x % <span class="hljs-number">2</span> == <span class="hljs-number">0</span>) {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, i);
}
</div></code></pre>
<p>这会打印出1到100之间所有的偶数。（注意因为<code>filter</code>并不消费它迭代的元素，它传递每个元素的引用，所以过滤器使用<code>&amp;x</code>来提取其中的整型数据。）</p>
<p>你可以链式的调用所有三种结构：以一个迭代器开始，适配几次，然后处理结果。看看下面的：</p>
<pre class="hljs"><code><div>(<span class="hljs-number">1</span>..)
    .filter(|&amp;x| x % <span class="hljs-number">2</span> == <span class="hljs-number">0</span>)
    .filter(|&amp;x| x % <span class="hljs-number">3</span> == <span class="hljs-number">0</span>)
    .take(<span class="hljs-number">5</span>)
    .collect::&lt;<span class="hljs-built_in">Vec</span>&lt;<span class="hljs-built_in">i32</span>&gt;&gt;();
</div></code></pre>
<p>这会给你一个包含<code>6</code>，<code>12</code>，<code>18</code>，<code>24</code>和<code>30</code>的向量。</p>
<p>这只是一个迭代器、迭代适配器和消费者如何帮助你的小尝试。有很多非常实用的迭代器，当然你也可以编写你自己的迭代器。迭代器提供了一个安全、高效的处理所有类型列表的方法。最开始它们显得比较不寻常，不过如果你玩转了它们，你就会上瘾的。关于不同迭代器和消费者的列表，查看<a href="http://doc.rust-lang.org/std/iter/">迭代器模块文档</a>。</p>
<h2 id="%E5%85%B6%E4%BB%96">其他</h2>
<p>上文中我们了解了迭代器、适配器、消费者的基本概念。下面将以例子来介绍Rust中的其他的适配器和消费者。</p>
<h3 id="skip%E5%92%8Ctake">skip和take</h3>
<p>take(n)的作用是取前n个元素，而skip(n)正好相反，跳过前n个元素。</p>
<h3 id="zip-%E5%92%8C-enumerate%E7%9A%84%E6%81%A9%E6%80%A8%E6%83%85%E4%BB%87">zip 和 enumerate</h3>
<p>zip是一个适配器，他的作用就是将两个迭代器的内容压缩到一起，形成 <code>Iterator&lt;Item=(ValueFromA, ValueFromB)&gt;</code> 这样的新的迭代器；</p>

<h3>Rust的迭代器有一系列的查找函数</h3>
<ul>
<li>all(): 传入一个函数，对所有元素调用这个函数，一旦有一个返回false,则整个表达式返回false，否则返回true</li>
<li>any(): 类似all()，不过这次是任何一个返回true，则整个表达式返回true，否则false</li>
<li>max()和min(): 查找整个迭代器里所有元素，返回最大或最小值的元素。注意：因为第七章讲过的PartialOrder的原因，浮点数无法参被max正确的理解</li>
</ul>

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
  name: 'express',
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