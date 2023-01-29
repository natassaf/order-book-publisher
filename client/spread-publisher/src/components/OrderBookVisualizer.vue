<template>
    <EasyDataTable  :headers="headers" :items="items" />
</template>

<script>
export default {

    props: ["items"],
    data() {
        return {
            headers: [
                { text: "Exchange", value: "exchange" },
                { text: "Price", value: "price" },
                { text: "Amount", value: "amount" },
                { text: "Order type", value: "order_type", sortable: true }
            ]
        }
    },
}
</script>

<style scoped>

</style>