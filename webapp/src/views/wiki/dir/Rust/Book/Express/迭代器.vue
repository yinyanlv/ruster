<template>
  <div  id="iterator">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
<h1 id="%E8%BF%AD%E4%BB%A3%E5%99%A8">迭代器</h1>
<h2 id="%E4%BB%8Efor%E5%BE%AA%E7%8E%AF%E8%AE%B2%E8%B5%B7">从for循环讲起</h2>
<p>我们在控制语句里学习了Rust的<code>for</code>循环表达式，我们知道，Rust的for循环实际上和C语言的循环语句是不同的。这是为什么呢？因为，<code>for</code>循环不过是Rust编译器提供的语法糖！</p>
<p>首先，我们知道Rust有一个<code>for</code>循环能够依次对迭代器的任意元素进行访问，即：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">for</span> i <span class="hljs-keyword">in</span> <span class="hljs-number">1</span>..<span class="hljs-number">10</span> {
    <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, i);
}
</div></code></pre>
<p>这里我们知道， (1..10) 其本身是一个迭代器，我们能对这个迭代器调用 <code>.next()</code> 方法，因此，<code>for</code>循环就能完整的遍历一个循环。
而对于<code>Vec</code>来说：</p>
<pre><code>let values = vec![1,2,3];
for x in values {
    println!("{}", x);
}
</code></pre>
<p>在上面的代码中，我们并没有显式地将一个<code>Vec</code>转换成一个迭代器，那么它是如何工作的呢？现在就打开标准库翻api的同学可能发现了,<code>Vec</code>本身并没有实现 <code>Iterator</code> ，也就是说，你无法对<code>Vec</code>本身调用 <code>.next()</code> 方法。但是，我们在搜索的时候，发现了<code>Vec</code>实现了 <code>IntoIterator</code> 的 trait。</p>
<p>其实，<code>for</code>循环真正循环的，并不是一个迭代器(Iterator)，真正在这个语法糖里起作用的，是 <code>IntoIterator</code> 这个 trait。</p>
<p>因此，上面的代码可以被展开成如下的等效代码(只是示意，不保证编译成功):</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> values = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>];

