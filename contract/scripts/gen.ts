import { OpenAPIGenerator } from "@orpc/openapi";
import { contract } from "../src/routes/root";
import { ZodToJsonSchemaConverter } from "@orpc/zod";

const generator = new OpenAPIGenerator({
  schemaConverters: [new ZodToJsonSchemaConverter()],
});

const spec = await generator.generate(contract, {
  info: {
    title: "Planet API",
    version: "1.0.0",
  },
});

const output = JSON.stringify(spec, null, 2);
const outputPath = "./openapi.json";
Bun.write(outputPath, output);
