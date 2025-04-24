import { Heading } from "@/components/ui/heading";
import { Button } from "@/components/ui/styled/button";
import { createFileRoute } from "@tanstack/react-router";
import { css } from "styled-system/css";
import { center } from "styled-system/patterns";

export const Route = createFileRoute("/")({
  component: App,
});

function App() {
  return (
    <main
      className={center({
        height: "100dvh",
        flexDirection: "column",
      })}
    >
      <section
        className={center({
          maxWidth: "3xl",
        flexDirection: "column",

        })}
        >
        <Heading
          size="4xl"
          as="h1">
          Vertrag: Contract-first development with multi-language support
        </Heading>
        <Heading
          size="2xl"
        >
          Seamless frontend-backend handshakes for TypeScript, Python, and Rust
          (and more!)
        </Heading>
        <p
          className={css({
            fontSize: "lg",
            color: "text",
          })}
        >
          Packed with opinionated defaults where you can override them if you want
          to. Including:
        </p>
      </section>
      <section>

        <ul
          className={css({
            listStyleType: "disc",
            paddingLeft: "2rem",
            color: "text",
          })}
        >
          <li>TanStack Router</li>
          <li>TanStack Query</li>
          <li>PandaCSS w/ Park UI</li>
          <li>Biome Lint and Formatter</li>
        </ul>
        <Button>
          hi
        </Button>
      </section>
    </main>
  );
}
