<template>
    <div id="more">
        <!-- <mnav id="mnav"></mnav> -->
        <div id="content">
        </div>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Mnav from '../../components/nav/Mnav'
export default {
    name: 'more',
    components: {
        "mnav": Mnav
    },
    data() {
        return {
            file: '',
            size: '',
            path: ''
        }
    },
    methods: {
    }
}
</script>

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