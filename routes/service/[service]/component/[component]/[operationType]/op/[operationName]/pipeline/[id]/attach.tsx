import { Handlers } from "$fresh/src/server/types.ts";
import { SuccessType } from "../../../../../../../../../_middleware.ts";
import { OperationType, ResponseCode } from "snitch-protos/protos/common.ts";
import { attachPipeline } from "../../../../../../../../../../lib/mutation.ts";
import { HandlerContext } from "$fresh/server.ts";

export const handler: Handlers<SuccessType> = {
  async POST(req, { params }: HandlerContext) {
    const response = await attachPipeline(params.id, {
      serviceName: params.service,
      componentName: params.component,
      operationType: OperationType[params.operationType],
      operationName: params.operationName,
    });

    return new Response(
      JSON.stringify({
        success: {
          status: response.code === ResponseCode.OK,
          message: response.code === ResponseCode.OK
            ? "Pipeline successfully attached"
            : response.message,
        },
      }),
      { status: response.code === ResponseCode.OK ? 200 : 400 },
    );
  },
};

export default function AttachPipeLineRoute() {
  return null;
}
