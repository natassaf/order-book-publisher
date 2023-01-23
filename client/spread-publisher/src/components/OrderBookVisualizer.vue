<template>
    <EasyDataTable :headers="headers" :items="items" />
</template>

<script>
export default {

    props: ["items"],
    data() {
        return {
            // items: [
            //     { "exchange": "binance", "price": 8491.25, "amount": 0.008, "order_type": "ask" },
            //     { "exchange": "coinbase", "price": 8496.37, "amount": 0.0303, "order_type": "ask" },
            //     { "exchange": "binance", "price": 8488.53, "amount": 0.002, "order_type": "bid" }
            // ],
            headers: [
                { text: "Exchange", value: "exchange" },
                { text: "Price <name of currency>", value: "price" },
                { text: "Amount", value: "amount" },
                { text: "Order type", value: "order_type", sortable: true }
            ]
        }
    }
}
</script>

<style scoped>

</style>