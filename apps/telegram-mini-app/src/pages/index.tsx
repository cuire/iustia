import { useEffect } from "react";
import {
  bindMiniAppCSSVars,
  bindThemeParamsCSSVars,
  bindViewportCSSVars,
  useClosingBehavior,
  useMiniApp,
  useSettingsButton,
  useThemeParams,
  useViewport,
} from "@tma.js/sdk-react";
import { Route, Switch, useLocation } from "wouter";

import TinderRoute from "./tinder";
import HelloRoute from "./hello";
import { Theme } from "./debug/theme";

export const App = () => {
  const [_, setLocation] = useLocation();

  const closingBehaviour = useClosingBehavior();

  const settingsButton = useSettingsButton();

  useEffect(() => {
    closingBehaviour.enableConfirmation();
  }, [closingBehaviour]);

  useEffect(() => {
    settingsButton.show();
    const removeSettingsButtonClick = settingsButton.on("click", () => {
      setLocation("/debug/theme");
    });
    return () => {
      removeSettingsButtonClick();
    };
  }, [setLocation, settingsButton]);

  const miniApp = useMiniApp();
  const themeParams = useThemeParams();
  const viewport = useViewport();

  useEffect(() => {
    return bindMiniAppCSSVars(miniApp, themeParams);
  }, [miniApp, themeParams]);

  useEffect(() => {
    return bindThemeParamsCSSVars(themeParams);
  }, [themeParams]);

  useEffect(() => {
    return viewport && bindViewportCSSVars(viewport);
  }, [viewport]);

  return (
    <main className="flex flex-col h-screen text-base-content overflow-hidden">
      <div className="flex-1 p-6 bg-base-100 relative">
        <Switch>
          <Route path="/" component={HelloRoute} />

          <Route path="/tinder" component={TinderRoute} />

          <Route path="/debug/theme" component={Theme} />

          <Route>404: No such page!</Route>
        </Switch>
      </div>
    </main>
  );
};
