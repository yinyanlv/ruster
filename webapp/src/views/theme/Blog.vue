<template>
    <div id="theblog">
        <main >
            <div id="bcontainer">
                <div id="mei">
                    <div id="title">
                            <h1> {{ theme.title }} </h1> 
                            <span v-if="saveorno == false" id="save" @click="save">收藏</span>
                            <span v-else id="saved">已收藏</span>
                            <span id="like">喜欢 <span id="likeid">{{like}} </span> </span>
                            <span id="right"> 
                            <span id="info" class="first"><a :href="'/a/home/' + theme_category_name">博客</a></span> • 
                            <span id="info"><a :href="'/a/user/' + theme_user.id">{{ theme_user.username }}</a></span> •   
                            <span id="info">{{ theme_rtime }}</span>  
                            </span>
                    </div>
                </div>
                <div id="center">
                    <div id="theme">
                        <div id="content" v-html="theme.content" ></div>
                    </div>
                    <hr>
                    <div id="comment">
                        <div id="count">评论 &nbsp; {{ theme.comment_count }} </div>
                        <div v-for="(comment, index) in theme_comments" :key="index">
                            <div id="detail">
                                <div id="infos">
                                    <span id="info" >{{ index + 1 }}&nbsp;</span>
                                    <span id="info"><a :href="'/a/user/' + comment.user_id">{{ comment.username }}</a></span> • <span id="info">{{ comment.rtime }}</span>
                                </div>
                                <div id="content" v-html="comment.content" > </div>
                            </div>
                        </div>
                    </div>
                    <hr>
                    <div id="reply" v-if="signin_user">
                        <div id="messagenote">
                            <p><strong>注意:</strong>发消息格式:[<strong>@人名 内容</strong>]中间有一空格,不然发不出去或收不到</p>
                        </div>
                        <div id="editor">
                            <mavon-editor name="content" v-model="Content" :ishljs = "true" style="height: 100%;" :toolbars="set"></mavon-editor>
                        </div>
                        <button style="margin-top: 1vh;
                                        width:66px; 
                                        line-height:25px;
                                        background-color:#ffffff;
                                        border :1px solid #a39c9c;" type="submit" id="submit" @click="comment">评论
                        </button>
                    </div>  
                    <div v-else style="margin: 10px;">请先登录再发表评论.
                        <a href="/a/signin" style="background-color:aqua;">登陆</a>
                    </div>    
                </div>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import { mavonEditor } from 'mavon-editor'
import 'mavon-editor/dist/css/index.css'
import URLprefix from '../../config'
export default {
    name: 'theblog',
    components: {
        mavonEditor
    },
    data: function() {
        return {
            Content: '',
            theme: '',
            theme_user: '',
            theme_category_name: '',
            theme_rtime: '',
            theme_comments: '',
            signin_user: '',
            like: '',
            saveorno: '',
            set:{
                bold: true, // 粗体
                italic: true, // 斜体
                header: true, // 标题
                underline: true, // 下划线
                strikethrough: true, // 中划线
                mark: true, // 标记
                quote: true, // 引用
                ol: true, // 有序列表
                ul: true, // 无序列表
                link: true, // 链接
                code: true, // code
                trash: true, // 清空
                table: true, // 表格
                fullscreen: true, // 全屏编辑
                alignleft: true, // 左对齐
                aligncenter: true, // 居中
                alignright: true, // 右对齐
                preview: true, // 预览
                help: true, // 帮助

                superscript: false, // 上角标
                subscript: false, // 下角标
                undo: false, // 上一步
                redo: false, // 下一步
                imagelink: false, // 图片链接
                readmodel: false, // 沉浸式阅读
                htmlcode: false, // 展示html源码
                save: false, // 保存（触发events中的save事件）
                navigation: false, // 导航目录
                subfield: false, // 单双栏模式
            }
        }
    },
    mounted: function() {
        let md = mavonEditor.getMarkdownIt() 
        this.saveorno = false
        let theme_id = this.$route.params.id
        if (sessionStorage.getItem('signin_user')){
            this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
            let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            let data = { 
                theme_id: Number.parseInt(theme_id),
                user_id: user_id
            }
            fetch(URLprefix + 'api/blog/like', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.like = json.number
                  this.saveorno = json.saveorno
              })
              .catch((e) => {
                console.log(e)
              })
        }else{
            let user_id = 0
            let data = { 
                theme_id: Number.parseInt(theme_id),
                user_id: user_id
            }
            fetch(URLprefix + 'api/blog/like', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.like = json.number
                  this.saveorno = json.saveorno
              })
              .catch((e) => {
                console.log(e)
              })
        }
        
        fetch(URLprefix + 'api/theme/'+ this.$route.params.id,{
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            this.theme = json.theme
            this.theme_user = json.theme_user
            this.theme_rtime = json.theme_rtime
            this.theme_category_name = json.theme_category_name
            this.theme_comments = json.theme_comments
        }).catch((e) => {
            console.log(e)
        })
  },
  methods: {
    comment () {
        let comment = this.Content
        let theme_id = this.$route.params.id
        let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        let theme_user_id = this.theme_user.id
        let data = {
            theme_id: Number.parseInt(theme_id),
            theme_user_id: Number.parseInt(theme_user_id),
            user_id: Number.parseInt(user_id),
            comment: comment
        }
        if(comment == ''){
                alert("评论内容不能为空")
                return
        }else{
            fetch(URLprefix + 'api/theme/' + this.$route.params.id, {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  window.location.reload ( true ) 
                  this.$router.push('/')
              })
              .catch((e) => {
                console.log(e)
              })  
        }
    },
    save(){
        if (sessionStorage.getItem('signin_user')){
            let save = document.getElementById("save")
            let likeid = document.getElementById("likeid")
            save.style.color = "green"
            save.innerHTML = "已收藏"
            likeid.innerHTML = Number.parseInt(likeid.innerHTML) + 1
            let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            let theme_id = this.$route.params.id
            let data = { 
                user_id: Number.parseInt(user_id),
                theme_id: Number.parseInt(theme_id)
            }
            fetch(URLprefix + 'api/blog/save', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  console.log(json)
                  window.location.reload ( true )
              })
              .catch((e) => {
                console.log(e)
              })
        }
    }
  }
}
</script>

