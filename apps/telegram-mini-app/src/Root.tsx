import { SDKProvider, DisplayGate } from "@tma.js/sdk-react";
import { App } from "./pages";

interface SDKProviderErrorProps {
  error: unknown;
}

function SDKProviderError({ error }: SDKProviderErrorProps) {
  return (
    <div>
      Oops. Something went wrong.
      <blockquote>
        <code>
          {error instanceof Error ? error.message : JSON.stringify(error)}
        </code>
      </blockquote>
    </div>
  );
}

function SDKProviderLoading() {
  return <div>SDK is loading.</div>;
}

function SDKInitialState() {
  return <div>Waiting for initialization to start.</div>;
}

/**
 * Root component of the whole project.
 */
export function Root() {
  return (
    <SDKProvider
      options={{ acceptCustomStyles: true, cssVars: true, async: true }}
    >
      <DisplayGate
        error={SDKProviderError}
        loading={SDKProviderLoading}
        initial={SDKInitialState}
      >
        <App />
      </DisplayGate>
    </SDKProvider>
  );
}
