import Vue from 'vue'
import Router from 'vue-router'
import App from './App.vue'
import allrouter from './router'
import store from './store'
import './registerServiceWorker'
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'

Vue.directive('highlight', function (el) {
  let blocks = el.querySelectorAll('pre code')
  blocks.forEach((block) => {
    hljs.highlightBlock(block)
  })
})

Vue.use(Router);
Vue.config.productionTip = false
const router = allrouter();

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
