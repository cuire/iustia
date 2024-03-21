import { useState } from "react";
import TinderCard from "react-tinder-card";

import { Dislike, Icon, Info, Like } from "$lib/components/icon";

const job = {
  id: 1,
  title: "Senior Python Developer to Iustia",
  shortDescription: `
  We are looking for an experienced colleague who will help us develop Iustia
   - an internal Yadex service for searching, assessing and hiring employees. 
   Come if you write efficient and understandable code in Python, understand 
   the principles of web services and have experience interacting with databases.
  `,
  company: 1,
  location: null,
  job_type: "full_time",
  is_active: true,
  images: [
    {
      image_url: "https://images.unsplash.com/photo-1497215728101-856f4ea42174",
    },
  ],
};

type CardProps = {
  className?: string;
  images?: string[];
  title: string;
  description: string;
};

const Card: React.FC<CardProps> = ({ title }) => {
  return (
    <>
      <figure className="relative h-full w-full rounded-xl overflow-hidden flex flex-col-reverse border-2 border-base-200">
        <img
          className="absolute inset-0 object-cover w-full h-full -z-20 overflow-hidden"
          src="https://images.unsplash.com/photo-1497215728101-856f4ea42174"
          alt=""
          draggable="false"
        />

        <div className="flex justify-center space-x-7 bg-white/75 dark:bg-black/75 pb-4">
          <button className="btn btn-circle">
            <Icon icon={Like} solid />
          </button>
          <button className="btn btn-circle">
            <Icon icon={Info} />
          </button>
          <button className="btn btn-circle">
            <Icon icon={Dislike} />
          </button>
        </div>

        <div className="p-4 rounded-t-xl max-h-[80%] relative overflow-visible">
          <div className="absolute inset-0 -top-20 bg-gradient-to-t from-65% from-white/75 dark:from-black/75 to-transparent -z-10" />

          <div className="z-10 relative">
            <blockquote>
              <div className="text-primary text-sm">
                Yadex - Saint Petersburg
              </div>
              <h2 className="text-lg font-medium">{title}</h2>
            </blockquote>
            <figcaption className="font-medium">
              <div className="flex justify-between">{job.shortDescription}</div>
            </figcaption>
          </div>
        </div>
      </figure>
    </>
  );
};

const cards = Array.from({ length: 10 }, (_, i) => i.toString());

export const TinderRoute: React.FC = () => {
  const [modalCardId, setModalCardId] = useState<string | null>(null);

  return (
    <>
      {modalCardId && (
        <div className="fixed inset-0 z-50 bg-black bg-opacity-50 flex justify-center items-end">
          <div className="bg-base-200 w-full min-h-40 max-h-[50%] rounded-t-xl overflow-scroll">
            <div className="sticky top-0 right-0 bg-inherit p-4">
              <button
                className="text-primary"
                onClick={() => setModalCardId(null)}
              >
                CLOSE
              </button>
            </div>
            <article className="flex flex-col gap-2 p-4">
              <h1 className="text-3xl">{job.title}</h1>
              <p>{job.shortDescription}</p>
            </article>
          </div>
        </div>
      )}

      <div className="flex flex-col h-full gap-6">
        <div className="h-full w-full stack">
          {cards.map((cardId) => (
            <TinderCard
              className="h-full w-full overflow-hidden select-none"
              preventSwipe={["up", "down"]}
              onSwipe={(dir) => console.log(dir)}
            >
              <Card title={job.title} description={job.title} key={cardId} />
            </TinderCard>
          ))}
        </div>
      </div>
    </>
  );
};
