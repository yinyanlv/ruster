<template>
  <div  id="wikirust">
        <details >
              <summary id="summary1">Rust</summary>
                <li id="details2">
                    <a  href="">Crates</a>
                </li>
                <li>
                    <details id="details2">
                      <summary>Book</summary>
                          <li>
                              <details id="details2">
                                <summary>Cargo</summary>
                                    <li><a  href="/a/wiki/rust/book/cargo/cargobase">Cargo基础</a></li>
                                    <li><a  href="/a/wiki/rust/book/cargo/cargoset">Cargo配置</a></li>
                                </details>
                          </li>
                          <li>
                              <details id="details2">
                                <summary>Cargo</summary>
                                    <li><a  href="">Cargo基础</a></li>
                                    <li><a  href="">Cargo配置</a></li>
                                </details>
                          </li>
                          <li>
                              <details id="details2">
                                <summary>Cargo</summary>
                                    <li><a  href="">Cargo基础</a></li>
                                    <li><a  href="">Cargo配置</a></li>
                                </details>
                          </li>
                          <li>
                              <details id="details2">
                                <summary>Cargo</summary>
                                    <li><a  href="">Cargo基础</a></li>
                                    <li><a  href="">Cargo配置</a></li>
                                </details>
                          </li>
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
    #wikirust #summary1 {
        font-size: 1.2rem;
    }
    #wikirust #details2 {
        font-size: 1.1rem;
        padding-left: 1vw; 
        margin: 0.2rem;
    }
    #wikirust #details2 li {
        font-size: 0.9rem;
        padding: 0.3vh 1vw; 
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   #wikirust #summary1 {
      font-size: 1.1rem;
   }
   #wikirust #details2 {
      font-size: 1rem;
      padding-left: 1vw; 
      margin: 0.2rem;
   }
   #wikirust #details2 li {
      font-size: 0.9rem;
      padding: 0.3vh 1vw; 
   }
}
@media only screen and (min-width: 850px) {
   #wikirust  #summary1 {
      font-size: 1.2rem;
   }
   #wikirust  #details2 {
      font-size: 1.1rem;
      padding-left: 1vw; 
      margin: 0.2rem;
   }
   #wikirust  #details2 li {
      font-size: 0.9rem;
      padding: 0.3vh 1vw; 
   }
}
</style>