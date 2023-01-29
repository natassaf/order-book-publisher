<template>
  <header> Spread Monitor</header>

  <body>
    <div>
      <form @submit.prevent="submitForm">
        <div class="box">

          <h3> Select market tokens symbol </h3>

          <!-- <div v-for="token in availableTokens" :key="token">
            <input type="checkbox" :id="token" :value="token" v-model="selectedTokens">
            <label>{{ token }}</label>
          </div> -->
          <div>
            <input type="text"  v-model.trim="symbol">
          </div>
          <br>
          <div>
            <button @click="saveSymbol">Submit</button>
          </div>

        </div>
      </form>

      <div class="box">
        <b>Exchanges: </b> Binance, Bitstamp <br><br>
        <b>Selected tokens: </b> {{ selectedSymbolFlag? selectedSymbol : ""}} <br> <br>
        <div v-if="selectedSymbol">
          <button @click="requestOrderBook"> Request order book </button>
          {{ loadingFlag? "Loading...": "" }}
        </div>
        <br><br>
        <h3>Spread: {{ spread }}</h3>
        <div v-if="items"><order-book-visualizer :items="items"></order-book-visualizer></div>


      </div>
    </div>

  </body>
</template>

<script>
import OrderBookVisualizer from './components/OrderBookVisualizer.vue';
import { OrderbookAggregatorClient } from './order-book_grpc_web_pb';
import {BookSummaryRequest } from "./order-book_pb";


export default {
  name: 'App',
  components: {
    OrderBookVisualizer
  },
  data() {
    return {
      selectedSymbolFlag: false,
      connectionFlag: false,
      loadingFlag: false,
      orderBookResponse: false,
      response: null,
      items: [],
      spread: null,
      symbol:null,
      selectedSymbol:null
    }
  },
  methods: {
    saveSymbol(){
      this.selectedSymbol = this.symbol; 
      this.selectedSymbolFlag = true;
    }
    ,
    requestConnection() {
      this.connectionFlag = true;
    },
    requestOrderBook() {
      this.loadingFlag = true;
      this.orderBookResponse = true;
      let  bookSummaryRequest = new BookSummaryRequest();
      bookSummaryRequest.setSymbol(this.selectedSymbol);
      this.response = this.client.bookSummary(bookSummaryRequest).on("data",
        response => {
          let spread = response.getSpread();
          let bids = response.getBidsList();
          let asks = response.getAsksList();

          let items = []
          for (let i = 0; i <asks.length ; i++){
            let level = {"exchange": asks[i].getExchange(), "price": asks[i].getPrice(), "amount": asks[i].getAmount(), "order_type": "ask"};
            items.push(level);
          }

          for (let i = 0; i <bids.length ; i++){
            let level = {"exchange": bids[i].getExchange(), "price": bids[i].getPrice(), "amount": bids[i].getAmount(), "order_type": "bid"};
            items.push(level);
          }

          this.items = items;
          this.spread = spread;
          this.loadingFlag=false;
        });

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
  created: function () {
    console.log("Created cliend running at 14586")
    this.client = new OrderbookAggregatorClient("http://localhost:14586", null, null);
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
