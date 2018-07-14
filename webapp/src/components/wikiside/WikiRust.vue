<template>
  <div  id="wikirust">
        <details >
              <summary >Rust</summary>
                <!-- <li >
                    <a  href="">Crates</a>
                </li> -->
                <li>
                    <details >
                      <summary>Book</summary>
                          <li>
                              <details >
                                <summary>Cargo</summary>
                                    <li><a  href="/a/wiki/rust/book/cargo/cargobase">Cargo基础</a></li>
                                    <li><a  href="/a/wiki/rust/book/cargo/cargoset">Cargo配置</a></li>
                                    <li><a  href="/a/wiki/rust/book/cargo/rustup">Rustup</a></li>
                                </details>
                          </li>
                          <li>
                              <details >
                                <summary>Express</summary>
                                    <li><a  href="/a/wiki/rust/book/express/pattern">匹配-模式</a></li>
                                    <li><a  href="/a/wiki/rust/book/express/iterator">迭代器</a></li>
                                    <li><a  href="/a/wiki/rust/book/express/express">表达式</a></li>
                                </details>
                          </li>
                          <!-- <li>
                              <details >
                                <summary>Cargo</summary>
                                    <li><a  href="">Cargo基础</a></li>
                                    <li><a  href="">Cargo配置</a></li>
                                </details>
                          </li>
                          <li>
                              <details >
                                <summary>Cargo</summary>
                                    <li><a  href="">Cargo基础</a></li>
                                    <li><a  href="">Cargo配置</a></li>
                                </details>
                          </li> -->
                    </details>
                </li>
        </details>
  </div>
</template>
<script>
/* eslint-disable */
export default {
  name: 'wikirust'
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    #wikirust details summary {
      font-size: 1.2rem;
      margin: 0.1rem;
   }
   #wikirust  details li {
      padding: 0.1vh 3vw; 
   }
   #wikirust  details details summary {
      font-size: 1.1rem;
   }
   #wikirust  details details details summary {
      font-size: 1.0rem;
   }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   #wikirust details summary {
      font-size: 1.1rem;
      margin: 0.1rem;
   }
   #wikirust  details li {
      padding: 0.2vh 2vw; 
   }
   #wikirust  details details summary {
      font-size: 1rem;
   }
   #wikirust  details details details summary {
      font-size: 0.9rem;
   }
}
@media only screen and (min-width: 850px) {
   #wikirust details summary {
      font-size: 1.2rem;
      margin: 0.1rem;
   }
   #wikirust  details li {
      padding: 0.2vh 1vw; 
   }
   #wikirust  details details summary {
      font-size: 1.1rem;
   }
   #wikirust  details details details summary {
      font-size: 1.0rem;
   }
}
</style>