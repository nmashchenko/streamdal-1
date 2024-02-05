// @generated by protobuf-ts 2.9.0 with parameter long_type_string
// @generated from protobuf file "sp_pipeline.proto" (package "protos", syntax proto3)
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
import { SchemaValidationStep } from "./steps/sp_steps_schema_validation.ts";
import { ValidJSONStep } from "./steps/sp_steps_valid_json.ts";
import { InferSchemaStep } from "./steps/sp_steps_inferschema.ts";
import { KVStep } from "./steps/sp_steps_kv.ts";
import { HttpRequestStep } from "./steps/sp_steps_httprequest.ts";
import { CustomStep } from "./steps/sp_steps_custom.ts";
import { DecodeStep } from "./steps/sp_steps_decode.ts";
import { EncodeStep } from "./steps/sp_steps_encode.ts";
import { TransformStep } from "./steps/sp_steps_transform.ts";
import { DetectiveStep } from "./steps/sp_steps_detective.ts";
import { NotificationConfig } from "./sp_notify.ts";
/**
 * Pipeline is a structure that holds one or more pipeline steps. This structure
 * is intended to be immutable; clients are expected to generate WASMRequest's
 * that contain a pipeline step.
 *
 * @generated from protobuf message protos.Pipeline
 */
export interface Pipeline {
    /**
     * ID should NOT be set by external gRPC client on CreatePipelineRequest - it
     * will be ignored; it _does_ need to be set on UpdatePipelineRequest.
     *
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * Friendly name for the pipeline
     *
     * @generated from protobuf field: string name = 2;
     */
    name: string;
    /**
     * One or more steps to execute
     *
     * @generated from protobuf field: repeated protos.PipelineStep steps = 3;
     */
    steps: PipelineStep[];
    /**
     * Notification configs for this pipeline. Only filled out in external API responses
     * This is deprecated and the data has moved to PipelineStep
     *
     * @deprecated
     * @generated from protobuf field: repeated protos.NotificationConfig _notification_configs = 4 [deprecated = true];
     */
    NotificationConfigs: NotificationConfig[]; // protolint:disable:this FIELD_NAMES_LOWER_SNAKE_CASE
    /**
     * Indicates whether the pipeline is paused or not. Used internally by server.
     *
     * @generated from protobuf field: optional bool _paused = 1000;
     */
    Paused?: boolean; // protolint:disable:this FIELD_NAMES_LOWER_SNAKE_CASE
}
/**
 * Conditions define how the SDK should handle a Wasm response in a step.
 * Should it continue executing the pipeline, should it abort, should it notify
 * and on_error.
 * TODO: de-pluralize this name
 *
 * @generated from protobuf message protos.PipelineStepConditions
 */
export interface PipelineStepConditions {
    /**
     * Should we abort execution?
     *
     * @generated from protobuf field: protos.AbortCondition abort = 1;
     */
    abort: AbortCondition;
    /**
     * @deprecated
     * @generated from protobuf field: bool notify = 2 [deprecated = true];
     */
    notify: boolean;
    /**
     * Should we include additional metadata that SDK should pass back to user?
     *
     * @generated from protobuf field: map<string, string> metadata = 3;
     */
    metadata: {
        [key: string]: string;
    };
    /**
     * @generated from protobuf field: protos.PipelineStepNotification notification = 4;
     */
    notification?: PipelineStepNotification;
}
/**
 * @generated from protobuf message protos.PipelineStepNotification
 */
export interface PipelineStepNotification {
    /**
     * The UUIDs of the notification config to use
     * This is kept separate to avoid having to configure slack/email settings
     * every time and also because that config info is sensitive and is encrypted
     *
     * @generated from protobuf field: repeated string notification_config_ids = 1;
     */
    notificationConfigIds: string[];
    /**
     * @generated from protobuf field: protos.PipelineStepNotification.PayloadType payload_type = 2;
     */
    payloadType: PipelineStepNotification_PayloadType;
    /**
     * If type == paths, then we will look here for a list of json paths to include
     * in the notification payload.
     *
     * @generated from protobuf field: repeated string paths = 3;
     */
    paths: string[];
}
/**
 * @generated from protobuf enum protos.PipelineStepNotification.PayloadType
 */
