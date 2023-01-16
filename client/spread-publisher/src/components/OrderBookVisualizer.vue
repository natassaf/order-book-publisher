<template>
    <EasyDataTable :headers="headers" :items="items" />
</template>

<script>
export default {
    setup() {
        const headers = [
            { text: "Exchange", value: "exchange" },
            { text: "Price <name of currency>", value: "amount"},
            { text: "Amount", value: "amount" },
            { text: "Order type", value: "order_type", sortable: true }
        ];
        const items = [
            { "exchange": "binance", "price": 8491.25, "amount": 0.008, "order_type": "ask" },
            { "exchange": "coinbase", "price": 8496.37, "amount": 0.0303, "order_type": "ask" },
            { "exchange": "binance", "price": 8488.53, "amount": 0.002, "order_type": "bid" }
        ];
        return {
            headers,
            items
        }
    },

    data() {
        return {
            my_items: [
                { age: 40, first_name: 'Dickerson', last_name: 'Macdonald' },
                { age: 21, first_name: 'Larsen', last_name: 'Shaw' },
                { age: 89, first_name: 'Geneva', last_name: 'Wilson' },
                { age: 38, first_name: 'Jami', last_name: 'Carney' }
            ]
        }
    }
}
</script>

<style scoped>

</style>