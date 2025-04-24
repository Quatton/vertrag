import { useORPC } from "@/lib/client";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
	component: App,
});

function App() {
	const orpc = useORPC();

	const { data, error } = useQuery(
		orpc.products.list.queryOptions({
			input: {},
		}),
	);

	return (
		<main className="flex h-screen w-screen flex-col items-center justify-center gap-4">
			<section className="max-w-4xl">
				<h1 className="text-center font-semibold text-4xl">
					Seamless handshakes between Client and Server <br /> in two different
					languages
				</h1>
			</section>
			<section className="max-w-4xl">
				<h2 className="text-center font-semibold text-2xl">Products</h2>
				{data?.products.map((product) => (
					<div key={product.id} className="flex flex-col gap-2">
						<h3 className="font-semibold text-lg">{product.name}</h3>
						<p>{product.description}</p>
						<p>${product.price}</p>
						{product.discountedPrice ? (
							<p className="text-red-500">
								Discounted Price: ${product.discountedPrice}
							</p>
						) : null}
					</div>
				))}
				{error ? (
					<div className="text-red-500">
						<p>Error: {error.message}</p>
					</div>
				) : null}
			</section>
		</main>
	);
}
