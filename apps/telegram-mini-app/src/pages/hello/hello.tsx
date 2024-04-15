import { useMainButton } from "@tma.js/sdk-react";
import { useEffect } from "react";
import { useLocation } from "wouter";
import Lottie from "lottie-react";

import plane from "$lib/assets/plane.json";

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

  return (
    <div className="flex flex-col items-center h-full">
      <Lottie animationData={plane} className="w-1/2 h-1/2 -my-16" />

      <h1 className="text-3xl font-bold text-center">Welcome to Iustia</h1>

      <p className="text-center">The best place to find your next job!</p>
    </div>
  );
};
