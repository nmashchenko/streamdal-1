// @generated by protobuf-ts 2.9.0 with parameter long_type_string
// @generated from protobuf file "sp_internal.proto" (package "protos", syntax proto3)
// tslint:disable
import { TailResponse } from "./sp_common.js";
import { StandardResponse } from "./sp_common.js";
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import { WireType } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MESSAGE_TYPE } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
import { Schema } from "./sp_common.js";
import { Command } from "./sp_command.js";
import { Metric } from "./sp_common.js";
import { PipelineStepNotification } from "./sp_pipeline.js";
import { PipelineStep } from "./sp_pipeline.js";
import { ClientInfo } from "./sp_info.js";
import { Audience } from "./sp_common.js";
// @generated message type with reflection information, may provide speed optimized methods
class NewAudienceRequest$Type extends MessageType {
    constructor() {
        super("protos.NewAudienceRequest", [
            { no: 1, name: "session_id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "audience", kind: "message", T: () => Audience }
        ]);
    }
    create(value) {
        const message = { sessionId: "" };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string session_id */ 1:
                    message.sessionId = reader.string();
                    break;
                case /* protos.Audience audience */ 2:
                    message.audience = Audience.internalBinaryRead(reader, reader.uint32(), options, message.audience);
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
    internalBinaryWrite(message, writer, options) {
        /* string session_id = 1; */
        if (message.sessionId !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.sessionId);
        /* protos.Audience audience = 2; */
        if (message.audience)
            Audience.internalBinaryWrite(message.audience, writer.tag(2, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.NewAudienceRequest
 */
export const NewAudienceRequest = new NewAudienceRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class HeartbeatRequest$Type extends MessageType {
    constructor() {
        super("protos.HeartbeatRequest", [
            { no: 1, name: "session_id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "service_name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "audiences", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => Audience },
            { no: 4, name: "client_info", kind: "message", T: () => ClientInfo }
        ]);
    }
    create(value) {
        const message = { sessionId: "", serviceName: "", audiences: [] };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string session_id */ 1:
                    message.sessionId = reader.string();
                    break;
                case /* string service_name */ 2:
                    message.serviceName = reader.string();
                    break;
                case /* repeated protos.Audience audiences */ 3:
                    message.audiences.push(Audience.internalBinaryRead(reader, reader.uint32(), options));
                    break;
                case /* protos.ClientInfo client_info */ 4:
                    message.clientInfo = ClientInfo.internalBinaryRead(reader, reader.uint32(), options, message.clientInfo);
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
    internalBinaryWrite(message, writer, options) {
        /* string session_id = 1; */
        if (message.sessionId !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.sessionId);
        /* string service_name = 2; */
        if (message.serviceName !== "")
            writer.tag(2, WireType.LengthDelimited).string(message.serviceName);
        /* repeated protos.Audience audiences = 3; */
        for (let i = 0; i < message.audiences.length; i++)
            Audience.internalBinaryWrite(message.audiences[i], writer.tag(3, WireType.LengthDelimited).fork(), options).join();
        /* protos.ClientInfo client_info = 4; */
        if (message.clientInfo)
            ClientInfo.internalBinaryWrite(message.clientInfo, writer.tag(4, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.HeartbeatRequest
 */
export const HeartbeatRequest = new HeartbeatRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class NotifyRequest$Type extends MessageType {
    constructor() {
        super("protos.NotifyRequest", [
            { no: 1, name: "pipeline_id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "step_name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "audience", kind: "message", T: () => Audience },
            { no: 4, name: "occurred_at_unix_ts_utc", kind: "scalar", T: 3 /*ScalarType.INT64*/ },
            { no: 5, name: "payload", kind: "scalar", T: 12 /*ScalarType.BYTES*/ },
            { no: 6, name: "step", kind: "message", T: () => PipelineStep },
            { no: 7, name: "notification", kind: "message", T: () => PipelineStepNotification }
        ]);
    }
    create(value) {
        const message = { pipelineId: "", stepName: "", occurredAtUnixTsUtc: "0", payload: new Uint8Array(0) };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string pipeline_id */ 1:
                    message.pipelineId = reader.string();
                    break;
                case /* string step_name = 2 [deprecated = true];*/ 2:
                    message.stepName = reader.string();
                    break;
                case /* protos.Audience audience */ 3:
                    message.audience = Audience.internalBinaryRead(reader, reader.uint32(), options, message.audience);
                    break;
                case /* int64 occurred_at_unix_ts_utc */ 4:
                    message.occurredAtUnixTsUtc = reader.int64().toString();
                    break;
                case /* bytes payload */ 5:
                    message.payload = reader.bytes();
                    break;
                case /* protos.PipelineStep step */ 6:
                    message.step = PipelineStep.internalBinaryRead(reader, reader.uint32(), options, message.step);
                    break;
                case /* protos.PipelineStepNotification notification */ 7:
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
    internalBinaryWrite(message, writer, options) {
        /* string pipeline_id = 1; */
        if (message.pipelineId !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.pipelineId);
        /* string step_name = 2 [deprecated = true]; */
        if (message.stepName !== "")
            writer.tag(2, WireType.LengthDelimited).string(message.stepName);
        /* protos.Audience audience = 3; */
        if (message.audience)
            Audience.internalBinaryWrite(message.audience, writer.tag(3, WireType.LengthDelimited).fork(), options).join();
        /* int64 occurred_at_unix_ts_utc = 4; */
        if (message.occurredAtUnixTsUtc !== "0")
            writer.tag(4, WireType.Varint).int64(message.occurredAtUnixTsUtc);
        /* bytes payload = 5; */
        if (message.payload.length)
            writer.tag(5, WireType.LengthDelimited).bytes(message.payload);
        /* protos.PipelineStep step = 6; */
        if (message.step)
            PipelineStep.internalBinaryWrite(message.step, writer.tag(6, WireType.LengthDelimited).fork(), options).join();
        /* protos.PipelineStepNotification notification = 7; */
        if (message.notification)
            PipelineStepNotification.internalBinaryWrite(message.notification, writer.tag(7, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.NotifyRequest
 */
export const NotifyRequest = new NotifyRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class MetricsRequest$Type extends MessageType {
    constructor() {
        super("protos.MetricsRequest", [
            { no: 1, name: "metrics", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => Metric }
        ]);
    }
    create(value) {
        const message = { metrics: [] };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* repeated protos.Metric metrics */ 1:
                    message.metrics.push(Metric.internalBinaryRead(reader, reader.uint32(), options));
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
    internalBinaryWrite(message, writer, options) {
        /* repeated protos.Metric metrics = 1; */
        for (let i = 0; i < message.metrics.length; i++)
            Metric.internalBinaryWrite(message.metrics[i], writer.tag(1, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.MetricsRequest
 */
export const MetricsRequest = new MetricsRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class RegisterRequest$Type extends MessageType {
    constructor() {
        super("protos.RegisterRequest", [
            { no: 1, name: "service_name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "session_id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "client_info", kind: "message", T: () => ClientInfo },
            { no: 4, name: "audiences", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => Audience },
            { no: 5, name: "dry_run", kind: "scalar", T: 8 /*ScalarType.BOOL*/ }
        ]);
    }
    create(value) {
        const message = { serviceName: "", sessionId: "", audiences: [], dryRun: false };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string service_name */ 1:
                    message.serviceName = reader.string();
                    break;
                case /* string session_id */ 2:
                    message.sessionId = reader.string();
                    break;
                case /* protos.ClientInfo client_info */ 3:
                    message.clientInfo = ClientInfo.internalBinaryRead(reader, reader.uint32(), options, message.clientInfo);
                    break;
                case /* repeated protos.Audience audiences */ 4:
                    message.audiences.push(Audience.internalBinaryRead(reader, reader.uint32(), options));
                    break;
                case /* bool dry_run */ 5:
                    message.dryRun = reader.bool();
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
    internalBinaryWrite(message, writer, options) {
        /* string service_name = 1; */
        if (message.serviceName !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.serviceName);
        /* string session_id = 2; */
        if (message.sessionId !== "")
            writer.tag(2, WireType.LengthDelimited).string(message.sessionId);
        /* protos.ClientInfo client_info = 3; */
        if (message.clientInfo)
            ClientInfo.internalBinaryWrite(message.clientInfo, writer.tag(3, WireType.LengthDelimited).fork(), options).join();
        /* repeated protos.Audience audiences = 4; */
        for (let i = 0; i < message.audiences.length; i++)
            Audience.internalBinaryWrite(message.audiences[i], writer.tag(4, WireType.LengthDelimited).fork(), options).join();
        /* bool dry_run = 5; */
        if (message.dryRun !== false)
            writer.tag(5, WireType.Varint).bool(message.dryRun);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.RegisterRequest
 */
export const RegisterRequest = new RegisterRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class DeregisterRequest$Type extends MessageType {
    constructor() {
        super("protos.DeregisterRequest", [
            { no: 1, name: "service_name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "session_id", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value) {
        const message = { serviceName: "", sessionId: "" };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string service_name */ 1:
                    message.serviceName = reader.string();
                    break;
                case /* string session_id */ 2:
                    message.sessionId = reader.string();
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
    internalBinaryWrite(message, writer, options) {
        /* string service_name = 1; */
        if (message.serviceName !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.serviceName);
        /* string session_id = 2; */
        if (message.sessionId !== "")
            writer.tag(2, WireType.LengthDelimited).string(message.sessionId);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.DeregisterRequest
 */
export const DeregisterRequest = new DeregisterRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetSetPipelinesCommandsByServiceRequest$Type extends MessageType {
    constructor() {
        super("protos.GetSetPipelinesCommandsByServiceRequest", [
            { no: 1, name: "service_name", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value) {
        const message = { serviceName: "" };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string service_name */ 1:
                    message.serviceName = reader.string();
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
    internalBinaryWrite(message, writer, options) {
        /* string service_name = 1; */
        if (message.serviceName !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.serviceName);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.GetSetPipelinesCommandsByServiceRequest
 */
export const GetSetPipelinesCommandsByServiceRequest = new GetSetPipelinesCommandsByServiceRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetSetPipelinesCommandsByServiceResponse$Type extends MessageType {
    constructor() {
        super("protos.GetSetPipelinesCommandsByServiceResponse", [
            { no: 1, name: "set_pipeline_commands", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => Command },
            { no: 3, name: "wasm_modules", kind: "map", K: 9 /*ScalarType.STRING*/, V: { kind: "message", T: () => WasmModule } }
        ]);
    }
    create(value) {
        const message = { setPipelineCommands: [], wasmModules: {} };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* repeated protos.Command set_pipeline_commands */ 1:
                    message.setPipelineCommands.push(Command.internalBinaryRead(reader, reader.uint32(), options));
                    break;
                case /* map<string, protos.WasmModule> wasm_modules */ 3:
                    this.binaryReadMap3(message.wasmModules, reader, options);
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
    binaryReadMap3(map, reader, options) {
        let len = reader.uint32(), end = reader.pos + len, key, val;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case 1:
                    key = reader.string();
                    break;
                case 2:
                    val = WasmModule.internalBinaryRead(reader, reader.uint32(), options);
                    break;
                default: throw new globalThis.Error("unknown map entry field for field protos.GetSetPipelinesCommandsByServiceResponse.wasm_modules");
            }
        }
        map[key !== null && key !== void 0 ? key : ""] = val !== null && val !== void 0 ? val : WasmModule.create();
    }
    internalBinaryWrite(message, writer, options) {
        /* repeated protos.Command set_pipeline_commands = 1; */
        for (let i = 0; i < message.setPipelineCommands.length; i++)
            Command.internalBinaryWrite(message.setPipelineCommands[i], writer.tag(1, WireType.LengthDelimited).fork(), options).join();
        /* map<string, protos.WasmModule> wasm_modules = 3; */
        for (let k of Object.keys(message.wasmModules)) {
            writer.tag(3, WireType.LengthDelimited).fork().tag(1, WireType.LengthDelimited).string(k);
            writer.tag(2, WireType.LengthDelimited).fork();
            WasmModule.internalBinaryWrite(message.wasmModules[k], writer, options);
            writer.join().join();
        }
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.GetSetPipelinesCommandsByServiceResponse
 */
export const GetSetPipelinesCommandsByServiceResponse = new GetSetPipelinesCommandsByServiceResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class WasmModule$Type extends MessageType {
    constructor() {
        super("protos.WasmModule", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "bytes", kind: "scalar", T: 12 /*ScalarType.BYTES*/ },
            { no: 3, name: "function", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value) {
        const message = { id: "", bytes: new Uint8Array(0), function: "" };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string id */ 1:
                    message.id = reader.string();
                    break;
                case /* bytes bytes */ 2:
                    message.bytes = reader.bytes();
                    break;
                case /* string function */ 3:
                    message.function = reader.string();
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
    internalBinaryWrite(message, writer, options) {
        /* string id = 1; */
        if (message.id !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.id);
        /* bytes bytes = 2; */
        if (message.bytes.length)
            writer.tag(2, WireType.LengthDelimited).bytes(message.bytes);
        /* string function = 3; */
        if (message.function !== "")
            writer.tag(3, WireType.LengthDelimited).string(message.function);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.WasmModule
 */
export const WasmModule = new WasmModule$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SendSchemaRequest$Type extends MessageType {
    constructor() {
        super("protos.SendSchemaRequest", [
            { no: 1, name: "audience", kind: "message", T: () => Audience },
            { no: 2, name: "schema", kind: "message", T: () => Schema }
        ]);
    }
    create(value) {
        const message = {};
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial(this, message, value);
        return message;
    }
    internalBinaryRead(reader, length, options, target) {
        let message = target !== null && target !== void 0 ? target : this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* protos.Audience audience */ 1:
                    message.audience = Audience.internalBinaryRead(reader, reader.uint32(), options, message.audience);
                    break;
                case /* protos.Schema schema */ 2:
                    message.schema = Schema.internalBinaryRead(reader, reader.uint32(), options, message.schema);
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
    internalBinaryWrite(message, writer, options) {
        /* protos.Audience audience = 1; */
        if (message.audience)
            Audience.internalBinaryWrite(message.audience, writer.tag(1, WireType.LengthDelimited).fork(), options).join();
        /* protos.Schema schema = 2; */
        if (message.schema)
            Schema.internalBinaryWrite(message.schema, writer.tag(2, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message protos.SendSchemaRequest
 */
export const SendSchemaRequest = new SendSchemaRequest$Type();
/**
 * @generated ServiceType for protobuf service protos.Internal
 */
export const Internal = new ServiceType("protos.Internal", [
    { name: "Register", serverStreaming: true, options: {}, I: RegisterRequest, O: Command },
    { name: "NewAudience", options: {}, I: NewAudienceRequest, O: StandardResponse },
    { name: "Heartbeat", options: {}, I: HeartbeatRequest, O: StandardResponse },
    { name: "Notify", options: {}, I: NotifyRequest, O: StandardResponse },
    { name: "Metrics", options: {}, I: MetricsRequest, O: StandardResponse },
    { name: "GetSetPipelinesCommandsByService", options: {}, I: GetSetPipelinesCommandsByServiceRequest, O: GetSetPipelinesCommandsByServiceResponse },
    { name: "SendTail", clientStreaming: true, options: {}, I: TailResponse, O: StandardResponse },
    { name: "SendSchema", options: {}, I: SendSchemaRequest, O: StandardResponse }
]);
