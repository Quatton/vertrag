import { Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

import "@fontsource-variable/dm-sans";
import "../styles.css";

import { type ContractClient, ORPCContext } from "@/lib/client";
import { createORPCReactQueryUtils } from "@orpc/react-query";
import { useState } from "react";
import { OpenAPILink } from "@orpc/openapi-client/fetch";
import { contract } from "@contract/routes/root";
import { createORPCClient } from "@orpc/client";
import type { JsonifiedClient } from "@orpc/openapi-client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

export const Route = createRootRoute({
	component: RootComponent,
});

const link = new OpenAPILink(contract, {
	url: "http://localhost:8000",
});

const queryClient = new QueryClient();

function RootComponent() {
	const [client] = useState(() =>
		createORPCClient<JsonifiedClient<ContractClient>>(link),
	);
	const [orpc] = useState(() =>
		createORPCReactQueryUtils<ContractClient>(client),
	);

	return (
		<ORPCContext.Provider value={orpc}>
			<QueryClientProvider client={queryClient}>
				<Outlet />
				<TanStackRouterDevtools />
			</QueryClientProvider>
		</ORPCContext.Provider>
	);
}
