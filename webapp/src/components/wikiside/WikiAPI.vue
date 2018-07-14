<template>
  <div  id="wikiapi">
        <details >
              <summary >API</summary>
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
  name: 'wikiapi'
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    #wikiapi details summary {
      font-size: 1.2rem;
      margin: 0.1rem;
   }
   #wikiapi  details li {
      padding: 0.1vh 3vw; 
   }
   #wikiapi  details details summary {
      font-size: 1.1rem;
   }
   #wikiapi  details details details summary {
      font-size: 1.0rem;
   }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   #wikiapi details summary {
      font-size: 1.1rem;
      margin: 0.1rem;
   }
   #wikiapi  details li {
      padding: 0.2vh 2vw; 
   }
   #wikiapi  details details summary {
      font-size: 1rem;
   }
   #wikiapi  details details details summary {
      font-size: 0.9rem;
   }
}
@media only screen and (min-width: 850px) {
   #wikiapi details summary {
      font-size: 1.2rem;
      margin: 0.1rem;
   }
   #wikiapi  details li {
      padding: 0.2vh 1vw; 
   }
   #wikiapi  details details summary {
      font-size: 1.1rem;
   }
   #wikiapi  details details details summary {
      font-size: 1.0rem;
   }
}
</style>