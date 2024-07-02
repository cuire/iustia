import { useEffect } from "react";
import {
  bindMiniAppCSSVars,
  bindThemeParamsCSSVars,
  bindViewportCSSVars,
  useClosingBehavior,
  useLaunchParams,
  useMiniApp,
  useSettingsButton,
  useThemeParams,
  useViewport,
} from "@tma.js/sdk-react";
import { AppRoot } from "@telegram-apps/telegram-ui";
import { Route, Switch, useLocation } from "wouter";

import "@telegram-apps/telegram-ui/dist/styles.css";

import TinderRoute from "./tinder";
import HelloRoute from "./hello";
import { Theme } from "./debug/theme";
import { ProfileEditRoute } from "./profile";

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

  const lp = useLaunchParams();
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
    <AppRoot
      appearance={miniApp.isDark ? "dark" : "light"}
      platform={["macos", "ios"].includes(lp.platform) ? "ios" : "base"}
    >
      <main className="flex flex-col h-screen overflow-hidden">
        <div className="flex-1 p-6 relative">
          <Switch>
            <Route path="/" component={HelloRoute} />

            <Route path="/tinder" component={TinderRoute} />

            <Route path="/debug/theme" component={Theme} />

            <Route path="/profile/edit" component={ProfileEditRoute} />

            <Route>404: No such page!</Route>
          </Switch>
        </div>
      </main>
    </AppRoot>
  );
};
