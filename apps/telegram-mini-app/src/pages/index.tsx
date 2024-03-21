import { useEffect } from "react";
import { useClosingBehavior } from "@tma.js/sdk-react";
import { Route, Switch } from "wouter";

import TinderRoute from "./tinder";
import GalleryRoute from "./gallery";
import { Theme } from "./debug/theme";

import Nav from "../lib/components/nav/";

export const App = () => {
  const closingBehaviour = useClosingBehavior();
  useEffect(() => {
    closingBehaviour.enableConfirmation();
  }, [closingBehaviour]);

  return (
    <main className="flex flex-col h-screen text-base-content overflow-hidden">
      <div className="flex-1 p-6 bg-base-100 relative">
        <Switch>
          <Route path="/" component={GalleryRoute} />

          <Route path="/tinder" component={TinderRoute} />

          <Route path="/debug/theme" component={Theme} />

          <Route>404: No such page!</Route>
        </Switch>
      </div>

      <Nav />
    </main>
  );
};
