import { SDKProvider } from "@tma.js/sdk-react";
import { App } from "./pages";

export function Root() {
  return (
    <SDKProvider acceptCustomStyles>
      <App />
    </SDKProvider>
  );
}
