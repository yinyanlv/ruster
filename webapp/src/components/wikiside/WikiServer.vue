<template>
  <div  id="wikiserver">
        <details >
              <summary id="summary1">Server</summary>
                <li>
                    <details id="details2">
                      <summary>Actix</summary>
                          <!-- <li><a  href="">Type</a></li>
                          <li><a  href="">Std</a></li> -->
                    </details>
                </li>
                <li>
                    <details id="details2">
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
@media only screen and (min-width: 850px) {
   #wikiserver  #summary1 {
      font-size: 1.2rem;
   }
   #wikiserver  #details2 {
      font-size: 1.1rem;
      padding-left: 1vw; 
      margin: 0.2rem;
   }
   #wikiserver  #details2 li {
      font-size: 0.9rem;
      padding: 0.3vh 1vw; 
   }
}
</style>