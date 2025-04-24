import { oc } from "@orpc/contract";
import * as z from "zod";

export const Product = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string(),
  price: z.number(),
});

export const products = {
  list: oc
    .route({
      method: "GET",
      path: "/products",
    })
    .input(
      z.object({
        page: z.number().optional(),
        limit: z.number().optional(),
      })
    )
    .output(
      z.object({
        products: z.array(Product),
        total: z.number(),
        page: z.number(),
        limit: z.number(),
      })
    ),
};
