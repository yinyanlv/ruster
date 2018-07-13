<template>
    <div id="post">
        <main>
            <div id="container">
                <div id="center">
                    <div id="new-title"><p>新主题</p></div>
                    <form id="form" >
                            <div id="topic-group">
                                <span  id="category">
                                        <select v-if="username == 'admin'" name="category_name" v-model="CategoryName" id="category-control" >
                                            <option v-bind:value="category_name" v-for="(category_name, index) in category_names_admin" :key="index">
                                                    {{category_name}}
                                            </option>
                                        </select>
                                        <select v-else name="category_name" v-model="CategoryName" id="category-control">
                                            <option v-bind:value="category_name" v-for="(category_name, index) in category_names" :key="index">
                                                    {{category_name}}
                                            </option>
                                        </select>
                                </span>
                                <span id="title">
                                        <input type="text" name="title" v-model="Title" placeholder="Please input title">
                                </span>
                            </div>    
                            <div id="editor">
                                <mavon-editor name="content" v-model="Content" :ishljs = "true" style="height: 100%;" :toolbars="set"></mavon-editor>
                            </div>
                            <div id="new">
                                    <button type="submit" id="submit" @click="post" >发布</button>
                            </div>
                    </form>
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
    name: 'post',
    components: {
        "side": Side,
        mavonEditor
    },
    data () {
        return {
            username: '',
            category_names: '',
            category_names_admin: '',
            CategoryName: '',
            Title: '',
            Content: '',
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
        var username = JSON.parse(sessionStorage.getItem('signin_user')).username
        this.username = username
              fetch(URLprefix + 'api/categorys', {
                  method: 'GET',
              }).then(response => response.json())
              .then(json => {
                    this.categorys = json.categorys
                    let category_names_admin = []
                    let category_names = []
                    this.categorys.map((item) => category_names_admin.push(item.category_name_cn))
                    this.category_names_admin = category_names_admin
                    category_names_admin.filter((item) => { if (item != '官方') category_names.push(item)})
                    this.category_names = category_names
              })
              .catch((e) => {
                console.log(e)
              })
    },
    methods: {
        post() {
            let category_name = this.CategoryName
            let title = this.Title
            let content = this.Content
            let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            if(category_name == ''){
                alert("主题分类不能为空, 请选择一个分类")
                return
            }else if(title == ''){
                alert("标题不能为空")
                return
            }else if(content == ''){
                alert("内容不能为空")
                return
            }else{
                let data = { 
                    user_id: user_id,
                    category_name: category_name,
                    title: title,
                    content: content
                }
                fetch(URLprefix + 'api/theme_new', {
                    body: JSON.stringify(data), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
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

<style scoped>
#center {
  background-color: #FFFFFF;
}
#center #new-title {
    width: 100%;
    line-height: 33px;
    border :1px solid #CAC1C1;
    background-color:#f5fdfa;
}
#center form #topic-group {
   margin: 11px 0 11px 0;
}
#center form #topic-group #category #category-control {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    padding-left: 9px;
}
#center form #topic-group #category #category-control, #center form #topic-group input {
    height: 30px;
}
#center #editor {
    margin: auto;
    height: 444px;
}
#center form #new button {
    margin-top: 0.3rem;
    width:63px; 
    line-height:25px;
    background-color:#FFFFFF;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 2vh auto;
        width: 97%;
    }
    #center form #topic-group #category #category-control, #center form #topic-group input {
        width: 100%;
    }
    #center form span {
        display: block;
        margin-top: 1vh;
        border: 1px solid #e4e3e3;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 90%;
        margin-right: 1vw;
    }
    #container #side {
        flex: 1;
    }
    #center form #topic-group input {
        width: 80%;
        float: right;
    }
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 75%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1vw;
    }
    #container #side {
        flex: 1;
    }
     #center form #topic-group input {
        width: 86%;
        float: right;
    }
}
</style>