<template>
  <div  id="wikiside">
        <div id="container">
            <wikirust></wikirust>
            <wikiserver></wikiserver>
            <wikiwasm></wikiwasm>
            <wikiiot></wikiiot>
            <wikiapi></wikiapi>
        </div>
  </div>
</template>

<script>
/* eslint-disable */
import WikiRust from './WikiRust'
import WikiServer from './WikiServer'
import WikiWasm from './WikiWasm'
import WikiIOT from './WikiIOT'
import WikiAPI from './WikiAPI'
export default {
  name: 'wikiside',
  components: {
    "wikirust": WikiRust,
    "wikiserver": WikiServer,
    "wikiwasm": WikiWasm,
    "wikiiot": WikiIOT,
    "wikiapi": WikiAPI
  }
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    #container #wikirust, 
    #container #wikiserver,
    #container #wikiwasm,
    #container #wikiiot,
    #container #wikiapi {
      display: block;
      margin: 0.5rem;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   #container #wikirust, 
   #container #wikiserver,
   #container #wikiwasm,
   #container #wikiiot,
   #container #wikiapi {
     display: block;
     margin: 0.8rem;
   }
}
@media only screen and (min-width: 850px) {
   #container #wikirust, 
   #container #wikiserver,
   #container #wikiwasm,
   #container #wikiiot,
   #container #wikiapi {
     display: block;
     margin: 1rem;
   }
}
</style>