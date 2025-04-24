import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: App,
});

function App() {
  return (
    <main className="flex h-screen w-screen items-center justify-center">
      <h1>
        Seamless Handshakes between Client and Server in multiple languages
      </h1>
    </main>
  );
}
