// @generated by protobuf-ts 2.9.0 with parameter optimize_code_size
// @generated from protobuf file "steps/sp_steps_httprequest.proto" (package "protos.steps", syntax proto3)
// tslint:disable
import { MessageType } from "@protobuf-ts/runtime";
/**
 * @generated from protobuf message protos.steps.HttpRequest
 */
export interface HttpRequest {
    /**
     * @generated from protobuf field: protos.steps.HttpRequestMethod method = 1;
     */
    method: HttpRequestMethod;
    /**
     * @generated from protobuf field: string url = 2;
     */
    url: string;
    /**
     * @generated from protobuf field: bytes body = 3;
     */
    body: Uint8Array;
    /**
     * @generated from protobuf field: map<string, string> headers = 4;
     */
    headers: {
        [key: string]: string;
    };
}
/**
 * @generated from protobuf message protos.steps.HttpResponse
 */
export interface HttpResponse {
    /**
     * @generated from protobuf field: int32 code = 1;
     */
    code: number;
    /**
     * @generated from protobuf field: bytes body = 2;
     */
    body: Uint8Array;
    /**
     * @generated from protobuf field: map<string, string> headers = 3;
     */
    headers: {
        [key: string]: string;
    };
}
/**
 * @generated from protobuf message protos.steps.HttpRequestStep
 */
export interface HttpRequestStep {
    /**
     * @generated from protobuf field: protos.steps.HttpRequest request = 1;
     */
    request?: HttpRequest;
}
/**
 * @generated from protobuf enum protos.steps.HttpRequestMethod
 */
export enum HttpRequestMethod {
    /**
     * @generated from protobuf enum value: HTTP_REQUEST_METHOD_UNSET = 0;
     */
    UNSET = 0,
    /**
     * @generated from protobuf enum value: HTTP_REQUEST_METHOD_GET = 1;
     */
    GET = 1,
    /**
     * @generated from protobuf enum value: HTTP_REQUEST_METHOD_POST = 2;
     */
    POST = 2,
    /**
     * @generated from protobuf enum value: HTTP_REQUEST_METHOD_PUT = 3;
     */
    PUT = 3,
    /**
     * @generated from protobuf enum value: HTTP_REQUEST_METHOD_DELETE = 4;
     */
    DELETE = 4
}
// @generated message type with reflection information, may provide speed optimized methods
class HttpRequest$Type extends MessageType<HttpRequest> {
    constructor() {
        super("protos.steps.HttpRequest", [
            { no: 1, name: "method", kind: "enum", T: () => ["protos.steps.HttpRequestMethod", HttpRequestMethod, "HTTP_REQUEST_METHOD_"] },
            { no: 2, name: "url", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "body", kind: "scalar", T: 12 /*ScalarType.BYTES*/ },
            { no: 4, name: "headers", kind: "map", K: 9 /*ScalarType.STRING*/, V: { kind: "scalar", T: 9 /*ScalarType.STRING*/ } }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message protos.steps.HttpRequest
 */
export const HttpRequest = new HttpRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class HttpResponse$Type extends MessageType<HttpResponse> {
    constructor() {
        super("protos.steps.HttpResponse", [
            { no: 1, name: "code", kind: "scalar", T: 5 /*ScalarType.INT32*/ },
            { no: 2, name: "body", kind: "scalar", T: 12 /*ScalarType.BYTES*/ },
            { no: 3, name: "headers", kind: "map", K: 9 /*ScalarType.STRING*/, V: { kind: "scalar", T: 9 /*ScalarType.STRING*/ } }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message protos.steps.HttpResponse
 */
export const HttpResponse = new HttpResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class HttpRequestStep$Type extends MessageType<HttpRequestStep> {
    constructor() {
        super("protos.steps.HttpRequestStep", [
            { no: 1, name: "request", kind: "message", T: () => HttpRequest }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message protos.steps.HttpRequestStep
 */
export const HttpRequestStep = new HttpRequestStep$Type();
