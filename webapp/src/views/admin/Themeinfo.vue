<template>
    <div id="themeinfo">
      <div id="show"><img src="../../../static/imgs/ruster.png" /></div>
      <div id="title">
          <ul>
              <li><a :href="'/a/user/' + $route.params.id" id="theme-title">主题</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/comment'" >评论</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/save'" >收藏</a></li>
              <li v-if="this.$route.params.id == signin_user.id"><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title">消息</a></li>
              <li v-else><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title"></a></li>
          </ul>
      </div>
      <main>
        <div id="container">
            <div id="center">
                <div id="items" v-for="(theme, index) in theme_list" :key="index">
                            <div id="item">
                                <div id="infos">
                                    <!-- <span id="info"><a :href="'/a/home/' + theme.category_name">{{ theme.category_name_cn }}</a></span>&emsp; -->
                                    <span id="info"><a :href="'/a/user/' + hourse_user.id">{{ hourse_user.username }}</a></span>&emsp;
                                    <span id="info"><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id">{{ theme.comment_count }}</a></span>&emsp;
                                    <span id="info">{{ theme.view_count }}</span>&emsp;
                                    <span id="info"> {{ theme.created_at }} </span>
                                </div> 
                                <div id="item-title">
                                  <a :href="'/a/'+ theme.category_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a>
                                </div>
                            </div>
                      </div>
            </div>
            <div id="aside">
                <div id="right">
                    <p><strong>用户名:{{ hourse_user.username }}</strong></p>
                    <p v-if="current_user != ''"><strong>邮箱:{{ current_user.email }}</strong></p>
                    <p>注册时间:{{ hourse_user.created_at }}</p>
                    <p>注册排名:第{{ hourse_user.id }}位</p>
                    
                    <div v-if="current_user != ''" id="userself">
                        <button id="submit"  @click="update">账号更新</button><br/>
                        <button id="submit"  @click="deleteme">账号删除</button><br/>

                        <div id="update" v-if="userupdate == true">
                            <p>账号更新</p> 
                                <input type="email" name="newmail" placeholder="新邮箱" v-model="Newmail"  required /><br/>
                                <input type="password" name="newpassword" placeholder="新密码" v-model="Newpassword"  required/><br/>
                                <input type="password" name="confirm_newpassword" placeholder="确认新密码" v-model="ConfirmNewpassword"  required/><br/>
                                <button id="submit" @click="submitnow">更新</button>
                        </div>
                    </div>
                </div>
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
  name: 'themeinfo',
  components: {
    "gotop": Gotop,
    "mnav": Mnav
  },
  data: function() {
    return {
      hourse_user: '',
      current_user: '',
      Newname: '',
      Newmail: '',
      Newpassword: '',
      ConfirmNewpassword: '',
      signin_user: '',
      userupdate: false,
      theme_list: ''
    }
  },
  mounted: function() {
      let data = { user_id : Number.parseInt(this.$route.params.id)}
      fetch(URLprefix + 'api/user/id/themes',{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.theme_list = json.themes.reverse()
              })
              .catch((e) => {
                console.log(e)
              })

      if (sessionStorage.getItem('token')){
            let signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
            this.signin_user = signin_user
            if (signin_user.id == this.$route.params.id) {
                fetch(URLprefix + 'api/user_info',{
                headers: {
                    'Authorization': 'Bearer ' + sessionStorage.getItem('token')
                },
                method: 'GET',
                }).then(response => response.json())
                .then(json => {
                    json.current_user.created_at = json.current_user.created_at.slice(0,10)
                    this.current_user =  json.current_user
                }).catch((e) => {
                    console.log(e)
                }) 
            }
      }

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
  methods: {
    update() {
        this.userupdate = true
    },
    submitnow() {
        var newname = current_user.username
        var newmail = this.Newmail
        var newpassword = this.Newpassword
        var confirm_newpassword = this.ConfirmNewpassword
        let data = { 
            user_id: current_user.id,
            newname: newname,
            newmail: newmail,
            newpassword: newpassword,
            confirm_newpassword: confirm_newpassword
        }
              fetch(URLprefix + 'api/user_update', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                    this.userupdate = false
                    window.location.reload ( true )
              })
              .catch((e) => {
                console.log(e)
              })
    },
    deleteme() {
        fetch(URLprefix + 'api/user_delete',{
            headers: {
                'Authorization': 'Bearer ' + sessionStorage.getItem('token')
            },
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            sessionStorage.removeItem('token')
            sessionStorage.removeItem('signin_user')
            window.location.reload ( true ); 
            this.$router.push('/')
        }).catch((e) => {
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
#title #theme-title {
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
#center #items #infos {
    font-size: 0.85rem;
}
button, #aside #update input {
    width: 7rem; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
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
  #container #aside {
      margin: 1vh auto;
      padding: 0.6rem;
      border: 1px solid rgb(212, 212, 212);
      background-color: #ffffff;
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
    #container #aside #right{
        padding: 1rem;
        border: 1px solid rgb(212, 212, 212);
        background-color: #ffffff;
    }
    #container #aside #right #update {
        margin: 1rem auto;
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

    #container #aside #right{
        padding: 1rem;
        border: 1px solid rgb(212, 212, 212);
        background-color: #ffffff;
    }
    #container #aside #right #update {
        margin: 1rem auto;
    }
    #center #items #item-title {
        font-size: 1.1rem;
    }
}
</style>