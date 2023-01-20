/**
 * @fileoverview gRPC-Web generated client stub for orderbook
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.4.2
// 	protoc              v3.20.3
// source: order-book.proto


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.orderbook = require('./order-book_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.orderbook.OrderbookAggregatorClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.orderbook.OrderbookAggregatorPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.orderbook.Empty,
 *   !proto.orderbook.Summary>}
 */
const methodDescriptor_OrderbookAggregator_BookSummary = new grpc.web.MethodDescriptor(
  '/orderbook.OrderbookAggregator/BookSummary',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.orderbook.Empty,
  proto.orderbook.Summary,
  /**
   * @param {!proto.orderbook.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.orderbook.Summary.deserializeBinary
);


/**
 * @param {!proto.orderbook.Empty} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.orderbook.Summary>}
 *     The XHR Node Readable Stream
 */
proto.orderbook.OrderbookAggregatorClient.prototype.bookSummary =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/orderbook.OrderbookAggregator/BookSummary',
      request,
      metadata || {},
      methodDescriptor_OrderbookAggregator_BookSummary);
};


/**
 * @param {!proto.orderbook.Empty} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.orderbook.Summary>}
 *     The XHR Node Readable Stream
 */
proto.orderbook.OrderbookAggregatorPromiseClient.prototype.bookSummary =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/orderbook.OrderbookAggregator/BookSummary',
      request,
      metadata || {},
      methodDescriptor_OrderbookAggregator_BookSummary);
};


module.exports = proto.orderbook;
