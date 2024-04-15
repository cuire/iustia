import { useMainButton } from "@tma.js/sdk-react";
import { useEffect } from "react";
import { useLocation } from "wouter";

export const HelloRoute: React.FC = () => {
  const mainButton = useMainButton();

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [_, setLocation] = useLocation();

  useEffect(() => {
    const listener = () => setLocation("/tinder");

    mainButton.enable().show();
    mainButton.on("click", listener);

    return () => {
      mainButton.off("click", listener);
      mainButton.hide();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    mainButton.setText(`Start Searching!`);
  }, [mainButton]);

  return <>Gallery</>;
};
