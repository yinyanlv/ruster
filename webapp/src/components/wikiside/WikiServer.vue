<template>
  <div  id="wikiserver">
        <details >
              <summary>Server</summary>
                <li>
                    <details >
                      <summary>Actix</summary>
                          <li>
                              <details >
                                <summary>Actix</summary>
                                    <!-- <li><a  href="/a/wiki/rust/book/cargo/cargoset">Cargo配置</a></li> -->
                                </details>
                          </li>
                          <li>
                              <details >
                                <summary>Actix-web</summary>
                                    <li><a  href="/a/wiki/server/actix/actix-web/whatis">何为Actix</a></li>
                                    <li><a  href="/a/wiki/server/actix/actix-web/installation">安装</a></li>
                                    <li><a  href="/a/wiki/server/actix/actix-web/getting-starte">开始</a></li>
                                    <li><a  href="/a/wiki/server/actix/actix-web/application">应用</a></li>
                                    <li><a  href="/a/wiki/server/actix/actix-web/getting-starte">开始</a></li>
                                    <li><a  href="/a/wiki/server/actix/actix-web/getting-starte">开始</a></li>
                                    <li><a  href="/a/wiki/server/actix/actix-web/getting-starte">开始</a></li>
                                </details>
                          </li>
                    </details>
                </li>
                <li>
                    <details >
                      <summary>Diesel</summary>
                          <!-- <li><a  href="">Type</a></li>
                          <li><a  href="">Std</a></li> -->
                    </details>
                </li>
        </details>
  </div>
</template>
<script>
/* eslint-disable */
export default {
  name: 'wikiserver'
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    #wikiserver details summary {
      font-size: 1.2rem;
      margin: 0.1rem;
   }
   #wikiserver  details li {
      padding: 0.1vh 3vw; 
   }
   #wikiserver  details details summary {
      font-size: 1.1rem;
   }
   #wikiserver  details details details summary {
      font-size: 1.0rem;
   }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   #wikiserver details summary {
      font-size: 1.1rem;
      margin: 0.1rem;
   }
   #wikiserver  details li {
      padding: 0.2vh 2vw; 
   }
   #wikiserver  details details summary {
      font-size: 1rem;
   }
   #wikiserver  details details details summary {
      font-size: 0.9rem;
   }
}
@media only screen and (min-width: 850px) {
   #wikiserver details summary {
      font-size: 1.2rem;
      margin: 0.1rem;
   }
   #wikiserver  details li {
      padding: 0.2vh 1vw; 
   }
   #wikiserver  details details summary {
      font-size: 1.1rem;
   }
   #wikiserver  details details details summary {
      font-size: 1.0rem;
   }
}
</style>