#order-book-publisher

# Order Book Publisher
2
A system composed of a client and a server. The client is writen in VueJS and provides an very basic interface were the user can select 
 tokens and exchanges and retrieve an order book with the top ten bids and top ten asks and the spread.

## Client
*************

#### Usage

``` 
npm install 
npm run serve 

cd ../../
protoc --proto_path=protos --js_out=import_style=commonjs,binary:client/spread-publisher/src/ --grpc-web_out=import_style=commonjs,mode=grpcwebtext:client/spread-publisher/src/ protos/order-book.proto

```

*** Deploy Envoy Docker Container ***
``` 
cd envoy
docker build -t envoy-img .
sudo docker run  -p 8080:8080 --net=host  envoy-img

``` 

## Server
*************

#### Usage
 
``` 
cargo build 
cargo run --bin order-book-server 
cargo run --bin 
```