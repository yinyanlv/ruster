<template>
  <div  id="wikiside">
        <div id="container">
            <details id="details1">
              <summary id="summary1">Rust</summary>
                <li id="details2">
                    <a  href="">Crates</a>
                </li>
                <li>
                    <details id="details2">
                      <summary>Explore</summary>
                          <li><a  href="">Type</a></li>
                          <li><a  href="">Std</a></li>
                    </details>
                </li>
            </details>

            <details id="details1">
              <summary id="summary1">Server</summary>
                  <li>
                    <details id="details2">
                      <summary>Actix</summary>
                          <li><a  href="">何为Actix</a></li>
                          <li><a  href="">安装</a></li>
                    </details>
                  </li>
                  <li>
                    <details id="details2">
                      <summary>Diesel</summary>
                          <!-- <li><a  href="">何为Actix</a></li>
                          <li><a  href="">安装</a></li> -->
                    </details>
                  </li>
            </details>

            <details id="details1">
              <summary id="summary1">Wasm</summary>
                  <!-- <li><a  href="">何为Actix</a></li>
                  <li><a  href="">安装</a></li> -->
            </details>

            <details id="details1">
              <summary id="summary1">IOT</summary>
                  <!-- <li><a  href="">何为Actix</a></li>
                  <li><a  href="">安装</a></li> -->
            </details>
        </div>
  </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'wikiside'
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    #container #details1 {
      display: block;
      padding: 0.3rem;
    }
    #container #details1 #summary1 {
        font-size: 1.2rem;
    }
    #container #details1 #details2 {
        font-size: 1.1rem;
        padding-left: 1vw; 
        margin: 0.2rem;
    }
    #container #details1 #details2 li {
        font-size: 0.9rem;
        padding: 0.3vh 1vw; 
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   #container #details1 {
     display: block;
     margin: 0.7rem;
   }
   #container #details1 #summary1 {
      font-size: 1.1rem;
   }
   #container #details1 #details2 {
      font-size: 1rem;
      padding-left: 1vw; 
      margin: 0.2rem;
   }
   #container #details1 #details2 li {
      font-size: 0.9rem;
      padding: 0.3vh 1vw; 
   }
}
@media only screen and (min-width: 850px) {
   #container #details1 {
     display: block;
     margin: 1rem;
   }
   #container #details1 #summary1 {
      font-size: 1.2rem;
   }
   #container #details1 #details2 {
      font-size: 1.1rem;
      padding-left: 1vw; 
      margin: 0.2rem;
   }
   #container #details1 #details2 li {
      font-size: 0.9rem;
      padding: 0.3vh 1vw; 
   }
}
</style>