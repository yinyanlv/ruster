<template>
  <div id="app">
    <mnav id="mnav"></mnav>
    <div id="body">
      <router-view></router-view>
    </div>
  </div>
</template>

<script>
import Mnav from './components/nav/Mnav'
export default {
  name: 'app',
  components: {
    "mnav": Mnav
  }
}
</script>

<style>
:root {
    --purple: #99177d;
}
* {
    padding:0px; margin:0px;
    box-sizing: border-box;
}
a {
  text-decoration: none;
  color: inherit;
}
a:hover { color: var(--purple); }

ul, ol, li {
    list-style: none;
}
#body {
  background-color: #faf5f5;
  padding-bottom: 1rem;
  min-height: 100vh;
}
</style>