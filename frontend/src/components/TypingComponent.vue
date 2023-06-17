<template>
    <div id="app" class="container">
      <div class="title">
        <h1>たいぴんぐ.</h1>
        <div class="marker"></div>
      </div>
      <!-- Assigning multiple classes to a button -->
      <button v-if="currentGameState == gameState.READY" class="startButton mb-20" @click="gameStart">START</button>
      <div v-if="currentGameState != gameState.READY">
        <div class="question mb-20">{{ currentQuestion }}</div>
        <div v-if="currentQuestionCounts == totalQuestionNum" class="clear">Clear!</div>
        <div class="typeFormWrapper mb-20">
      <!-- The typed characters will be linked to typeBox -->
      <input id="typeForm" v-model="typeBox" type="text" class="typeForm" />
        </div>
        <div class="gaugeWrapper mb-20">
          <div :style="styleObject" class="gauge"></div>
        </div>
  
        <p>Time: {{ formatTime(elapsedTime) }}</p>
        <div v-if="currentGameState!=gameState.CLEARED" class="mb-20">
          {{ currentQuestionCounts }}/{{ totalQuestionNum }}
        </div>

        <div v-if="currentGameState==gameState.CLEARED">
          <p> {{username}}'s rank is {{ rank }} th</p>
          <button class="rankButton mb-20" @click="handleRankingButton">RANKING</button>
          <div v-if="isRankingShown" class="ranking-container" ref="rankingContainer">
            <transition-group name="ranking-item" tag="div">
              <div v-for="(ranking, index) in rankings" :key="index" class="ranking-item">
                {{ "Top " + (index + 1) + " - " + ranking + "s"}}
              </div>
            </transition-group>
            <div class="yourScore" ref="yourScore">
              {{ "Top " + (this.rank) + " - " + this.username + " : " + this.score + " s"}}
            </div>
          </div>
        </div>

        <button class="resetButton mb-20 " @click="gameReset">RESET</button>   
        
          
      </div>
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
        // "hello",
        // "apple",
        // "orange",
        // "banana",
        // "grape",
        // "peach",
        // "otukaresamadeshita",
        // "oyasuminasai"
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
      score: 0,
      rank: null,
      error: null,

      isRankingShown: false,
      showRankingNum: 10,
      rankings: [],
      backendUrl: "http://localhost:8000",
      // backendUrl: process.env.VUE_APP_BACKEND_URL,
      username: "Player",
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
    async postScore() {
      console.log("[Post URL]: " + `${this.backendUrl}/users`);
      try {
       //!@note The name of the message must be spelled the same as on the backend side.
        const response = await axios.post(`${this.backendUrl}/users`, {
          username: this.username,
          score: this.score,
          created_at: new Date(),
        });
        console.log(response.data);
      } catch (error) {
        console.error(error);
      }
    },
    async fetchScoreRank() {
      console.log("[Fetch URL]: " + `${this.backendUrl}/score_rank/${this.score}`);
      try {
        const response = await axios.get(`${this.backendUrl}/score_rank/${this.score}`);
        this.rank = response.data;
        this.error = null;
      } catch (error) {
        this.rank = null;
        this.error = 'Failed to fetch score rank';
        console.error(error);
      }
      console.log("rank is " + this.rank);
    },
    scrollRankingContainer() {
      const container = this.$refs.rankingContainer;
      container.scrollTop = container.scrollHeight;
    },
    scrollYourScore() {
      if (this.isRankingShown) {
        this.$nextTick(() => {
          this.scrollRankingContainer();
          setTimeout(() => {
            const yourScore = this.$refs.yourScore;
            yourScore.scrollIntoView({ behavior: "smooth", block: "end" });
          },100);
        });
      }
    },
    handleRankingButton(){
      this.featchRanking();
      this.toggleRanking();
      this.scrollYourScore();
    },
    async featchRanking() {
      console.log("[Fetch URL]: " + `${this.backendUrl}/ranking/${this.showRankingNum}`);
      try {
        const response = await axios.get(`${this.backendUrl}/ranking/${this.showRankingNum}`);
        this.rankings = response.data.map(player => player.username + ": " + player.score);
        this.error = null;
      } catch (error) {
        this.rankings = null;
        this.error = 'Failed to fetch score rank';
        console.error(error);
      }
      console.log("rankings are " + this.rankings);
    },
    toggleRanking() {
      this.isRankingShown = !this.isRankingShown;
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
      this.score = this.elapsedTime;
      this.postScore();
      this.fetchScoreRank();
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
      this.toggleRanking();
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
    this.$nextTick(() => {
      if (this.isRankingShown) {
        this.scrollRankingContainer();
        this.scrollYourScore();
        setInterval(this.scrollRankingContainer, 1000);
      }
    });
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
    },
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

/* This is a commonly used method to secure margins for buttons. */
.mb-20{
    margin-bottom: 20px;
}

.container{
    width: 400px;
    /* The game is centered in the middle of the screen. */
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
   /* absolute: specifies the position absolutely to the reference position of the parent element, which is the title. */
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
    /* Changes over 0.3 seconds. */
    transition: all .3s ease; 
}

.rankButton{
    background-color: #eba21b;
    color: #fff;
    padding: 4px 60px;
    border-radius: 8px;
    border: none;
    outline: none;
    cursor: pointer;
}
.rankButton:hover{
    opacity: 0.7;
    background-color: #eba21b;
}

.ranking-container {
  display: flex;
  flex-direction: column;
  align-items: center;
}


@keyframes ranking-item {
  0% {
    opacity: 0;
    transform: translateY(30px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.ranking-item {
  font-size: 24px;
  animation: ranking-item 1.0s ease-in-out;
}

.yourScore {
  font-size: 24px;
  font-weight: bold;
  color: #FFFFFF;
  display: inline-block; /* テキストのある部分のみを含むインラインブロック要素にする */
  background-color: rgb(233, 161, 7);
  padding-left: 5px;
  padding-right: 5px;
  margin: 0 auto; /* 水平方向に中央に配置 */
  margin-bottom: 50px;
  border-radius: 5px;
  animation: ranking-item 1.5s ease-in-out;
}

</style>
