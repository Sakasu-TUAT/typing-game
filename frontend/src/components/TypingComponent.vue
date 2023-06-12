<template>
    <div id="app" class="container">
      <div class="title">
        <h1>たいぴんぐ.</h1>
        <div class="marker"></div>
      </div>
      <!-- ボタンにクラスを複数割り当てている -->
      <button v-if="currentGameState == gameState.READY" class="startButton mb-20" @click="gameStart">START</button>
      <div v-if="currentGameState != gameState.READY">
        <div class="question mb-20">{{ currentQuestion }}</div>
        <div v-if="currentQuestionCounts == totalQuestionNum" class="clear">Clear!</div>
        <div class="typeFormWrapper mb-20">
          <!-- タイプした文字がtypeBoxと連動するようになる -->
          <input id="typeForm" v-model="typeBox" type="text" class="typeForm" />
        </div>
        <div class="gaugeWrapper mb-20">
          <div :style="styleObject" class="gauge"></div>
        </div>
  
        <p>Time: {{ formatTime(elapsedTime) }}</p>
        <div class="mb-20">{{ currentQuestionCounts }}/{{ totalQuestionNum }}</div>
        <button class="resetButton mb-20" @click="gameReset">RESET</button>
      </div>
      <!-- <input type="text" v-model="message" placeholder="Enter a message" />
      <button @click="sendMessage">Send</button> -->
    </div>
  </template>
  
  <script>
  import axios from "axios";

export default {
  data() {
    return {
      startFlg: "",
      currentQuestion: "",
      questions: [
        "a",
        "a",
        // "orange",
        // "banana",
        // "grape",
        // "peach"
      ],
      currentQuestions: [],
      typeBox: "",
      currentQuestionCounts: 0,
      totalQuestionNum: 0,
      timer: {
        startTime: null,
        elapsedTime: 0
      },
      gameState: {
        READY: "ready",
        PLAYING: "playing",
        CLEARED: "cleared",
      },
      currentGameState: null,
      messages: [],
      message: '',
    };
  },
  computed: {
    styleObject() {
      const width = this.currentQuestionCounts * (100 / this.totalQuestionNum) + "%";
      const color = this.currentQuestionCounts === this.totalQuestionNum ? "#03a9f4" : "orange";
      return {
        width: width,
        backgroundColor: color
      };
    },
  },
  methods: {
    async sendMessage() {
      try {
        // 環境変数からバックエンドのURLを取得する
        const backendUrl = process.env.BACKEND_URL;
        // const backendUrl = "http://localhost:8000"; //ローカルの場合
        // バックエンドのURLにメッセージをPOSTする
        const response = await axios.post(`${backendUrl}/message`, {
          message: this.message,
        });
        // レスポンスをコンソールに表示する
        console.log(response.data);
      } catch (error) {
        // エラーが発生した場合はコンソールに表示する
        console.error(error);
      }
    },
    updateState(state) {
      this.currentGameState = state;
    },
    gameStart() {
      this.startTimer();
      setTimeout(() => {
        document.getElementById("typeForm").focus();
      }, 0);
      this.updateState(this.gameState.PLAYING);
    },
    gameClear() {
      this.updateState(this.gameState.CLEARED);
      this.message = this.formatTime(this.elapsedTime)
      this.sendMessage();
      clearInterval(this.timer);
    },
    config() {
      this.currentQuestions = Array.from(this.questions);
      this.currentQuestion = this.currentQuestions[0];
      this.totalQuestionNum = this.questions.length;
      this.currentQuestionCounts = 0;
      this.updateState(this.gameState.READY);
    },
    gameReset() {
      this.config();
      clearInterval(this.timer);
    },
    startTimer() {
      this.startTime = new Date();
      this.elapsedTime = 0;
      this.timer = setInterval(() => {
        this.updateTimer();
      }, 1);
    },
    updateTimer() {
      this.$forceUpdate();
      const currentTime = new Date();
      const elapsedMilliseconds = currentTime - this.startTime;
      this.elapsedTime = Math.floor(elapsedMilliseconds / 10) / 100;
      console.log("elapsedTime: " + this.elapsedTime + "ms");
    },
    formatTime(time) {
      const minutes = Math.floor(time / 60);
      const seconds = Math.floor(time % 60);
      const milliseconds = Math.floor((time % 1) * 1000);
      return `${this.padZero(minutes)}:${this.padZero(seconds)}:${this.padZero(milliseconds, 3)}`;
    },
    padZero(number, length = 2) {
      return number.toString().padStart(length, '0');
    },
  },
  mounted() {
    this.config();
  },
  watch: {
    typeBox(e) {
      if (e === this.currentQuestion) {
        this.currentQuestions.splice(0, 1);
        this.currentQuestion = this.currentQuestions[0];
        this.typeBox = "";
        this.currentQuestionCounts += 1;
      }
    },
    currentQuestionCounts(newValue) {
      if (newValue === this.totalQuestionNum) {
        this.gameClear();
      }
    }
  },
};
</script>

<style>
*{
    font-family: inherit;
    font-size: inherit;
    line-height: inherit;
}

body{
    font-size: 32px;
}

/* ボタンのマージン確保のためによく使われる方法らしい */
.mb-20{
    margin-bottom: 20px;
}

.container{
    width: 400px;
    /* ゲームが全体中央に寄る */
    margin: 0 auto;
    text-align: center;
}

.title{
    position: relative;
    font-size: 48px;
}

.marker{
    width: 100%;
    height: 35%;
    background-color: rgb(7, 195, 233);
    /* absolute: 親要素であるtitleの基準位置に対して絶対的に位置を指定 */
    position: absolute;
    bottom: 5%;
    z-index: -1;
}

.startButton{
    background-color: #333;
    color: #fff;
    padding: 4px 60px;
    border-radius: 8px;
    border: none;
    outline: none;
    cursor: pointer;
}
.startButton:hover{
    opacity: 0.7;
    background-color: #a31010;
}
.resetButton{
    background-color: #333;
    color: #fff;
    padding: 4px 60px;
    border-radius: 8px;
    border: none;
    outline: none;
    cursor: pointer;
}
.resetButton:hover{
    opacity: 0.7;
    background-color: #0d1c69;
}

.question{
    color: rgb(103, 86, 86);
    font-weight: bold;
}
.clear{
    color: #03a9f4;
    font-size: 45px;
    font-weight: bold;
    /* color: yellow; */
    /* -webkit-text-stroke: 1.5px rgb(0, 0, 0); */
}
.typeForm{
    text-align: center;
    outline: none;
    border: none;
}
.typeFormWrapper{
    border-bottom: 1px solid #333;
}
.gaugeWrapper{
    border: 1px solid #333;
    height: 12px;
}
.gauge{
    height: 12px;
    /* 0.3秒かけて変化する */
    transition: all .3s ease; 
}
</style>
