<template>
    <div id="side">
        <main>
            <h3>What</h3>
            <p>Chania is a city on the island of Crete.</p>
            <h3>Where</h3>
            <p>Crete is a Greek island in the Mediterranean Sea.</p>
            <h3>How</h3>
            <p>You can reach Chania airport from all over Europe.</p>

        </main>
    </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'side'
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
   main{
        padding: 0.6rem;
        border: 1px solid rgb(212, 212, 212);
    }
}
</style>