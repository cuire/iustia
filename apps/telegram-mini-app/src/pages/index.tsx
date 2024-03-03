import { Route, Switch } from "wouter";

import TinderRoute from "./tinder";
import GalleryRoute from "./gallery";
import { Theme } from "./debug/theme";

import Nav from "../lib/components/nav/";

export const App = () => (
  <main className="flex flex-col min-h-screen text-base-content">
    <div className="flex-1 p-6 bg-base-100">
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
