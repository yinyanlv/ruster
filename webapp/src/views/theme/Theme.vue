<template>
    <div id="theme">
        <main id="main">
            <div id="container">
                <div id="center">
                    <div id="theme">
                        <div id="title">
                            <h2> {{ theme.title }} </h2> 
                            <span id="info"><a :href="'/a/home/' + theme_category_name">{{ theme_category_name_cn }}</a></span> • 
                            <span id="info"><a :href="'/a/user/' + theme_user.id">{{ theme_user.username }}</a></span> •   
                            <span id="info">{{ theme_rtime }}</span>  
                        </div>
                        <div id="content" v-html="theme.content" ></div>
                    </div>
                    <hr>
                    <div id="comment">
                        <div id="count">评论 &nbsp; {{ theme.comment_count }} </div>
                        <div v-for="(comment, index) in theme_comments" :key="index">
                            <div id="detail">
                                <div id="infos">
                                    <span id="info" >{{ index + 1 }}&nbsp;</span>
                                    <span id="info"><a :href="'/a/user/' + comment.user_id">{{ comment.username }}</a></span> • 
                                    <span id="info">{{ comment.rtime }}</span>
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
                <side></side>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import { mavonEditor } from 'mavon-editor'
import 'mavon-editor/dist/css/index.css'
import URLprefix from '../../config'
import Side from '../../components/side/Side'
export default {
    name: 'theme',
    components: {
        "side": Side,
         mavonEditor
    },
    data: function() {
        return {
            Content: '',
            theme: '',
            theme_user: '',
            theme_category_name: '',
            theme_category_name_cn: '',
            theme_rtime: '',
            theme_comments: '',
            signin_user: '',
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
        if (sessionStorage.getItem('signin_user')){
            this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
        }
        fetch(URLprefix + 'api/theme/'+ this.$route.params.id,{
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            this.theme = json.theme
            this.theme_user = json.theme_user
            this.theme_rtime = json.theme_rtime
            this.theme_category_name_cn = json.theme_category_name_cn
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
    }
  }
}
</script>

<style>
#main #center {
    background-color: #ffffff;
}
#main a {
    color: #0541af;
}
#main #center #theme, #main #center #comment, #main #center #reply {
    border-top: 1px solid fuchsia;
}
#main #center #theme > #title {
    margin-top: 2px;
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#main #center #theme > #title h2 { 
    padding-bottom: 0.3rem;
}
#main #center #theme > #title #info {
    display: inline-block;
    font-size: 14px;
}
#main #center #theme > #content {
    margin: 10px;
}
#main hr {
    height: 11px;
    background-color: #faf5f5;
    border: 0;
}
#main #center #comment > #count {
    font-weight: bold;
    color: fuchsia;
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#main #center #comment #detail {
    border-bottom: 1px solid rgb(223, 223, 223);
}
#main #center #comment #detail #infos{
    margin: 10px;
    margin-bottom: 10px;
}
#main #center #comment #detail #info{
    display: inline-block;
    font-size: 14px;
}
#main #center #comment #detail #content {
    margin: 10px;
}
#main #editor {
    margin: auto;
    height: 333px;
}
#main #reply #messagenote {
    color: fuchsia;
}
#main iframe {
    margin: 1rem auto;
    padding: 0.1rem;
}
#main img {
        margin: 1rem auto;
        padding: 0.1rem;
        width: 100%;
}
#main pre {
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

#main code {
    padding: 2px 4px;
    font-size: 90%;
    background-color: #f5f5f5;
    border-radius: 4px;
    border: 1px solid #ccc;
    color: var(--purple);
    text-shadow: none;
}

#main pre code {
    padding: 0;
    font-size: inherit;
    color: inherit;
    white-space: pre-wrap;
    background-color: transparent;
    border-radius: 0;
    border: 0;
}
@media only screen and (max-width: 600px) {
    #main  {
       margin: 2vh auto;
       width: 97%;
    }
    #main iframe {
        width: 350px;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    #main {
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
    #main #container {
      display: flex;
      flex-flow: row;
    }
    #main #container #center {
        width: 90%;
        margin-right: 1vw;
    }
    #main #container #side {
        flex: 1;
    }
    #theblog iframe {
        width: 455px;
    }
}
@media only screen and (min-width: 850px) {
    #main {
        margin: 0 auto;
        width: 75%;
        padding-top: 77px;
    }
    #main #container {
      display: flex;
      flex-flow: row;
    }
    #main #container #center {
        width: 80%;
        margin-right: 1vw;
    }
    #main #container #side {
        flex: 1;
    }
    #theblog iframe {
        width: 960px;
    }
}
</style>