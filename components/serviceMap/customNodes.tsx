import { Handle, Position } from "reactflow";
import IconGripVertical from "tabler-icons/tsx/grip-vertical.tsx";
import IconDatabase from "tabler-icons/tsx/database.tsx";
import "twind";
import { OperationType } from "snitch-protos/protos/sp_common.ts";
import { ServiceNodeMenu } from "./nodeMenu.tsx";
import { ProducerIcon } from "../icons/producer.tsx";
import { ConsumerIcon } from "../icons/consumer.tsx";
import { removeWhitespace, titleCase } from "../../lib/utils.ts";
import { Tooltip } from "../tooltip/tooltip.tsx";
import { NodeData } from "../../lib/nodeMapper.ts";
import { opModal } from "./opModalSignal.ts";

export const ServiceNode = ({ data }: { data: NodeData }) => {
  return (
    <div>
      <div
        class="min-h-[80px] w-[320px] flex items-center justify-between bg-white rounded-lg shadow-lg z-10 border-1 border-purple-200 px-2"
        id="dragHandle"
      >
        <div class="flex flex-row items-center">
          <IconGripVertical class="w-6 h-6 text-purple-100 mr-1" />
          <img
            src={"/images/placeholder-icon.png"}
            className={"h-[40px]"}
          />
          <div class="flex flex-col ml-2">
            <h2 className={"text-lg"}>{data.audience.serviceName}</h2>
          </div>
        </div>
        <ServiceNodeMenu data={data} />
      </div>

      <div className={"flex justify-evenly w-1/2 mt-2"}>
        <Handle
          type="target"
          position={Position.Bottom}
          className="opacity-0 relative"
        />
        <Handle
          type="source"
          position={Position.Bottom}
          className="opacity-0 relative"
        />
      </div>
    </div>
  );
};

export const GroupNode = ({ data }: { data: NodeData }) => {
  const op = OperationType[data.audience.operationType];
  const producer = op === OperationType[OperationType.PRODUCER];

  const height = 124 + ((data.groupCount - 1) * 76);

  return (
    <div
      class={`rounded-lg shadow-lg border-1 border-purple-200 min-w-[280px] min-h-[${height}px]`}
    >
      <div id="dragHandle" class="flex flex-row items-center mt-2">
        <IconGripVertical class="w-6 h-6 mx-2 text-purple-100 bg-white border border-purple-200" />
        {producer
          ? <ProducerIcon class="w-5 h-5 mr-2" />
          : <ConsumerIcon class="w-5 h-5 mr-2" />}
        {`${titleCase(op)}s`}
      </div>

      <Handle
        type="source"
        position={producer ? Position.Bottom : Position.Top}
        className="opacity-0"
      />
      <Handle
        type="target"
        position={producer ? Position.Top : Position.Bottom}
        className="opacity-0"
      />
    </div>
  );
};

export const OperationNode = ({ data }: { data: NodeData }) => {
  const toolTipId = removeWhitespace(data.audience.operationName);

  return (
    <div class="h-[96px]">
      <div
        type="button"
        class="flex items-center justify-between w-[260px] h-[64px] bg-white rounded-lg shadow-lg border-1 border-purple-200 pl-1 pr-2"
      >
        <div class="w-[170px] whitespace-nowrap text-ellipsis overflow-hidden">
          <div
            class={"flex flex-col justify-start p-1 cursor-pointer"}
            onClick={() =>
              opModal.value = {
                audience: data.audience,
                attachedPipeline: data.attachedPipeline,
                clients: data.clients?.length,
              }}
          >
            <h2
              data-tooltip-target={toolTipId}
              class={"text-[16px] whitespace-nowrap text-ellipsis overflow-hidden"}
            >
              {data.audience.operationName}
            </h2>
            <Tooltip
              targetId={toolTipId}
              message={"Click to attach and detach pipelines"}
            />
            <h3 class="text-xs text-streamdalPurple font-semibold">
              {`${data.clients?.length || 0} attached client${
                data.clients?.length !== 1 ? "s" : ""
              }`}
            </h3>
          </div>
        </div>
      </div>

      <Handle
        type="source"
        position={Position.Top}
        className="opacity-0"
      />
      <Handle
        type="target"
        position={Position.Bottom}
        className="opacity-0"
      />
    </div>
  );
};

export const ComponentImage = (
  { componentName }: { componentName: string },
) => {
  if (componentName.toLowerCase().includes("kafka")) {
    return (
      <img
        src={"/images/kafka-dark.svg"}
        className="w-[30px]"
      />
    );
  }

  if (componentName.toLowerCase().includes("postgres")) {
    return (
      <img
        src={"/images/postgresql.svg"}
        className="w-[30px]"
      />
    );
  }

  return <IconDatabase class="w-6 h-6" />;
};

export const ComponentNode = ({ data }: { data: NodeData }) => {
  return (
    <div
      id="dragHandle"
      class="z-0 bg-web rounded-md border-1 border-black h-[145px] w-[145px] shadow-xl flex justify-center items-center"
    >
      <div class="flex justify-center flex-col items-center">
        <ComponentImage componentName={data.audience.componentName} />
        <p class={"z-10 mt-2 text-white"}>{data.audience.componentName}</p>
      </div>
      <Handle
        type="source"
        position={Position.Left}
        className="opacity-0"
      />
      <Handle
        type="target"
        position={Position.Right}
        className="opacity-0"
      />
    </div>
  );
};
