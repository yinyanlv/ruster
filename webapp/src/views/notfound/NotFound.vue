<template>
  <div >
    <!-- <mnav id="mnav"></mnav> -->
      <h1 id="content">Not Found.</h1>
  </div>
</template>

<script>
/* eslint-disable */
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'notfound',
  components: {
    "mnav": Mnav
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    #content  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px;
  }
}
@media only screen and (min-width: 850px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
}
</style>