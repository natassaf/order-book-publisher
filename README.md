#order-book-publisher

# Order Book Publisher
An order book and spread publisher for binance and bitstamp exchanges. The client exposes a simple form where the user can enter a symbol name and retrieve a sample of the combined order book containing the lowest 10 asks and the highest 10 bids from both exchanges.

## Client
*************

#### Usage

``` 
npm install 
npm run serve 

cd ../../
protoc --proto_path=protos --js_out=import_style=commonjs,binary:client/spread-publisher/src/ --grpc-web_out=import_style=commonjs,mode=grpcwebtext:client/spread-publisher/src/ protos/order-book.proto

```

## Server
*************

#### Usage
 
``` 
cargo build 
cargo run --bin order-book-server 
```