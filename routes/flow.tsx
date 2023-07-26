import { Layout } from "../components/layout.tsx";
import { RuleSetType } from "../components/rules/sets.tsx";
import { PageProps } from "$fresh/src/server/types.ts";
import Flow from "../islands/flow.tsx";
import {
  ReactFlowProvider,
} from "https://esm.sh/v128/@reactflow/core@11.7.4/X-YS9AdHlwZXMvcmVhY3Q6cHJlYWN0L2NvbXBhdCxyZWFjdC1kb206cHJlYWN0L2NvbXBhdCxyZWFjdDpwcmVhY3QvY29tcGF0CmUvcHJlYWN0L2NvbXBhdA/denonext/core.mjs";

export default function FlowRoute() {
  return (
    <Layout>
      <ReactFlowProvider>
        <Flow />
      </ReactFlowProvider>
    </Layout>
  );
}