export enum PipelineStepNotification_PayloadType {
    /**
     * Same functionality as PAYLOAD_TYPE_EXCLUDE
     *
     * @generated from protobuf enum value: PAYLOAD_TYPE_UNSET = 0;
     */
    UNSET = 0,
    /**
     * Default. No payload data included in notification
     *
     * @generated from protobuf enum value: PAYLOAD_TYPE_EXCLUDE = 1;
     */
    EXCLUDE = 1,
    /**
     * Entire payload content included in notification
     *
     * @generated from protobuf enum value: PAYLOAD_TYPE_FULL_PAYLOAD = 2;
     */
    FULL_PAYLOAD = 2,
    /**
     * Only specified paths of payload content included in notification
     * Only works on JSON. Plaintext payloads will be ignored.
     *
     * @generated from protobuf enum value: PAYLOAD_TYPE_SELECT_PATHS = 3;
     */
    SELECT_PATHS = 3
}
/**
 * A pipeline step is a single step in a pipeline.
 *
 * @generated from protobuf message protos.PipelineStep
 */
export interface PipelineStep {
    /**
     * Friendly name for the step
     *
     * @generated from protobuf field: string name = 1;
     */
    name: string;
    /**
     * SDKs should read this when Wasm returns 'true' to determine what to do next.
     *
     * @generated from protobuf field: protos.PipelineStepConditions on_true = 2;
     */
    onTrue?: PipelineStepConditions;
    /**
     * SDKs should read this when Wasm returns 'false' to determine what to do next.
     *
     * @generated from protobuf field: protos.PipelineStepConditions on_false = 3;
     */
    onFalse?: PipelineStepConditions;
    /**
     * Indicates whether to use the results from a previous step as input to this step
     *
     * @generated from protobuf field: bool dynamic = 4;
     */
    dynamic: boolean;
    /**
     * SDKs should read this when Wasm returns 'error' to determine what to do next.
     *
     * @generated from protobuf field: protos.PipelineStepConditions on_error = 5;
     */
    onError?: PipelineStepConditions;
    /**
     * @generated from protobuf oneof: step
     */
    step: {
        oneofKind: "detective";
        /**
         * @generated from protobuf field: protos.steps.DetectiveStep detective = 1000;
         */
        detective: DetectiveStep;
    } | {
        oneofKind: "transform";
        /**
         * @generated from protobuf field: protos.steps.TransformStep transform = 1001;
         */
        transform: TransformStep;
    } | {
        oneofKind: "encode";
        /**
         * @generated from protobuf field: protos.steps.EncodeStep encode = 1002;
         */
        encode: EncodeStep;
    } | {
        oneofKind: "decode";
        /**
         * @generated from protobuf field: protos.steps.DecodeStep decode = 1003;
         */
        decode: DecodeStep;
    } | {
        oneofKind: "custom";
        /**
         * @generated from protobuf field: protos.steps.CustomStep custom = 1004;
         */
        custom: CustomStep;
    } | {
        oneofKind: "httpRequest";
        /**
         * @generated from protobuf field: protos.steps.HttpRequestStep http_request = 1005;
         */
        httpRequest: HttpRequestStep;
    } | {
        oneofKind: "kv";
        /**
         * @generated from protobuf field: protos.steps.KVStep kv = 1006;
         */
        kv: KVStep;
    } | {
        oneofKind: "inferSchema";
        /**
         * @generated from protobuf field: protos.steps.InferSchemaStep infer_schema = 1007;
         */
        inferSchema: InferSchemaStep;
    } | {
        oneofKind: "validJson";
        /**
         * @generated from protobuf field: protos.steps.ValidJSONStep valid_json = 1008;
         */
        validJson: ValidJSONStep;
    } | {
        oneofKind: "schemaValidation";
        /**
         * @generated from protobuf field: protos.steps.SchemaValidationStep schema_validation = 1009;
         */
        schemaValidation: SchemaValidationStep;
    } | {
        oneofKind: undefined;
    };
    /**
     * ID is a uuid(sha256(_wasm_bytes)) that is set by server
     *
     * @generated from protobuf field: optional string _wasm_id = 10000;
     */
    WasmId?: string; // protolint:disable:this FIELD_NAMES_LOWER_SNAKE_CASE
    /**
     * WASM module bytes (set by server)
     *
     * @generated from protobuf field: optional bytes _wasm_bytes = 10001;
     */
    WasmBytes?: Uint8Array; // protolint:disable:this FIELD_NAMES_LOWER_SNAKE_CASE
    /**
     * WASM function name to execute (set by server)
     *
     * @generated from protobuf field: optional string _wasm_function = 10002;
     */
    WasmFunction?: string; // protolint:disable:this FIELD_NAMES_LOWER_SNAKE_CASE
}
/**
 * Defines the ways in which a pipeline can be aborted
 *
 * @generated from protobuf enum protos.AbortCondition
 */
