import type { ContractRouterClient } from "@orpc/contract";
import type { contract } from "@contract/routes/root";
import type { RouterUtils } from "@orpc/react-query";
import { createContext, use } from "react";

export type ContractClient = ContractRouterClient<typeof contract>;
type ORPCReactUtils = RouterUtils<ContractClient>;
export const ORPCContext = createContext<ORPCReactUtils | undefined>(undefined);

export function useORPC(): ORPCReactUtils {
	const orpc = use(ORPCContext);
	if (!orpc) {
		throw new Error("ORPCContext is not set up properly");
	}
	return orpc;
}
