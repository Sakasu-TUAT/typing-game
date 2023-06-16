<template>
  <!-- <img alt="Vue logo" src="./assets/logo.png"> -->
  <HelloWorld msg="Welcome to Your Vue.js App"/>
  <TypingGame /> <!-- TypingGameコンポーネントを呼び出す -->
</template>

<script>
import TypingGame from './components/TypingComponent.vue';

export default {
  name: 'App',
  components: {
    TypingGame
  }
}
</script>

