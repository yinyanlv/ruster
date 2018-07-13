<template>
    <div id="usercomment">
      <div id="show"><img src="../../../static/imgs/ruster.png" /></div>
      <div id="title">
          <ul>
              <li><a :href="'/a/user/' + $route.params.id" >主题</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/comment'" id="comment-title">评论</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/save'" >收藏</a></li>
              <li v-if="this.$route.params.id == signin_user.id"><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title">消息</a></li>
              <li v-else><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title"></a></li>
          </ul>
      </div>
      <main>
        <div id="container">
            <div id="center">
                <div id="items" v-for="(comment, index) in comment_result" :key="index">
                            <div id="item">
                                <div id="infos">
                                    <span id="info"><a :href="'/a/user/' + hourse_user.id">{{ hourse_user.username }}</a></span>&emsp;
                                    <span id="info"><a :href="'/a/undefined' + '/theme/' + comment.theme_id">主题{{ comment.theme_id }}</a></span>&emsp;
                                    <span id="info"> {{ comment.created_at }} </span>&emsp;
                                </div> 
                                <div id="item-title">
                                  <a :href="'/a/undefined' + '/theme/' + comment.theme_id"> {{ comment.content }} </a>&emsp;
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
  name: 'usercomment',
  components: {
    "gotop": Gotop,
    "mnav": Mnav
  },
  data: function() {
    return {
      signin_user: '',
      hourse_user: '',
      comment_result: ''
    }
  },
  mounted: function() {
      let data = { user_id : Number.parseInt(this.$route.params.id)}
      if (sessionStorage.getItem('signin_user')){
          this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
      }
      
      fetch(URLprefix + 'api/user/id/comments',{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.comment_result = json.comments.reverse()
                  console.log(this.comment_result)
              })
              .catch((e) => {
                console.log(e)
              })
        fetch(URLprefix + 'api/user_id',{
                    body: JSON.stringify(data), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
                }).then(response => response.json())
                .then(json => {
                    json.hourse_user.created_at = json.hourse_user.created_at.slice(0,10)
                    this.hourse_user =  json.hourse_user
                }).catch((e) => {
                    console.log(e)
                })
  },
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
#title #comment-title {
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
#center #items #infos{
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