{
    <span class="hljs-keyword">let</span> result = <span class="hljs-keyword">match</span> <span class="hljs-built_in">IntoIterator</span>::into_iter(values) {
        <span class="hljs-keyword">mut</span> iter =&gt; <span class="hljs-keyword">loop</span> {
            <span class="hljs-keyword">match</span> iter.next() {
                <span class="hljs-literal">Some</span>(x) =&gt; { <span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, x); },
                <span class="hljs-literal">None</span> =&gt; <span class="hljs-keyword">break</span>,
            }
        },
    };
    result
}
</div></code></pre>
<p>在这个代码里，我们首先对<code>Vec</code>调用 <code>into_iter</code> 来判断其是否能被转换成一个迭代器，如果能，则进行迭代。</p>
<p>那么，迭代器自己怎么办？</p>
<p>为此，Rust在标准库里提供了一个实现：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">impl</span>&lt;I: <span class="hljs-built_in">Iterator</span>&gt; <span class="hljs-built_in">IntoIterator</span> <span class="hljs-keyword">for</span> I {
    <span class="hljs-comment">// ...</span>
}
</div></code></pre>
<p>也就是说，Rust为所有的迭代器默认的实现了 <code>IntoIterator</code>，这个实现很简单，就是每次返回自己就好了。</p>
<p>也就是说：</p>
<p>任意一个 <code>Iterator</code> 都可以被用在 <code>for</code> 循环上！</p>
<h3 id="%E6%97%A0%E9%99%90%E8%BF%AD%E4%BB%A3%E5%99%A8">无限迭代器</h3>
<p>Rust支持通过省略高位的形式生成一个无限长度的自增序列，即：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> inf_seq = (<span class="hljs-number">1</span>..).into_iter();
</div></code></pre>
<p>不过不用担心这个无限增长的序列撑爆你的内存，占用你的CPU，因为适配器的惰性的特性，它本身是安全的，除非你对这个序列进行<code>collect</code>或者<code>fold</code>！
不过，我想聪明如你，不会犯这种错误吧！
因此，想要应用这个，你需要用<code>take</code>或者<code>take_while</code>来截断他，必须？ 除非你将它当作一个生成器。当然了，那就是另外一个故事了。</p>
<h2 id="%E6%B6%88%E8%B4%B9%E8%80%85%E4%B8%8E%E9%80%82%E9%85%8D%E5%99%A8">消费者与适配器</h2>
<p>说完了<code>for</code>循环，我们大致弄清楚了 <code>Iterator</code> 和 <code>IntoIterator</code> 之间的关系。下面我们来说一说消费者和适配器。</p>
<p>消费者是迭代器上一种特殊的操作，其主要作用就是将迭代器转换成其他类型的值，而非另一个迭代器。</p>
<p>而适配器，则是对迭代器进行遍历，并且其生成的结果是另一个迭代器，可以被链式调用直接调用下去。</p>
<p>由上面的推论我们可以得出: <em>迭代器其实也是一种适配器！</em></p>
<h3 id="%E6%B6%88%E8%B4%B9%E8%80%85">消费者</h3>
<p>就像所有人都熟知的生产者消费者模型，迭代器负责生产，而消费者则负责将生产出来的东西最终做一个转化。一个典型的消费者就是<code>collect</code>。前面我们写过<code>collect</code>的相关操作，它负责将迭代器里面的所有数据取出，例如下面的操作：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> v = (<span class="hljs-number">1</span>..<span class="hljs-number">20</span>).collect(); <span class="hljs-comment">//编译通不过的！</span>
</div></code></pre>
<p>尝试运行上面的代码，却发现编译器并不让你通过。因为你没指定类型！指定什么类型呢？原来collect只知道将迭代器收集到一个实现了 <code>FromIterator</code> 的类型中去，但是，事实上实现这个 trait 的类型有很多（Vec, HashMap等），因此，collect没有一个上下文来判断应该将v按照什么样的方式收集！！</p>
<p>要解决这个问题，我们有两种解决办法：</p>
<ol>
<li>
<p>显式地标明<code>v</code>的类型:</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> v: <span class="hljs-built_in">Vec</span>&lt;_&gt; = (<span class="hljs-number">1</span>..<span class="hljs-number">20</span>).collect();
</div></code></pre>
</li>
<li>
<p>显式地指定<code>collect</code>调用时的类型：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> v = (<span class="hljs-number">1</span>..<span class="hljs-number">20</span>).collect::&lt;<span class="hljs-built_in">Vec</span>&lt;_&gt;&gt;();
</div></code></pre>
</li>
</ol>
<p>当然，一个迭代器中还存在其他的消费者，比如取第几个值所用的 <code>.nth()</code>函数，还有用来查找值的 <code>.find()</code> 函数，调用下一个值的<code>next()</code>函数等等，这里限于篇幅我们不能一一介绍。所以，下面我们只介绍另一个比较常用的消费者—— <code>fold</code> 。</p>
<p>当然了，提起Rust里的名字你可能没啥感觉，其实，<code>fold</code>函数，正是大名鼎鼎的 MapReduce 中的 Reduce 函数(稍微有点区别就是这个Reduce是带初始值的)。</p>
<p><code>fold</code>函数的形式如下：</p>
<pre class="hljs"><code><div>fold(base, |accumulator, element| .. )
</div></code></pre>
<p>我们可以写成如下例子：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> m = (<span class="hljs-number">1</span>..<span class="hljs-number">20</span>).fold(<span class="hljs-number">1u64</span>, |mul, x| mul*x);
</div></code></pre>
<p>需要注意的是，<code>fold</code>的输出结果的类型，最终是和<code>base</code>的类型是一致的（如果<code>base</code>的类型没指定，那么可以根据前面<code>m</code>的类型进行反推，除非<code>m</code>的类型也未指定），也就是说，一旦我们将上面代码中的<code>base</code>从 <code>1u64</code> 改成 <code>1</code>，那么这行代码最终将会因为数据溢出而崩溃！</p>
<h3 id="%E9%80%82%E9%85%8D%E5%99%A8">适配器</h3>
<p>我们所熟知的生产消费的模型里，生产者所生产的东西不一定都会被消费者买账，因此，需要对原有的产品进行再组装。这个再组装的过程，就是适配器。因为适配器返回的是一个新的迭代器，所以可以直接用链式请求一直写下去。</p>
<p>前面提到了 Reduce 函数，那么自然不得不提一下另一个配套函数 —— <code>map</code> :</p>
<p>熟悉Python语言的同学肯定知道，Python里内置了一个<code>map</code>函数，可以将一个迭代器的值进行变换，成为另一种。Rust中的<code>map</code>函数实际上也是起的同样的作用，甚至连调用方法也惊人的相似！</p>
<pre class="hljs"><code><div>(<span class="hljs-number">1</span>..<span class="hljs-number">20</span>).map(|x| x+<span class="hljs-number">1</span>);
</div></code></pre>
<p>上面的代码展示了一个“迭代器所有元素的自加一”操作，但是，如果你尝试编译这段代码，编译器会给你提示：</p>
<pre><code>warning: unused result which must be used: iterator adaptors are lazy and
         do nothing unless consumed, #[warn(unused_must_use)] on by default
