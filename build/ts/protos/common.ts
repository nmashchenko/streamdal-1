// @generated by protobuf-ts 2.9.0 with parameter optimize_code_size
// @generated from protobuf file "common.proto" (package "protos", syntax proto3)
// tslint:disable
import { MessageType } from "@protobuf-ts/runtime";
/**
 * Common response message for many gRPC methods
 *
 * @generated from protobuf message protos.StandardResponse
 */
export interface StandardResponse {
    /**
     * Co-relation ID for the request / response
     *
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * @generated from protobuf field: protos.ResponseCode code = 2;
     */
    code: ResponseCode;
    /**
     * @generated from protobuf field: string message = 3;
     */
    message: string;
    /**
     * Debug info that server may populate with additional info.
     *
     * @generated from protobuf field: map<string, string> _metadata = 1000;
     */
    Metadata: {
        [key: string]: string;
    };
}
/**
 * Common status codes used in gRPC method responses
 *
 * @generated from protobuf enum protos.ResponseCode
 */
export enum ResponseCode {
    /**
     * @generated from protobuf enum value: RESPONSE_CODE_UNSET = 0;
     */
    UNSET = 0,
    /**
     * @generated from protobuf enum value: RESPONSE_CODE_OK = 1;
     */
    OK = 1,
    /**
     * @generated from protobuf enum value: RESPONSE_CODE_BAD_REQUEST = 2;
     */
    BAD_REQUEST = 2,
    /**
     * @generated from protobuf enum value: RESPONSE_CODE_NOT_FOUND = 3;
     */
    NOT_FOUND = 3,
    /**
     * @generated from protobuf enum value: RESPONSE_CODE_INTERNAL_SERVER_ERROR = 4;
     */
    INTERNAL_SERVER_ERROR = 4,
    /**
     * @generated from protobuf enum value: RESPONSE_CODE_GENERIC_ERROR = 5;
     */
    GENERIC_ERROR = 5
}
// @generated message type with reflection information, may provide speed optimized methods
class StandardResponse$Type extends MessageType<StandardResponse> {
    constructor() {
        super("protos.StandardResponse", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "code", kind: "enum", T: () => ["protos.ResponseCode", ResponseCode, "RESPONSE_CODE_"] },
            { no: 3, name: "message", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 1000, name: "_metadata", kind: "map", K: 9 /*ScalarType.STRING*/, V: { kind: "scalar", T: 9 /*ScalarType.STRING*/ } }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message protos.StandardResponse
 */
export const StandardResponse = new StandardResponse$Type();
