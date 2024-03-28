from shared import sp_shared_pb2 as _sp_shared_pb2
import sp_command_pb2 as _sp_command_pb2
import sp_common_pb2 as _sp_common_pb2
import sp_info_pb2 as _sp_info_pb2
import sp_pipeline_pb2 as _sp_pipeline_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class DeregisterRequest(_message.Message):
    __slots__ = ["service_name", "session_id"]
    SERVICE_NAME_FIELD_NUMBER: _ClassVar[int]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    service_name: str
    session_id: str
    def __init__(self, service_name: _Optional[str] = ..., session_id: _Optional[str] = ...) -> None: ...

class GetSetPipelinesCommandsByServiceRequest(_message.Message):
    __slots__ = ["service_name"]
    SERVICE_NAME_FIELD_NUMBER: _ClassVar[int]
    service_name: str
    def __init__(self, service_name: _Optional[str] = ...) -> None: ...

class GetSetPipelinesCommandsByServiceResponse(_message.Message):
    __slots__ = ["set_pipeline_commands", "wasm_modules"]
    class WasmModulesEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: _sp_shared_pb2.WasmModule
        def __init__(self, key: _Optional[str] = ..., value: _Optional[_Union[_sp_shared_pb2.WasmModule, _Mapping]] = ...) -> None: ...
    SET_PIPELINE_COMMANDS_FIELD_NUMBER: _ClassVar[int]
    WASM_MODULES_FIELD_NUMBER: _ClassVar[int]
    set_pipeline_commands: _containers.RepeatedCompositeFieldContainer[_sp_command_pb2.Command]
    wasm_modules: _containers.MessageMap[str, _sp_shared_pb2.WasmModule]
    def __init__(self, set_pipeline_commands: _Optional[_Iterable[_Union[_sp_command_pb2.Command, _Mapping]]] = ..., wasm_modules: _Optional[_Mapping[str, _sp_shared_pb2.WasmModule]] = ...) -> None: ...

class HeartbeatRequest(_message.Message):
    __slots__ = ["audiences", "client_info", "service_name", "session_id"]
    AUDIENCES_FIELD_NUMBER: _ClassVar[int]
    CLIENT_INFO_FIELD_NUMBER: _ClassVar[int]
    SERVICE_NAME_FIELD_NUMBER: _ClassVar[int]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    audiences: _containers.RepeatedCompositeFieldContainer[_sp_common_pb2.Audience]
    client_info: _sp_info_pb2.ClientInfo
    service_name: str
    session_id: str
    def __init__(self, session_id: _Optional[str] = ..., service_name: _Optional[str] = ..., audiences: _Optional[_Iterable[_Union[_sp_common_pb2.Audience, _Mapping]]] = ..., client_info: _Optional[_Union[_sp_info_pb2.ClientInfo, _Mapping]] = ...) -> None: ...

class MetricsRequest(_message.Message):
    __slots__ = ["metrics"]
    METRICS_FIELD_NUMBER: _ClassVar[int]
    metrics: _containers.RepeatedCompositeFieldContainer[_sp_common_pb2.Metric]
    def __init__(self, metrics: _Optional[_Iterable[_Union[_sp_common_pb2.Metric, _Mapping]]] = ...) -> None: ...

class NewAudienceRequest(_message.Message):
    __slots__ = ["audience", "session_id"]
    AUDIENCE_FIELD_NUMBER: _ClassVar[int]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    audience: _sp_common_pb2.Audience
    session_id: str
    def __init__(self, session_id: _Optional[str] = ..., audience: _Optional[_Union[_sp_common_pb2.Audience, _Mapping]] = ...) -> None: ...

class NotifyRequest(_message.Message):
    __slots__ = ["audience", "condition_type", "occurred_at_unix_ts_utc", "payload", "pipeline_id", "step"]
    class ConditionType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    AUDIENCE_FIELD_NUMBER: _ClassVar[int]
    CONDITION_TYPE_FIELD_NUMBER: _ClassVar[int]
    CONDITION_TYPE_ON_ERROR: NotifyRequest.ConditionType
    CONDITION_TYPE_ON_FALSE: NotifyRequest.ConditionType
    CONDITION_TYPE_ON_TRUE: NotifyRequest.ConditionType
    CONDITION_TYPE_UNSET: NotifyRequest.ConditionType
    OCCURRED_AT_UNIX_TS_UTC_FIELD_NUMBER: _ClassVar[int]
    PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    PIPELINE_ID_FIELD_NUMBER: _ClassVar[int]
    STEP_FIELD_NUMBER: _ClassVar[int]
    audience: _sp_common_pb2.Audience
    condition_type: NotifyRequest.ConditionType
    occurred_at_unix_ts_utc: int
    payload: bytes
    pipeline_id: str
    step: _sp_pipeline_pb2.PipelineStep
    def __init__(self, condition_type: _Optional[_Union[NotifyRequest.ConditionType, str]] = ..., step: _Optional[_Union[_sp_pipeline_pb2.PipelineStep, _Mapping]] = ..., audience: _Optional[_Union[_sp_common_pb2.Audience, _Mapping]] = ..., occurred_at_unix_ts_utc: _Optional[int] = ..., pipeline_id: _Optional[str] = ..., payload: _Optional[bytes] = ...) -> None: ...

class RegisterRequest(_message.Message):
    __slots__ = ["audiences", "client_info", "dry_run", "service_name", "session_id"]
    AUDIENCES_FIELD_NUMBER: _ClassVar[int]
    CLIENT_INFO_FIELD_NUMBER: _ClassVar[int]
    DRY_RUN_FIELD_NUMBER: _ClassVar[int]
    SERVICE_NAME_FIELD_NUMBER: _ClassVar[int]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    audiences: _containers.RepeatedCompositeFieldContainer[_sp_common_pb2.Audience]
    client_info: _sp_info_pb2.ClientInfo
    dry_run: bool
    service_name: str
    session_id: str
    def __init__(self, service_name: _Optional[str] = ..., session_id: _Optional[str] = ..., client_info: _Optional[_Union[_sp_info_pb2.ClientInfo, _Mapping]] = ..., audiences: _Optional[_Iterable[_Union[_sp_common_pb2.Audience, _Mapping]]] = ..., dry_run: bool = ...) -> None: ...

class SendSchemaRequest(_message.Message):
    __slots__ = ["audience", "schema"]
    AUDIENCE_FIELD_NUMBER: _ClassVar[int]
    SCHEMA_FIELD_NUMBER: _ClassVar[int]
    audience: _sp_common_pb2.Audience
    schema: _sp_common_pb2.Schema
    def __init__(self, audience: _Optional[_Union[_sp_common_pb2.Audience, _Mapping]] = ..., schema: _Optional[_Union[_sp_common_pb2.Schema, _Mapping]] = ...) -> None: ...
