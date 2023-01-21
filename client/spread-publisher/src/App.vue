<template>
  <header> Spread Monitor</header>

  <body>
    <form @submit.prevent="submitForm">
      <div class="box">

        <h3> Select market tokens </h3>

        <div v-for="token in availableTokens" :key="token">
          <input type="checkbox" :id="token" :value="token" v-model="selectedTokens">
          <label>{{ token }}</label>
        </div>
        <br>
        <div>
          <button @click="saveTokenChoices">Submit</button>
        </div>

      </div>
    </form>

    <div class="box">
      <b>Exchanges: </b> Binance, Bitstamp <br><br>
      <b>Selected tokens: </b> {{ selectedTokensFlag? tokens.join(" ") : ""}} <br> <br>
      <button @click="requestConnection"> Connect with exchanges</button> <br><br>
      <div v-if="connectionFlag">
        <button @click="requestOrderBook"> Request order book </button>
        {{ loadingFlag? "Loading...": "" }}
      </div>
      <br><br>
      <div v-if="orderBookResponse"><order-book-visualizer></order-book-visualizer></div>


    </div>


  </body>
</template>

<script>
import OrderBookVisualizer from './components/OrderBookVisualizer.vue';
import { OrderbookAggregatorClient } from './order-book_grpc_web_pb';
import { Empty } from "./order-book_pb";


export default {
  name: 'App',
  components: {
    OrderBookVisualizer
  },
  data() {
    return {
      availableTokens: ["ETHBTC"],
      selectedTokens: [],
      selectedTokensFlag: false,
      tokens: [],
      connectionFlag: false,
      loadingFlag: false,
      orderBookResponse: false,
      response:null
    }
  },
  methods: {
    saveTokenChoices() {
      this.tokens = this.selectedTokens;
      this.selectedTokensFlag = !this.selectedTokensFlag;
    },
    requestConnection() {
      this.connectionFlag = true;
    },
    requestOrderBook() {
      this.loadingFlag = true;
      this.orderBookResponse = true;
      this.response = this.client.bookSummary(new Empty());
      alert("BookSummmaryResponse: "+ this.response);
      
    }

  },
  watch: {
    selectedTokens(newTokens, oldTokens) {
      console.log(newTokens);
      if (newTokens.length > 2) {
        this.selectedTokens = oldTokens;
        alert("Please select only 2 tokens");
      }
    }
  },
  created:function(){
    console.log("Created cliend running at 14586")
    this.client  = new OrderbookAggregatorClient("http://localhost:14586", null, null);
  }

}
</script>

<style>
* {
  box-sizing: border-box;
}

.form-control {
  margin: 0.5rem 0;
}

button {
  font: inherit;
  border: 2px solid #2e2759;
  background-color: #2e2759;
  color: white;
  cursor: pointer;
  padding: 0.3rem 1.2rem;
  border-radius: 3px;
}

input[type='text'] {
  display: block;
  width: 20rem;
  margin-top: 0.5rem;
}

.box {
  display: block;
  width: 500px;
  padding: 8px;
  border: 5px solid gray;
  margin: 1rem auto;
  background-color: whitesmoke;
}

html {
  font-family: Arial, Helvetica, sans-serif;
}

body {
  margin: 5;
  background-color: #ccc;
  font-family: Arial, Helvetica, sans-serif;
}

header {
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.26);
  margin: 1rem auto;
  border-radius: 15px;
  padding: 1.5rem;
  background-color: #2e2759;
  color: white;
  text-align: center;
  width: 90%;
  max-width: 50rem;
  font-family: "Copperplate", "Courier New", monospace;
}


#events button:hover,
#events button:active {
  background-color: #ec3169;
  border-color: #ec3169;
  box-shadow: 1px 1px 4px rgba(0, 0, 0, 0.26);
}
</style>