(1..20).map(|x| x + 1);
 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
</code></pre>
<p>呀，这是啥？</p>
<p>因为，所有的适配器，都是惰性求值的！</p>
<p><strong>也就是说，除非你调用一个消费者，不然，你的操作，永远也不会被调用到！</strong></p>
<p>现在，我们知道了<code>map</code>，那么熟悉Python的人又说了，是不是还有<code>filter</code>！？答，有……用法类似，<code>filter</code>接受一个闭包函数，返回一个布尔值，返回<code>true</code>的时候表示保留，<code>false</code>丢弃。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> v: <span class="hljs-built_in">Vec</span>&lt;_&gt; = (<span class="hljs-number">1</span>..<span class="hljs-number">20</span>).filter(|x| x%<span class="hljs-number">2</span> == <span class="hljs-number">0</span>).collect();
</div></code></pre>
<p>以上代码表示筛选出所有的偶数。</p>
<h2 id="%E5%85%B6%E4%BB%96">其他</h2>
<p>上文中我们了解了迭代器、适配器、消费者的基本概念。下面将以例子来介绍Rust中的其他的适配器和消费者。</p>
<h3 id="skip%E5%92%8Ctake">skip和take</h3>
<p><code>take(n)</code>的作用是取前<code>n</code>个元素，而<code>skip(n)</code>正好相反，跳过前<code>n</code>个元素。</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> v = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>, <span class="hljs-number">4</span>, <span class="hljs-number">5</span>, <span class="hljs-number">6</span>];
<span class="hljs-keyword">let</span> v_take = v.iter()
    .cloned()
    .take(<span class="hljs-number">2</span>)
    .collect::&lt;<span class="hljs-built_in">Vec</span>&lt;_&gt;&gt;();
<span class="hljs-built_in">assert_eq!</span>(v_take, <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1</span>, <span class="hljs-number">2</span>]);

<span class="hljs-keyword">let</span> v_skip: <span class="hljs-built_in">Vec</span>&lt;_&gt; = v.iter()
    .cloned()
    .skip(<span class="hljs-number">2</span>)
    .collect();
