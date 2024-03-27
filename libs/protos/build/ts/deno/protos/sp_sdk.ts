// @generated by protobuf-ts 2.9.0 with parameter long_type_string
// @generated from protobuf file "sp_sdk.proto" (package "protos", syntax proto3)
// tslint:disable
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MESSAGE_TYPE } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
import { AbortCondition } from "./sp_pipeline.ts";
/**
 * This is a separate type because some SDKs may want to implement some interface
 * for this type (ie. Stringer interface for Go SDK).
 *
 * @generated from protobuf message protos.Payload
 */
export interface Payload {
    /**
     * @generated from protobuf field: bytes data = 1;
     */
    data: Uint8Array;
}
/**
 * Common return response used by all SDKs
 *
 * @generated from protobuf message protos.SDKResponse
 */
export interface SDKResponse {
    /**
     * Contains (potentially) modified input data
     *
     * @generated from protobuf field: protos.Payload data = 1;
     */
    data?: Payload;
    /**
     * Execution status of the last step
     *
     * @generated from protobuf field: protos.ExecStatus status = 2;
     */
    status: ExecStatus;
    /**
     * Optional message accompanying the exec status for the last step
     *
     * @generated from protobuf field: optional string status_message = 3;
     */
    statusMessage?: string;
    /**
     * An array of pipelines that the SDK executed and the status of each step
     *
     * @generated from protobuf field: repeated protos.PipelineStatus pipeline_status = 4;
     */
    pipelineStatus: PipelineStatus[];
    /**
     * Includes any metadata that the step(s) may want to pass back to the user.
     *
     * NOTE: Metadata is aggregated across all steps in the pipeline, so if two
     * steps both set a key "foo" to different values, the value of "foo" in the
     * response will be the value set by the last step in the pipeline.
     *
     * To learn more about "metadata", see SDK Spec V2 doc "Pipeline Step & Error
     * Behavior" section.
     *
     * @generated from protobuf field: map<string, string> metadata = 5;
     */
    metadata: {
        [key: string]: string;
    };
}
/**
 * @generated from protobuf message protos.PipelineStatus
 */
export interface PipelineStatus {
    /**
     * ID of the pipeline
     *
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * The name of the pipeline
     *
     * @generated from protobuf field: string name = 2;
     */
    name: string;
    /**
     * The status of each step in the pipeline
     *
     * @generated from protobuf field: repeated protos.StepStatus step_status = 3;
     */
    stepStatus: StepStatus[];
}
/**
 * @generated from protobuf message protos.StepStatus
 */
export interface StepStatus {
    /**
     * The name of the step
     *
     * @generated from protobuf field: string name = 1;
     */
    name: string;
    /**
     * Execution outcome status of the step
     *
     * @generated from protobuf field: protos.ExecStatus status = 2;
     */
    status: ExecStatus;
    /**
     * Optional message accompanying the exec status
     *
     * @generated from protobuf field: optional string status_message = 3;
     */
    statusMessage?: string;
    /**
     * Indicates if current or all future pipelines were aborted.
     *
     * IMPORTANT: The SDK running into an error does not automatically abort
     * current or all future pipelines - the user must define the abort conditions
     * for "on_error".
     *
     * @generated from protobuf field: protos.AbortCondition abort_condition = 4;
     */
    abortCondition: AbortCondition;
}
/**
 * @generated from protobuf enum protos.ExecStatus
 */
