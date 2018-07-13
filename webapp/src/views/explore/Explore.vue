<template>
  <div  id="explore">
      <main>
        <div id="container">
          <div id="center">
              <div id="show">
                  <div id="billboard">
                    <div id="inner">
                        <h2 id="title">^_^ Ruster社区欢迎你！</h2>
                        <p id="desc">Open Source is Future Should Be Community!</p>
                        <h2>QQ群：570065685</h2>
                    </div>
                  </div>
              </div>
              <div id="actix">
                <h2>公司产品</h2>
                <p><br>
                <strong>微软</strong> : <a href="https://github.com/actix">Actix</a></p>
                <p><strong>Mozilla</strong> : <a href="https://github.com/mozilla-iot/webthing-rust">webthing-rust</a> : Rust implementation of a Web Thing server</p>
                <!-- &emsp;&emsp;&emsp;&emsp;[mozilla-iot/webthing-rust](https://github.com/mozilla-iot/webthing-rust) : Rust implementation of a Web Thing server -->
                <p><strong>Functional Software</strong> : <a href="https://github.com/getsentry/sentry-rust">sentry-rust</a> : Sentry for Rust</p>
                <p><br></p>
                <h2>社区项目</h2>
                <p><br></p>
                <ul>
                <li><a href="https://github.com/OUISRC/muro">muro</a> : The interest and community for internet .(reddit clone)</li>
                <li><a href="https://github.com/swipe-app/swipe-server">swipe-server</a> : Swipe app api with actix-web and graphql</li>
                <li><a href="https://github.com/havarnov/yew-actix-protobuf-sample">yew-actix-protobuf-sample</a> : web app written in yew and actix.</li>
                <li><a href="https://github.com/yinyanlv/partner">partner</a> : 一个私人生活辅助系统，前后端分离。前端：angular6+，material-design；后端：actix-web，diesel，mysql，redis
                <ul>
                <li>  <a href="https://github.com/yinyanlv/partner-client">partner-client</a> : partner项目的前端仓库</li>
                </ul>
                </li>
                <li><a href="https://github.com/davidarmstronglewis/personal-blog">personal-blog</a> : my blog</li>
                </ul>
                <p><br></p>
                <h2>Awesome-Actix</h2>
                <p><br>
                <strong>官方</strong> :</p>
                <ul>
                <li><a href="https://actix.rs/">Actix英文站点</a></li>
                <li><a href="https://actix-cn.github.io/">Actix中文站点</a></li>
                <li><a href="https://github.com/actix/actix">actix</a> : Actor framework for Rust</li>
                <li><a href="https://github.com/actix/actix-web">actix-web</a> : Actix web is a rust web framework.</li>
                <li><a href="https://github.com/actix/actix-redis">actix-redis</a> ： Redis actor and middleware for Actix</li>
                <li><a href="https://github.com/actix/actix-protobuf">actix-protobuf</a> : Protobuf integration for actix web</li>
                <li><a href="https://github.com/actix/actix-remote">actix-remote</a> : Derive macro for actix framework</li>
                <li><a href="https://github.com/actix/examples">examples</a> : Actix web examples</li>
                </ul>
              </div>
          </div>
        </div>
      </main>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
  name: 'explore',
  data: function() {
    return {
      
    }
  },
  mounted: function() {
     
  }
}
</script>

<style scoped>
a {
  color: #0541af;
}
#show {
    background-color: aliceblue;
}
#show #billboard{
    background:url(../../../static/imgs/billboard.png) no-repeat 5px 5px;
}
#center #show #billboard #inner {
    color:fuchsia;
}
#center #actix {
        padding: 1rem;
        background-color: #ffffff;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 0 auto;
        width: 97%;
    }
    #center #show #billboard{
        height: 33rem;
    }
    #center #show #billboard #inner {
        padding: 23rem 1vw 1vh 11vw;
    }
    #center #show #billboard #desc {
        padding: 3vh 0;
    } 
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
    #center #show #billboard{
        height: 23rem;
    }
    #center #show #billboard #inner {
        padding: 11vh 1vw 1vh 51vw;
    }
    #center #show #billboard #desc {
        padding: 3vh 0;
    } 
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 75%;
        padding-top: 77px;
    }
    #center #show #billboard{
        height: 23rem;
    }
    #center #show #billboard #inner {
        padding: 11vh 1vw 1vh 38vw;
    }
    #center #show #billboard #desc {
        padding: 4vh 0;
    } 
}
</style>