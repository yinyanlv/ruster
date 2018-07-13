<template>
    <div id="bestside">
        <main>
            <div id="show">
                <h4>最美的人</h4>
                <div id="title">
                    <li><h5>最近最美</h5></li><li>&emsp;<strong>|</strong>&emsp;</li><li><h5>一直最美</h5></li>
                </div>
                <ul>
                    <li><p>最美</p></li><li>&emsp;<strong>|</strong>&emsp;</li><li><p>最美</p></li>
                </ul>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'bestside'
}
</script>

<style scoped>
main{
    background-color: #ffffff;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 1.5vh auto;
        padding: 0.5rem;
        border: 1px solid rgb(212, 212, 212);
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
   main{
        padding: 0.6rem;
        border: 1px solid rgb(212, 212, 212);
    }
}
@media only screen and (min-width: 850px) {
   #show h4 {
       padding: 0.8rem;
       text-align: center;
       border-bottom: 2px solid #acc;
    }
    #show #title, #show ul {
        line-height: 2rem;
        text-align: center;
        border-bottom: 2px solid rgb(206, 209, 209);
    }
    #show li {
        display: inline-block;
    }
}
</style>