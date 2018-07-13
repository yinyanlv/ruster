<template>
  <div  id="wikiside">
        <div id="container">
            <wikirust></wikirust>
            <wikiserver></wikiserver>
            <wikiwasm></wikiwasm>
            <wikiiot></wikiiot>
        </div>
  </div>
</template>

<script>
/* eslint-disable */
import WikiRust from './WikiRust'
import WikiServer from './WikiServer'
import WikiWasm from './WikiWasm'
import WikiIOT from './WikiIOT'
export default {
  name: 'wikiside',
  components: {
    "wikirust": WikiRust,
    "wikiserver": WikiServer,
    "wikiwasm": WikiWasm,
    "wikiiot": WikiIOT
  }
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    #container  {
      display: block;
      padding: 0.3rem;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   #container  {
     display: block;
     margin: 0.7rem;
   }
}
@media only screen and (min-width: 850px) {
   #container #wikirust, 
   #container #wikiserver,
   #container #wikiwasm,
   #container #wikiiot {
     display: block;
     margin: 1rem;
   }
}
</style>