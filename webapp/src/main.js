import Vue from 'vue'
import Router from 'vue-router'
import App from './App.vue'
import allrouter from './router'
import store from './store'
import './registerServiceWorker'

Vue.use(Router);
Vue.config.productionTip = false
const router = allrouter();

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