<style>
#theblog #center {
    background-color: #ffffff;
}
#theblog a {
    color: #0541af;
}
#theblog #center #comment, #theblog #center #reply {
    border-top: 1px solid fuchsia;
}
#theblog #center #theme #content {
    margin: 10px;
}
#theblog hr {
    height: 11px;
    background-color: #faf5f5;
    border: 0;
}
#theblog #center #comment #count {
    font-weight: bold;
    color: fuchsia;
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#theblog #center #comment #detail {
    border-bottom: 1px solid rgb(223, 223, 223);
}
#theblog #center #comment #detail #infos{
    margin: 10px;
    margin-bottom: 10px;
}
#theblog #center #comment #detail #info{
    display: inline-block;
    font-size: 14px;
}
#theblog #center #comment #detail #content {
    margin: 10px;
}
#theblog #reply #messagenote {
    color: fuchsia;
}
#theblog #editor {
    margin: 0 auto;
    height: 333px;
}
#theblog iframe {
    margin: 1rem auto;
    padding: 0.1rem;
}
#theblog img {
        margin: 1rem auto;
        padding: 0.1rem;
        width: 100%;
}
#theblog pre {
    display: block;
    padding: 8px;
    margin: 5px 0;
    font-size: 13.3px;
    line-height: 1.5;
    color: var(--purple);
    word-break: break-all;
    word-wrap: break-word;
    background-color: #f5f5f5;
    border: 1px solid rgb(246, 226, 252);
    text-shadow: none;
}

#theblog code {
    padding: 2px 4px;
    font-size: 90%;
    background-color: #f5f5f5;
    border-radius: 4px;
    border: 1px solid #ccc;
    color: var(--purple);
    text-shadow: none;
}

#theblog pre code {
    padding: 0;
    font-size: inherit;
    color: inherit;
    white-space: pre-wrap;
    background-color: transparent;
    border-radius: 0;
    border: 0;
}
@media only screen and (max-width: 600px) {
    #theblog {
        margin: 2vh auto;
        width: 97%;
    }
    #theblog iframe {
        width: 350px;
    }
    #theblog #mei {
        margin: -1vh 0 1vh;
        padding: 1rem;
        background-color: #59C173;
    }
    #theblog #mei h1 {
        text-align: center;
        padding: 3rem 1rem;
        font-size: 1.8rem;
        vertical-align: middle;
    }
    #theblog #mei #title #save, #theblog #mei #title #saved {
        font-size: 0.8rem;
        margin-left: 0.8rem;
        padding: 0.8vh 0.9vw 0.3vh 0.7vw;
    }
    #theblog #mei #title #save {
        border: 0.1px solid fuchsia;
        color: fuchsia;
    }
    #theblog #mei #title #saved {
        color: green;
    }
    #theblog #mei #title #like {
        font-size: 0.85rem;
        margin-left: 1rem;
        color: fuchsia;
    }
    #theblog #mei #title #right {
        float: right;
        font-size: 0.85rem;
        margin-right: 0.8rem;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    #theblog main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
    #theblog iframe {
        width: 455px;
    }
    #theblog #mei {
        padding: 1rem;
        background-color: #59C173;
    }
    #theblog #mei h1 {
        text-align: center;
        margin: 4rem 2rem;
        font-size: 1.8rem;
        vertical-align: middle;
    }
    #theblog #mei #title #save, #theblog #mei #title #saved {
        font-size: 0.9rem;
        margin-left: 2rem;
        padding: 0.8vh 0.5vw 0.3vh 0.4vw;
    }
    #theblog #mei #title #save {
        border: 0.1px solid fuchsia;
        color: fuchsia;
    }
    #theblog #mei #title #saved {
        color: green;
    }
    #theblog #mei #title #like {
        font-size: 0.9rem;
        margin-left: 1rem;
        color: fuchsia;
    }
    #theblog #mei #title #right {
        float: right;
        font-size: 1rem;
        margin-right: 2rem;
    }
}
@media only screen and (min-width: 850px) {
    #theblog {
        margin: 0 auto;
        width: 66%;
        padding-top: 77px;
    }
    #theblog iframe {
        width: 960px;
    }
    #theblog #mei {
        padding: 1rem;
        background-color: #59C173;
    }
    #theblog #mei h1 {
        text-align: center;
        margin: 6rem 3rem;
        font-size: 2rem;
        vertical-align: middle;
    }
    #theblog #mei #title #save, #theblog #mei #title #saved {
        font-size: 0.9rem;
        margin-left: 4rem;
        padding: 0.8vh 0.3vw 0.3vh 0.2vw;
    }
    #theblog #mei #title #save {
        border: 0.1px solid fuchsia;
        color: fuchsia;
    }
    #theblog #mei #title #saved {
        color: green;
    }
    #theblog #mei #title #like {
        font-size: 0.9rem;
        margin-left: 2rem;
        color: fuchsia;
    }
    #theblog #mei #title #right {
        float: right;
        font-size: 0.9rem;
        margin-right: 2rem;
    }
}
</style>