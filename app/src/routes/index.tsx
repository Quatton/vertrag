import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: App,
});

function App() {
  return (
    <main className="flex h-screen w-screen items-center justify-center">
      <section className="max-w-4xl">
        <h1 className="text-4xl font-semibold text-center">
          Seamless handshakes between Client and Server <br/> in two different languages
        </h1>
      </section>
    </main>
  );
}
