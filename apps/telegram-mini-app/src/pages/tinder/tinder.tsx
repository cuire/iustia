import { useState } from "react";
import TinderCard from "react-tinder-card";

import { Icon, Like } from "$lib/components/icon";

type CardProps = {
  className?: string;
  images?: string[];
  title: string;
  description: string;
};

const Card: React.FC<CardProps> = ({ description }) => {
  return (
    <>
      <figure className="relative h-full w-full rounded-xl overflow-hidden flex flex-col-reverse">
        <img
          className="absolute inset-0 object-cover w-full h-full -z-10 overflow-hidden"
          src="https://images.unsplash.com/photo-1497215728101-856f4ea42174"
          alt=""
          draggable="false"
        />
        <div className="mx-4 p-4 bg-base-200 rounded-t-xl">
          <blockquote>
            <p className="text-lg font-medium">{description}</p>
          </blockquote>
          <figcaption className="font-medium">
            <div className="text-sky-500 dark:text-sky-400">Sarah Dayan</div>
            <div className="text-slate-700 dark:text-slate-500">
              Staff Engineer, Algolia
            </div>
          </figcaption>
        </div>
      </figure>
    </>
  );
};

const Controlls: React.FC = () => {
  return (
    <div className="flex justify-center space-x-4">
      <button className="p-4 bg-sky-500 text-white rounded-full">
        <Icon icon={Like} solid />
      </button>
      <button className="p-4 bg-slate-500 text-white rounded-full">
        <Icon icon={Like} />
      </button>
    </div>
  );
};

const cards = Array.from({ length: 10 }, (_, i) => i.toString());

export const TinderRoute: React.FC = () => {
  const [modalCardId, setModalCardId] = useState<string | null>(null);

  function onSwipeRequirementFulfilled(dir: string, cardId: string) {
    if (dir === "up") {
      setModalCardId(cardId);
    }
  }

  return (
    <>
      {modalCardId && (
        <div className="fixed inset-0 z-50 bg-black bg-opacity-50 flex justify-center items-end">
          <div className="bg-base-100 w-full p-4">
            <button onClick={() => setModalCardId(null)}>CLOSE</button>
          </div>
        </div>
      )}

      <div className="flex flex-col h-full">
        <div className="h-full w-full stack">
          {cards.map((cardId) => (
            <TinderCard
              className="h-full w-full overflow-hidden select-none"
              preventSwipe={["up"]}
              onSwipe={(dir) => console.log(dir)}
              onSwipeRequirementFulfilled={(dir) =>
                onSwipeRequirementFulfilled(dir, cardId)
              }
            >
              <Card title="" description={`TEST ${cardId}`} key={cardId} />
            </TinderCard>
          ))}
        </div>

        <Controlls />
      </div>
    </>
  );
};
