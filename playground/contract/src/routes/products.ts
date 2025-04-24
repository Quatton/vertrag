import { oc } from "@orpc/contract";
import * as z from "zod";

export const Product = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string(),
  price: z.number(),
  discountedPrice: z.number().optional(),
});

const PRODUCT = "Product";

export const products = {
  list: oc
    .route({
      tags: [PRODUCT],
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
  create: oc
    .route({
      tags: [PRODUCT],
      method: "POST",
      path: "/products",
    })
    .input(
      z.object({
        name: z.string().min(1),
        description: z.string(),
        price: z.number(),
        discountedPrice: z.number().optional(),
      })
    )
    .output(
      z.object({
        success: z.boolean(),
      })
    ),
};
