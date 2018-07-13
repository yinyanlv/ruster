<template>
  <div  id="wikiiot">
        <details >
              <summary id="summary1">IOT</summary>
                <!-- <li id="details2">
                    <a  href="">Crates</a>
                </li>
                <li>
                    <details id="details2">
                      <summary>Book</summary>
                          <li><a  href="">Type</a></li>
                          <li><a  href="">Std</a></li>
                    </details>
                </li> -->
        </details>
  </div>
</template>
<script>
/* eslint-disable */
export default {
  name: 'wikiiot'
}
</script>

<style scoped>
@media only screen and (min-width: 850px) {
   #wikiiot  #summary1 {
      font-size: 1.2rem;
   }
   #wikiiot  #details2 {
      font-size: 1.1rem;
      padding-left: 1vw; 
      margin: 0.2rem;
   }
   #wikiiot  #details2 li {
      font-size: 0.9rem;
      padding: 0.3vh 1vw; 
   }
}
</style>