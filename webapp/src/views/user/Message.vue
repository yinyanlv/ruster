<template>
    <div id="usermessage">
      <div id="show"><img src="../../../static/imgs/ruster.png" /></div>
      <div id="title">
          <ul>
              <li><a :href="'/a/user/' + $route.params.id" >主题</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/comment'" >评论</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/save'" >收藏</a></li>
              <li v-if="(messages_count != 0)&&(this.$route.params.id == signin_user.id)"><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title">消息</a>&emsp;<button id="readall" @click="readall">全部阅读</button></li>
              <li v-else-if="(messages_count == 0)&&(this.$route.params.id == signin_user.id)"><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title">消息</a></li>
              <li v-else><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title"></a></li>
          </ul>
      </div>
      <main>
        <div id="container">
            <div id="center">
                <div id="items" v-for="(message, index) in messages" :key="index">
                            <div id="item">
                                <div id="infos">
                                    <span id="info"><a :href="'/a/user/' + message.from_user_id">用户{{ message.from_user_id }}</a></span>&emsp;
                                    <span id="info"><a :href="'/a/undefined' + '/theme/' + message.theme_id">在主题{{ message.theme_id }}回复你</a></span>&emsp;
                                    <span id="info"> {{ message.created_at }} </span>&emsp;
                                    <span v-if="message.message_status == 0" id="messagenew">新</span>
                                </div> 
                                <div id="item-title">
                                  <a :href="'/a/undefined' + '/theme/' + message.theme_id + '#comment'" title="theme.title"> {{ message.content }} </a>
                                </div>
                            </div>
                      </div>
            </div>
            <div id="aside">
            </div>
            <gotop></gotop>
        </div>
      </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import auth from '../../utils/auth'
import Gotop from '../../components/gotop/Gotop'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'usermessage',
  components: {
    "gotop": Gotop,
    "mnav": Mnav
  },
  data: function() {
    return {
      theme_list: '',
      messages: '',
      messages_count: '',
      signin_user: '',
      messages_unread_ids: ''
    }
  },
  mounted: function() {
      if (sessionStorage.getItem('signin_user')){
          this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
      }
      let data = { user_id : Number.parseInt(this.$route.params.id)}
      fetch(URLprefix + 'api/user/id/messages',{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.messages = json.messages.reverse()
                  this.messages_count = json.messages_count
                  this.messages_unread_ids = json.messages_unread_ids
                  console.log(this.messages_unread_ids)
                  console.log(this.messages_count)
              })
              .catch((e) => {
                console.log(e)
              })
  },
  methods: {
    readall() {
              let data = { 
                  user_id : Number.parseInt(this.$route.params.id),
                  messages_unread_ids: this.messages_unread_ids
              }
              fetch(URLprefix + 'api/user/id/messages/readall',{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                 window.location.reload ( true );
              })
              .catch((e) => {
                console.log(e)
              })
    }
  }
}
</script>

<style scoped>
#show {
    background-color: #f1a3d6;
}
#title {
    line-height: 3.3rem;
    background-color: #faeaf5;
}
#title #message-title {
    padding-bottom: 0.2rem;
    border-bottom: 3px solid #a506a5;
}
#container a{
    color: #0541af;
}
#center {
    background-color: #ffffff;
}
#center #items #item {
    padding: 1.2vh 0.5vw;
    border-bottom: 1px solid #f3e1f8;
}
#center #items #item-title {
    margin-top: 1vh;
}
#title li #readall {
        border-radius: 50%;
        padding: 0.1vh 0.2vw;
        border: 1px solid #aac;
        background-color: #f0c7fc;
}
#center #items #infos #messagenew {
    padding: 0.2vh 0.2vw;
    font-size: 0.8rem;
    border-radius: 50%;
    color: yellow;
    background-color: fuchsia;
}
#center #items #infos {
    font-size: 0.85rem;
}
@media only screen and (max-width: 600px) {
  img {
      margin: 1vh 2vw;
      width: 5rem;
      height: 5rem;
      border-radius: 50%;
  }
  #title ul li {
      display: inline-block;
      padding-left: 3vw;
      font-weight: bold;
  }
  main{
      margin: 0 auto;
      width: 97%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    #show {
        padding-top: 5rem;
    }
    img {
        margin-left: 8vw;
        width: 7rem;
        height: 7rem;
        border-radius: 50%;
    }
    #title ul {
        margin-left: 6vw;
    }
    #title ul li {
        display: inline-block;
        font-weight: bold;
        padding-left: 2rem;
    }
    #title ul #item {
        padding-left: 2rem;
    }
    main {
        margin: 1rem auto;
        width: 80%;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 90%;
        margin-right: 1vw;
    }
    #container #aside {
        flex: 1;
    }
}
@media only screen and (min-width: 850px) {
    #show {
        padding-top: 6rem;
    }
    img {
        margin-left: 11vw;
        width: 8rem;
        height: 8rem;
        border-radius: 50%;
    }
    #title ul {
        margin-left: 10vw;
    }
    #title ul li{
        display: inline-block;
        font-weight: bold;
        font-size: 1.1rem;
        padding-left: 2rem;
    }
    main {
        margin: 1rem auto;
        width: 75%;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1vw;
    }
    #container #aside {
        flex: 1;
    }
}
</style>