<span class="hljs-built_in">assert_eq!</span>(v_skip, <span class="hljs-built_in">vec!</span>[<span class="hljs-number">3</span>, <span class="hljs-number">4</span>, <span class="hljs-number">5</span>, <span class="hljs-number">6</span>]);
</div></code></pre>
<h3 id="zip-%E5%92%8C-enumerate%E7%9A%84%E6%81%A9%E6%80%A8%E6%83%85%E4%BB%87">zip 和 enumerate的恩怨情仇</h3>
<p><code>zip</code>是一个适配器，他的作用就是将两个迭代器的内容压缩到一起，形成 <code>Iterator&lt;Item=(ValueFromA, ValueFromB)&gt;</code> 这样的新的迭代器；</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> names = <span class="hljs-built_in">vec!</span>[<span class="hljs-string">"WaySLOG"</span>, <span class="hljs-string">"Mike"</span>, <span class="hljs-string">"Elton"</span>];
<span class="hljs-keyword">let</span> scores = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">60</span>, <span class="hljs-number">80</span>, <span class="hljs-number">100</span>];
<span class="hljs-keyword">let</span> score_map: HashMap&lt;_, _&gt; = names.iter()
    .zip(scores.iter())
    .collect();
<span class="hljs-built_in">println!</span>(<span class="hljs-string">"{:?}"</span>, score_map);
</div></code></pre>
<p>而<code>enumerate</code>, 熟悉的Python的同学又叫了：Python里也有！对的，作用也是一样的，就是把迭代器的下标显示出来，即：</p>
<pre class="hljs"><code><div><span class="hljs-keyword">let</span> v = <span class="hljs-built_in">vec!</span>[<span class="hljs-number">1u64</span>, <span class="hljs-number">2</span>, <span class="hljs-number">3</span>, <span class="hljs-number">4</span>, <span class="hljs-number">5</span>, <span class="hljs-number">6</span>];
<span class="hljs-keyword">let</span> val = v.iter()
    .enumerate()
    <span class="hljs-comment">// 迭代生成标，并且每两个元素剔除一个</span>
    .filter(|&amp;(idx, _)| idx % <span class="hljs-number">2</span> == <span class="hljs-number">0</span>)
    <span class="hljs-comment">// 将下标去除,如果调用unzip获得最后结果的话，可以调用下面这句，终止链式调用</span>
    <span class="hljs-comment">// .unzip::&lt;_,_, vec&lt;_&gt;, vec&lt;_&gt;&gt;().1</span>
    .map(|(idx, val)| val)
    <span class="hljs-comment">// 累加 1+3+5 = 9</span>
    .fold(<span class="hljs-number">0u64</span>, |sum, acm| sum + acm);

<span class="hljs-built_in">println!</span>(<span class="hljs-string">"{}"</span>, val);
</div></code></pre>
<h3 id="%E4%B8%80%E7%B3%BB%E5%88%97%E6%9F%A5%E6%89%BE%E5%87%BD%E6%95%B0">一系列查找函数</h3>
<p>Rust的迭代器有一系列的查找函数，比如：</p>
<ul>
<li><code>find()</code>: 传入一个闭包函数，从开头到结尾依次查找能令这个闭包返回<code>true</code>的第一个元素，返回<code>Option&lt;Item&gt;</code></li>
<li><code>position()</code>: 类似<code>find</code>函数，不过这次输出的是<code>Option&lt;usize&gt;</code>，第几个元素。</li>
<li><code>all()</code>: 传入一个函数，如果对于任意一个元素，调用这个函数返回<code>false</code>,则整个表达式返回<code>false</code>，否则返回<code>true</code></li>
<li><code>any()</code>: 类似<code>all()</code>，不过这次是任何一个返回<code>true</code>，则整个表达式返回<code>true</code>，否则<code>false</code></li>
<li><code>max()</code>和<code>min()</code>: 查找整个迭代器里所有元素，返回最大或最小值的元素。注意：因为第七章讲过的<code>PartialOrder</code>的原因，<code>max</code>和<code>min</code>作用在浮点数上会有不符合预期的结果。</li>
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
  name: 'iterator',
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