export enum AbortCondition {
    /**
     * @generated from protobuf enum value: ABORT_CONDITION_UNSET = 0;
     */
    UNSET = 0,
    /**
     * @generated from protobuf enum value: ABORT_CONDITION_ABORT_CURRENT = 1;
     */
    ABORT_CURRENT = 1,
    /**
     * @generated from protobuf enum value: ABORT_CONDITION_ABORT_ALL = 2;
     */
    ABORT_ALL = 2
}
// @generated message type with reflection information, may provide speed optimized methods
class Pipeline$Type extends MessageType<Pipeline> {
    constructor() {
        super("protos.Pipeline", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "steps", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => PipelineStep },
            { no: 4, name: "_notification_configs", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => NotificationConfig },
            { no: 1000, name: "_paused", kind: "scalar", opt: true, T: 8 /*ScalarType.BOOL*/ }
        ]);
    }
    create(value?: PartialMessage<Pipeline>): Pipeline {
        const message = { id: "", name: "", steps: [], NotificationConfigs: [] };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<Pipeline>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: Pipeline): Pipeline {
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
                case /* repeated protos.PipelineStep steps */ 3:
                    message.steps.push(PipelineStep.internalBinaryRead(reader, reader.uint32(), options));
                    break;
                case /* repeated protos.NotificationConfig _notification_configs = 4 [deprecated = true];*/ 4:
                    message.NotificationConfigs.push(NotificationConfig.internalBinaryRead(reader, reader.uint32(), options));
                    break;
                case /* optional bool _paused */ 1000:
                    message.Paused = reader.bool();
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
    internalBinaryWrite(message: Pipeline, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* string id = 1; */
        if (message.id !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.id);
        /* string name = 2; */
        if (message.name !== "")
            writer.tag(2, WireType.LengthDelimited).string(message.name);
        /* repeated protos.PipelineStep steps = 3; */
        for (let i = 0; i < message.steps.length; i++)
            PipelineStep.internalBinaryWrite(message.steps[i], writer.tag(3, WireType.LengthDelimited).fork(), options).join();
        /* repeated protos.NotificationConfig _notification_configs = 4 [deprecated = true]; */
        for (let i = 0; i < message.NotificationConfigs.length; i++)
            NotificationConfig.internalBinaryWrite(message.NotificationConfigs[i], writer.tag(4, WireType.LengthDelimited).fork(), options).join();
        /* optional bool _paused = 1000; */
        if (message.Paused !== undefined)
            writer.tag(1000, WireType.Varint).bool(message.Paused);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.Pipeline
 */
export const Pipeline = new Pipeline$Type();
// @generated message type with reflection information, may provide speed optimized methods
class PipelineStepConditions$Type extends MessageType<PipelineStepConditions> {
    constructor() {
        super("protos.PipelineStepConditions", [
            { no: 1, name: "abort", kind: "enum", T: () => ["protos.AbortCondition", AbortCondition, "ABORT_CONDITION_"] },
            { no: 2, name: "notify", kind: "scalar", T: 8 /*ScalarType.BOOL*/ },
            { no: 3, name: "metadata", kind: "map", K: 9 /*ScalarType.STRING*/, V: { kind: "scalar", T: 9 /*ScalarType.STRING*/ } },
            { no: 4, name: "notification", kind: "message", T: () => PipelineStepNotification }
        ]);
    }
    create(value?: PartialMessage<PipelineStepConditions>): PipelineStepConditions {
        const message = { abort: 0, notify: false, metadata: {} };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<PipelineStepConditions>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: PipelineStepConditions): PipelineStepConditions {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* protos.AbortCondition abort */ 1:
                    message.abort = reader.int32();
                    break;
                case /* bool notify = 2 [deprecated = true];*/ 2:
                    message.notify = reader.bool();
                    break;
                case /* map<string, string> metadata */ 3:
                    this.binaryReadMap3(message.metadata, reader, options);
                    break;
                case /* protos.PipelineStepNotification notification */ 4:
                    message.notification = PipelineStepNotification.internalBinaryRead(reader, reader.uint32(), options, message.notification);
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
    private binaryReadMap3(map: PipelineStepConditions["metadata"], reader: IBinaryReader, options: BinaryReadOptions): void {
        let len = reader.uint32(), end = reader.pos + len, key: keyof PipelineStepConditions["metadata"] | undefined, val: PipelineStepConditions["metadata"][any] | undefined;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case 1:
                    key = reader.string();
                    break;
                case 2:
                    val = reader.string();
                    break;
                default: throw new globalThis.Error("unknown map entry field for field protos.PipelineStepConditions.metadata");
            }
        }
        map[key ?? ""] = val ?? "";
    }
    internalBinaryWrite(message: PipelineStepConditions, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* protos.AbortCondition abort = 1; */
        if (message.abort !== 0)
            writer.tag(1, WireType.Varint).int32(message.abort);
        /* bool notify = 2 [deprecated = true]; */
        if (message.notify !== false)
            writer.tag(2, WireType.Varint).bool(message.notify);
        /* map<string, string> metadata = 3; */
        for (let k of Object.keys(message.metadata))
            writer.tag(3, WireType.LengthDelimited).fork().tag(1, WireType.LengthDelimited).string(k).tag(2, WireType.LengthDelimited).string(message.metadata[k]).join();
        /* protos.PipelineStepNotification notification = 4; */
        if (message.notification)
            PipelineStepNotification.internalBinaryWrite(message.notification, writer.tag(4, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.PipelineStepConditions
 */
export const PipelineStepConditions = new PipelineStepConditions$Type();
// @generated message type with reflection information, may provide speed optimized methods
class PipelineStepNotification$Type extends MessageType<PipelineStepNotification> {
    constructor() {
        super("protos.PipelineStepNotification", [
            { no: 1, name: "notification_config_ids", kind: "scalar", repeat: 2 /*RepeatType.UNPACKED*/, T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "payload_type", kind: "enum", T: () => ["protos.PipelineStepNotification.PayloadType", PipelineStepNotification_PayloadType, "PAYLOAD_TYPE_"] },
            { no: 3, name: "paths", kind: "scalar", repeat: 2 /*RepeatType.UNPACKED*/, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value?: PartialMessage<PipelineStepNotification>): PipelineStepNotification {
        const message = { notificationConfigIds: [], payloadType: 0, paths: [] };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<PipelineStepNotification>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: PipelineStepNotification): PipelineStepNotification {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* repeated string notification_config_ids */ 1:
                    message.notificationConfigIds.push(reader.string());
                    break;
                case /* protos.PipelineStepNotification.PayloadType payload_type */ 2:
                    message.payloadType = reader.int32();
                    break;
                case /* repeated string paths */ 3:
                    message.paths.push(reader.string());
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
    internalBinaryWrite(message: PipelineStepNotification, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* repeated string notification_config_ids = 1; */
        for (let i = 0; i < message.notificationConfigIds.length; i++)
            writer.tag(1, WireType.LengthDelimited).string(message.notificationConfigIds[i]);
        /* protos.PipelineStepNotification.PayloadType payload_type = 2; */
        if (message.payloadType !== 0)
            writer.tag(2, WireType.Varint).int32(message.payloadType);
        /* repeated string paths = 3; */
        for (let i = 0; i < message.paths.length; i++)
            writer.tag(3, WireType.LengthDelimited).string(message.paths[i]);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.PipelineStepNotification
 */
export const PipelineStepNotification = new PipelineStepNotification$Type();
// @generated message type with reflection information, may provide speed optimized methods
class PipelineStep$Type extends MessageType<PipelineStep> {
    constructor() {
        super("protos.PipelineStep", [
            { no: 1, name: "name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "on_true", kind: "message", T: () => PipelineStepConditions },
            { no: 3, name: "on_false", kind: "message", T: () => PipelineStepConditions },
            { no: 4, name: "dynamic", kind: "scalar", T: 8 /*ScalarType.BOOL*/ },
            { no: 5, name: "on_error", kind: "message", T: () => PipelineStepConditions },
            { no: 1000, name: "detective", kind: "message", oneof: "step", T: () => DetectiveStep },
            { no: 1001, name: "transform", kind: "message", oneof: "step", T: () => TransformStep },
            { no: 1002, name: "encode", kind: "message", oneof: "step", T: () => EncodeStep },
            { no: 1003, name: "decode", kind: "message", oneof: "step", T: () => DecodeStep },
            { no: 1004, name: "custom", kind: "message", oneof: "step", T: () => CustomStep },
            { no: 1005, name: "http_request", kind: "message", oneof: "step", T: () => HttpRequestStep },
            { no: 1006, name: "kv", kind: "message", oneof: "step", T: () => KVStep },
            { no: 1007, name: "infer_schema", kind: "message", oneof: "step", T: () => InferSchemaStep },
            { no: 1008, name: "valid_json", kind: "message", oneof: "step", T: () => ValidJSONStep },
            { no: 1009, name: "schema_validation", kind: "message", oneof: "step", T: () => SchemaValidationStep },
            { no: 10000, name: "_wasm_id", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 10001, name: "_wasm_bytes", kind: "scalar", opt: true, T: 12 /*ScalarType.BYTES*/ },
            { no: 10002, name: "_wasm_function", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value?: PartialMessage<PipelineStep>): PipelineStep {
        const message = { name: "", dynamic: false, step: { oneofKind: undefined } };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<PipelineStep>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: PipelineStep): PipelineStep {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string name */ 1:
                    message.name = reader.string();
                    break;
                case /* protos.PipelineStepConditions on_true */ 2:
                    message.onTrue = PipelineStepConditions.internalBinaryRead(reader, reader.uint32(), options, message.onTrue);
                    break;
                case /* protos.PipelineStepConditions on_false */ 3:
                    message.onFalse = PipelineStepConditions.internalBinaryRead(reader, reader.uint32(), options, message.onFalse);
                    break;
                case /* bool dynamic */ 4:
                    message.dynamic = reader.bool();
                    break;
                case /* protos.PipelineStepConditions on_error */ 5:
                    message.onError = PipelineStepConditions.internalBinaryRead(reader, reader.uint32(), options, message.onError);
                    break;
                case /* protos.steps.DetectiveStep detective */ 1000:
                    message.step = {
                        oneofKind: "detective",
                        detective: DetectiveStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).detective)
                    };
                    break;
                case /* protos.steps.TransformStep transform */ 1001:
                    message.step = {
                        oneofKind: "transform",
                        transform: TransformStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).transform)
                    };
                    break;
                case /* protos.steps.EncodeStep encode */ 1002:
                    message.step = {
                        oneofKind: "encode",
                        encode: EncodeStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).encode)
                    };
                    break;
                case /* protos.steps.DecodeStep decode */ 1003:
                    message.step = {
                        oneofKind: "decode",
                        decode: DecodeStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).decode)
                    };
                    break;
                case /* protos.steps.CustomStep custom */ 1004:
                    message.step = {
                        oneofKind: "custom",
                        custom: CustomStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).custom)
                    };
                    break;
                case /* protos.steps.HttpRequestStep http_request */ 1005:
                    message.step = {
                        oneofKind: "httpRequest",
                        httpRequest: HttpRequestStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).httpRequest)
                    };
                    break;
                case /* protos.steps.KVStep kv */ 1006:
                    message.step = {
                        oneofKind: "kv",
                        kv: KVStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).kv)
                    };
                    break;
                case /* protos.steps.InferSchemaStep infer_schema */ 1007:
                    message.step = {
                        oneofKind: "inferSchema",
                        inferSchema: InferSchemaStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).inferSchema)
                    };
                    break;
                case /* protos.steps.ValidJSONStep valid_json */ 1008:
                    message.step = {
                        oneofKind: "validJson",
                        validJson: ValidJSONStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).validJson)
                    };
                    break;
                case /* protos.steps.SchemaValidationStep schema_validation */ 1009:
                    message.step = {
                        oneofKind: "schemaValidation",
                        schemaValidation: SchemaValidationStep.internalBinaryRead(reader, reader.uint32(), options, (message.step as any).schemaValidation)
                    };
                    break;
                case /* optional string _wasm_id */ 10000:
                    message.WasmId = reader.string();
                    break;
                case /* optional bytes _wasm_bytes */ 10001:
                    message.WasmBytes = reader.bytes();
                    break;
                case /* optional string _wasm_function */ 10002:
                    message.WasmFunction = reader.string();
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
    internalBinaryWrite(message: PipelineStep, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* string name = 1; */
        if (message.name !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.name);
        /* protos.PipelineStepConditions on_true = 2; */
        if (message.onTrue)
            PipelineStepConditions.internalBinaryWrite(message.onTrue, writer.tag(2, WireType.LengthDelimited).fork(), options).join();
        /* protos.PipelineStepConditions on_false = 3; */
        if (message.onFalse)
            PipelineStepConditions.internalBinaryWrite(message.onFalse, writer.tag(3, WireType.LengthDelimited).fork(), options).join();
        /* bool dynamic = 4; */
        if (message.dynamic !== false)
            writer.tag(4, WireType.Varint).bool(message.dynamic);
        /* protos.PipelineStepConditions on_error = 5; */
        if (message.onError)
            PipelineStepConditions.internalBinaryWrite(message.onError, writer.tag(5, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.DetectiveStep detective = 1000; */
        if (message.step.oneofKind === "detective")
            DetectiveStep.internalBinaryWrite(message.step.detective, writer.tag(1000, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.TransformStep transform = 1001; */
        if (message.step.oneofKind === "transform")
            TransformStep.internalBinaryWrite(message.step.transform, writer.tag(1001, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.EncodeStep encode = 1002; */
        if (message.step.oneofKind === "encode")
            EncodeStep.internalBinaryWrite(message.step.encode, writer.tag(1002, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.DecodeStep decode = 1003; */
        if (message.step.oneofKind === "decode")
            DecodeStep.internalBinaryWrite(message.step.decode, writer.tag(1003, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.CustomStep custom = 1004; */
        if (message.step.oneofKind === "custom")
            CustomStep.internalBinaryWrite(message.step.custom, writer.tag(1004, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.HttpRequestStep http_request = 1005; */
        if (message.step.oneofKind === "httpRequest")
            HttpRequestStep.internalBinaryWrite(message.step.httpRequest, writer.tag(1005, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.KVStep kv = 1006; */
        if (message.step.oneofKind === "kv")
            KVStep.internalBinaryWrite(message.step.kv, writer.tag(1006, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.InferSchemaStep infer_schema = 1007; */
        if (message.step.oneofKind === "inferSchema")
            InferSchemaStep.internalBinaryWrite(message.step.inferSchema, writer.tag(1007, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.ValidJSONStep valid_json = 1008; */
        if (message.step.oneofKind === "validJson")
            ValidJSONStep.internalBinaryWrite(message.step.validJson, writer.tag(1008, WireType.LengthDelimited).fork(), options).join();
        /* protos.steps.SchemaValidationStep schema_validation = 1009; */
        if (message.step.oneofKind === "schemaValidation")
            SchemaValidationStep.internalBinaryWrite(message.step.schemaValidation, writer.tag(1009, WireType.LengthDelimited).fork(), options).join();
        /* optional string _wasm_id = 10000; */
        if (message.WasmId !== undefined)
            writer.tag(10000, WireType.LengthDelimited).string(message.WasmId);
        /* optional bytes _wasm_bytes = 10001; */
        if (message.WasmBytes !== undefined)
            writer.tag(10001, WireType.LengthDelimited).bytes(message.WasmBytes);
        /* optional string _wasm_function = 10002; */
        if (message.WasmFunction !== undefined)
            writer.tag(10002, WireType.LengthDelimited).string(message.WasmFunction);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.PipelineStep
 */
export const PipelineStep = new PipelineStep$Type();
