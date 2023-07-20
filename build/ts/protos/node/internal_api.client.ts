// @generated by protobuf-ts 2.9.0 with parameter optimize_code_size
// @generated from protobuf file "internal_api.proto" (package "protos", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { Internal } from "./internal_api.js";
import type { MetricsRequest } from "./internal_api.js";
import type { NotifyRequest } from "./internal_api.js";
import type { StandardResponse } from "./common.js";
import type { HeartbeatRequest } from "./internal_api.js";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { CommandResponse } from "./internal_api.js";
import type { RegisterRequest } from "./internal_api.js";
import type { ServerStreamingCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service protos.Internal
 */
export interface IInternalClient {
    /**
     * Initial method that an SDK should call to register itself with the server.
     * The server will use this stream to send commands to the SDK via the
     * `CommandResponse` message. Clients should continuously listen for
     * CommandResponse messages and re-establish registration if the stream gets
     * disconnected.
     *
     * @generated from protobuf rpc: Register(protos.RegisterRequest) returns (stream protos.CommandResponse);
     */
    register(input: RegisterRequest, options?: RpcOptions): ServerStreamingCall<RegisterRequest, CommandResponse>;
    /**
     * SDK is responsible for sending heartbeats to the server to let the server
     * know about active consumers and producers.
     *
     * @generated from protobuf rpc: Heartbeat(protos.HeartbeatRequest) returns (protos.StandardResponse);
     */
    heartbeat(input: HeartbeatRequest, options?: RpcOptions): UnaryCall<HeartbeatRequest, StandardResponse>;
    /**
     * Use this method when Notify condition has been triggered; the server will
     * decide on what to do about the notification.
     *
     * @generated from protobuf rpc: Notify(protos.NotifyRequest) returns (protos.StandardResponse);
     */
    notify(input: NotifyRequest, options?: RpcOptions): UnaryCall<NotifyRequest, StandardResponse>;
    /**
     * Send periodic metrics to the server
     *
     * @generated from protobuf rpc: Metrics(protos.MetricsRequest) returns (protos.StandardResponse);
     */
    metrics(input: MetricsRequest, options?: RpcOptions): UnaryCall<MetricsRequest, StandardResponse>;
}
/**
 * @generated from protobuf service protos.Internal
 */
export class InternalClient implements IInternalClient, ServiceInfo {
    typeName = Internal.typeName;
    methods = Internal.methods;
    options = Internal.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * Initial method that an SDK should call to register itself with the server.
     * The server will use this stream to send commands to the SDK via the
     * `CommandResponse` message. Clients should continuously listen for
     * CommandResponse messages and re-establish registration if the stream gets
     * disconnected.
     *
     * @generated from protobuf rpc: Register(protos.RegisterRequest) returns (stream protos.CommandResponse);
     */
    register(input: RegisterRequest, options?: RpcOptions): ServerStreamingCall<RegisterRequest, CommandResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<RegisterRequest, CommandResponse>("serverStreaming", this._transport, method, opt, input);
    }
    /**
     * SDK is responsible for sending heartbeats to the server to let the server
     * know about active consumers and producers.
     *
     * @generated from protobuf rpc: Heartbeat(protos.HeartbeatRequest) returns (protos.StandardResponse);
     */
    heartbeat(input: HeartbeatRequest, options?: RpcOptions): UnaryCall<HeartbeatRequest, StandardResponse> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<HeartbeatRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Use this method when Notify condition has been triggered; the server will
     * decide on what to do about the notification.
     *
     * @generated from protobuf rpc: Notify(protos.NotifyRequest) returns (protos.StandardResponse);
     */
    notify(input: NotifyRequest, options?: RpcOptions): UnaryCall<NotifyRequest, StandardResponse> {
        const method = this.methods[2], opt = this._transport.mergeOptions(options);
        return stackIntercept<NotifyRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Send periodic metrics to the server
     *
     * @generated from protobuf rpc: Metrics(protos.MetricsRequest) returns (protos.StandardResponse);
     */
    metrics(input: MetricsRequest, options?: RpcOptions): UnaryCall<MetricsRequest, StandardResponse> {
        const method = this.methods[3], opt = this._transport.mergeOptions(options);
        return stackIntercept<MetricsRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
}
