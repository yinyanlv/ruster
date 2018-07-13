<template>
    <div id="yew">
        <main>
            <div id="container">
                <wikiside id="wikiside"></wikiside>
                <div id="content">
                    <p><strong>Yew</strong> This is the yew page.</p> <br>
                </div>
            </div>
      </main>
    </div>
</template>

<script>
import Wikiside from '../../../components/wikiside/Wikiside'
export default {
    name: 'yew',
    components: {
        "wikiside": Wikiside
    },
    mounted: function() {
        var path = window.location.pathname
        if (path == "/a/wiki/yew") {
            var actix = document.getElementById("yewnav")
            actix.style.background = "#18c27b" ;
        }
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
    main {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #content {
        width: 80%;
        margin-left: 1rem;
    }
    #container #wikiside {
        flex: 1;
        background-color: #f1ebeb;
    }
}
</style>