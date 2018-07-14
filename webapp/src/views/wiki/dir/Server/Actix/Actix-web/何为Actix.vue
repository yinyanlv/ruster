<template>
  <div  id="whatis">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="center">
              <div class="content">
                  <h1>Actix 指多种事情</h1>
                    <p>Actix是一个强大的Rust的actor系统, 在它之上是actix-web框架。这是你在工作中大多使用的东西。Actix-web给你提供了一个有趣且快速的Web开发框架。</p>
                    <p>我们称actix-web为小而务实的框架。对于所有的意图和目的来说，这是一个有少许曲折的微框架。如果你已经是一个Rust程序员，你可能会很快熟悉它，但即使你是来自另一种编程语言，你应该会发现actix-web很容易上手。</p>
                    <p>使用actix-web开发的应用程序将在本机可执行文件中包含HTTP服务器。你可以把它放在另一个像nginx这样的HTTP服务器上。即使完全不存在另一个HTTP服务器的情况下，actix-web也足以提供HTTP 1和HTTP 2支持以及SSL/TLS。这对于构建微服务分发非常有用。</p>
                    <p>最重要的是：actix-web可以稳定发布。</p>
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
  name: 'whatis',
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