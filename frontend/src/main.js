import Vue from 'vue'
import VueRouter from 'vue-router'

import App from './App.vue'
import PageIndex from './components/PageIndex.vue'
import PageLogin from './components/PageLogin.vue'
import PageRegister from './components/PageRegister.vue'
import PageChallenge from './components/PageChallenge.vue'
import PageChallenges from './components/PageChallenges.vue'
import PagePlaylists from './components/PagePlaylists.vue'
import PageScoreboard from './components/PageScoreboard.vue'

import 'mini.css'

Vue.use(VueRouter)
Vue.config.productionTip = false

const routes = [
  { path: '/', component: PageIndex },
  { path: '/login', component: PageLogin },
  { path: '/register', component: PageRegister },
  { path: '/challenges/:id', component: PageChallenge },
  { path: '/challenges', component: PageChallenges },
  { path: '/playlists', component: PagePlaylists },
  { path: '/scoreboard', component: PageScoreboard },
]

const router = new VueRouter({
  routes // short for `routes: routes`
})

new Vue({
  router,
  render: h => h(App),
}).$mount('#app')