export enum ExecStatus {
    /**
     * Unset status. This should never be returned by the SDK. If it does, it is
     * probably a bug (and you should file an issue)
     *
     * @generated from protobuf enum value: EXEC_STATUS_UNSET = 0;
     */
    UNSET = 0,
    /**
     * Indicates that the step execution evaluated to "true"
     *
     * @generated from protobuf enum value: EXEC_STATUS_TRUE = 1;
     */
    TRUE = 1,
    /**
     * Indicates that the step execution evaluated to "false"
     *
     * @generated from protobuf enum value: EXEC_STATUS_FALSE = 2;
     */
    FALSE = 2,
    /**
     * Indicates that the SDK encountered an error while trying to process the
     * request. Example error cases: SDK can't find the appropriate Wasm module,
     * Wasm function cannot alloc or dealloc memory, etc.
     *
     * @generated from protobuf enum value: EXEC_STATUS_ERROR = 3;
     */
    ERROR = 3
}
// @generated message type with reflection information, may provide speed optimized methods
class Payload$Type extends MessageType<Payload> {
    constructor() {
        super("protos.Payload", [
            { no: 1, name: "data", kind: "scalar", T: 12 /*ScalarType.BYTES*/ }
        ]);
    }
    create(value?: PartialMessage<Payload>): Payload {
        const message = { data: new Uint8Array(0) };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<Payload>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: Payload): Payload {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* bytes data */ 1:
                    message.data = reader.bytes();
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: Payload, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* bytes data = 1; */
        if (message.data.length)
            writer.tag(1, WireType.LengthDelimited).bytes(message.data);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.Payload
 */
export const Payload = new Payload$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SDKResponse$Type extends MessageType<SDKResponse> {
    constructor() {
        super("protos.SDKResponse", [
            { no: 1, name: "data", kind: "message", T: () => Payload },
            { no: 2, name: "status", kind: "enum", T: () => ["protos.ExecStatus", ExecStatus, "EXEC_STATUS_"] },
            { no: 3, name: "status_message", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "pipeline_status", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => PipelineStatus },
            { no: 5, name: "metadata", kind: "map", K: 9 /*ScalarType.STRING*/, V: { kind: "scalar", T: 9 /*ScalarType.STRING*/ } }
        ]);
    }
    create(value?: PartialMessage<SDKResponse>): SDKResponse {
        const message = { status: 0, pipelineStatus: [], metadata: {} };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<SDKResponse>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: SDKResponse): SDKResponse {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* protos.Payload data */ 1:
                    message.data = Payload.internalBinaryRead(reader, reader.uint32(), options, message.data);
                    break;
                case /* protos.ExecStatus status */ 2:
                    message.status = reader.int32();
                    break;
                case /* optional string status_message */ 3:
                    message.statusMessage = reader.string();
                    break;
                case /* repeated protos.PipelineStatus pipeline_status */ 4:
                    message.pipelineStatus.push(PipelineStatus.internalBinaryRead(reader, reader.uint32(), options));
                    break;
                case /* map<string, string> metadata */ 5:
                    this.binaryReadMap5(message.metadata, reader, options);
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    private binaryReadMap5(map: SDKResponse["metadata"], reader: IBinaryReader, options: BinaryReadOptions): void {
        let len = reader.uint32(), end = reader.pos + len, key: keyof SDKResponse["metadata"] | undefined, val: SDKResponse["metadata"][any] | undefined;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case 1:
                    key = reader.string();
                    break;
                case 2:
                    val = reader.string();
                    break;
                default: throw new globalThis.Error("unknown map entry field for field protos.SDKResponse.metadata");
            }
        }
        map[key ?? ""] = val ?? "";
    }
    internalBinaryWrite(message: SDKResponse, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* protos.Payload data = 1; */
        if (message.data)
            Payload.internalBinaryWrite(message.data, writer.tag(1, WireType.LengthDelimited).fork(), options).join();
        /* protos.ExecStatus status = 2; */
        if (message.status !== 0)
            writer.tag(2, WireType.Varint).int32(message.status);
        /* optional string status_message = 3; */
        if (message.statusMessage !== undefined)
            writer.tag(3, WireType.LengthDelimited).string(message.statusMessage);
        /* repeated protos.PipelineStatus pipeline_status = 4; */
        for (let i = 0; i < message.pipelineStatus.length; i++)
            PipelineStatus.internalBinaryWrite(message.pipelineStatus[i], writer.tag(4, WireType.LengthDelimited).fork(), options).join();
        /* map<string, string> metadata = 5; */
        for (let k of Object.keys(message.metadata))
            writer.tag(5, WireType.LengthDelimited).fork().tag(1, WireType.LengthDelimited).string(k).tag(2, WireType.LengthDelimited).string(message.metadata[k]).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.SDKResponse
 */
export const SDKResponse = new SDKResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class PipelineStatus$Type extends MessageType<PipelineStatus> {
    constructor() {
        super("protos.PipelineStatus", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "step_status", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => StepStatus }
        ]);
    }
    create(value?: PartialMessage<PipelineStatus>): PipelineStatus {
        const message = { id: "", name: "", stepStatus: [] };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<PipelineStatus>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: PipelineStatus): PipelineStatus {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string id */ 1:
                    message.id = reader.string();
                    break;
                case /* string name */ 2:
                    message.name = reader.string();
                    break;
                case /* repeated protos.StepStatus step_status */ 3:
                    message.stepStatus.push(StepStatus.internalBinaryRead(reader, reader.uint32(), options));
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: PipelineStatus, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* string id = 1; */
        if (message.id !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.id);
        /* string name = 2; */
        if (message.name !== "")
            writer.tag(2, WireType.LengthDelimited).string(message.name);
        /* repeated protos.StepStatus step_status = 3; */
        for (let i = 0; i < message.stepStatus.length; i++)
            StepStatus.internalBinaryWrite(message.stepStatus[i], writer.tag(3, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.PipelineStatus
 */
export const PipelineStatus = new PipelineStatus$Type();
// @generated message type with reflection information, may provide speed optimized methods
class StepStatus$Type extends MessageType<StepStatus> {
    constructor() {
        super("protos.StepStatus", [
            { no: 1, name: "name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "status", kind: "enum", T: () => ["protos.ExecStatus", ExecStatus, "EXEC_STATUS_"] },
            { no: 3, name: "status_message", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "abort_condition", kind: "enum", T: () => ["protos.AbortCondition", AbortCondition, "ABORT_CONDITION_"] }
        ]);
    }
    create(value?: PartialMessage<StepStatus>): StepStatus {
        const message = { name: "", status: 0, abortCondition: 0 };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<StepStatus>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: StepStatus): StepStatus {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string name */ 1:
                    message.name = reader.string();
                    break;
                case /* protos.ExecStatus status */ 2:
                    message.status = reader.int32();
                    break;
                case /* optional string status_message */ 3:
                    message.statusMessage = reader.string();
                    break;
                case /* protos.AbortCondition abort_condition */ 4:
                    message.abortCondition = reader.int32();
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: StepStatus, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* string name = 1; */
        if (message.name !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.name);
        /* protos.ExecStatus status = 2; */
        if (message.status !== 0)
            writer.tag(2, WireType.Varint).int32(message.status);
        /* optional string status_message = 3; */
        if (message.statusMessage !== undefined)
            writer.tag(3, WireType.LengthDelimited).string(message.statusMessage);
        /* protos.AbortCondition abort_condition = 4; */
        if (message.abortCondition !== 0)
            writer.tag(4, WireType.Varint).int32(message.abortCondition);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.StepStatus
 */
export const StepStatus = new StepStatus$Type();
