import { useORPC, type ContractClient } from "@/lib/client";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { useState } from "react";

export const Route = createFileRoute("/")({
	component: App,
});

type CreateForm = Parameters<ContractClient["products"]["create"]>[0]

function App() {
	const orpc = useORPC();
	const queryClient = useQueryClient();

	const { data, error } = useQuery(
		orpc.products.list.queryOptions({
			input: {},
		}),
	);

	const [form, setForm] = useState<CreateForm>({
		name: "",
		description: "",
		price: 0,
		discountedPrice: undefined
	})

	const { mutate, isPending } = useMutation(
		orpc.products.create.mutationOptions({
			onSettled() {
				queryClient.invalidateQueries(
					{
						queryKey: orpc.products.list.key()
					}
				)
			}
		})
	)

	return (
		<main className="flex h-screen w-screen flex-col items-center justify-center gap-4">
			<section className="max-w-4xl">
				<h1 className="text-center font-semibold text-4xl">
					Seamless handshakes between Client and Server <br /> in two different
					languages
				</h1>
			</section>
			<section>
				<form 
					className="flex flex-col gap-4"
					onSubmit={(e) => {
					e.preventDefault()
					mutate(form)
				}}>
					<label className="flex flex-col gap-1">
						Name:
						<input 
							className="px-1 py-0.5 border rounded"
							type="text" onChange={
							(e) => setForm({
								...form,
								name: e.target.value
							})
						}/>
					</label>
					<label className="flex flex-col gap-1">
						Description:
						<input className="px-1 py-0.5 border rounded"
						type="text" onChange={
							(e) => setForm({
								...form,
								description: e.target.value
							})
						}/>
					</label>
					<label className="flex flex-col gap-1">
						Price:
						<input className="px-1 py-0.5 border rounded"
						type="number" onChange={
							(e) => setForm({
								...form,
								price: Number(e.target.value)
							})
						}/>
					</label>
					<label className="flex flex-col gap-1">
						Discounted Price
						<input className="px-1 py-0.5 border rounded"
						type="number" onChange={
							(e) => setForm({
								...form,
								discountedPrice: Number(e.target.value)
							})
						}/>
					</label>
					<button 
						type="submit"
						className="px-4 py-2 border rounded bg-blue-500 text-white hover:bg-blue-400">
						{
							isPending ? "Submitting" : "Submit"
						}
					</button>
				</form>
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
