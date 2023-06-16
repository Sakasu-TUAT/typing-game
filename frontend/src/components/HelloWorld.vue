<template>
  <div>
    <input type="text" v-model="message" />
    <button @click="sendMessage">Send</button>
  </div>
</template>

<script>
import axios from "axios";

export default {
  data() {
    return {
      message: "",
    };
  },
  methods: {
    async sendMessage() {
      try {
        // バックエンドのURLにメッセージをPOSTする
        const response = await axios.post("http://localhost:8000/message", {
          message: this.message,
        });
        // レスポンスをコンソールに表示する
        console.log(response.data);
      } catch (error) {
        // エラーが発生した場合はコンソールに表示する
        console.error(error);
      }
    },
  },
};
</script>
