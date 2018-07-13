<template>
    <div id="help">
        <main>
            <div id="content">
                <div id="support">
                    <h2>支持Ruster</h2>
                    <p>Ruster 是一个社区项目，<a href="https://github.com/ruster-xyz/ruster">源码</a>&nbsp;他每月需要费用花在云服务器托管，如果你想帮助这个项目和希望他有个光明的未来，清给予捐助，谢谢！
                    目前你可以联系krircc@aliyun.com</p>
                </div><br>
                <div id="info">
                    <h3>捐助人组织和个人</h3>
                    <p>..&emsp;100</p>
                    <p>..&emsp;66.66</p>
                </div>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
    name: 'help'
}
</script>

<style scoped>
a {
  color: #0541af;
}
@media only screen and (max-width: 600px) {
    main  {
      margin: 0 auto;
      width: 90%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
}
</style>