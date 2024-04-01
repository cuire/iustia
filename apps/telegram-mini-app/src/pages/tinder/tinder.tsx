import { useRef } from "react";
import TinderCard from "react-tinder-card";

import { Dislike, Icon, Info, Like } from "$lib/components/icon";
import { useVacancies, type Tag as TagType } from "$lib/hooks/api";

type CardProps = {
  className?: string;
  image?: string;
  title: string;
  description: string;
  tags?: TagType[];

  onLike?: () => void;
  onDislike?: () => void;
};

const Tag: React.FC<TagType> = (props) => {
  return (
    <div className="px-2 py-1 bg-base-200 text-base-content rounded-full">
      {props.tag}
    </div>
  );
};

const Card: React.FC<CardProps> = (props) => {
  const { title, description } = props;

  function handleLike() {
    if (props.onLike) {
      props.onLike();
    }
  }

  function handleDislike() {
    if (props.onDislike) {
      props.onDislike();
    }
  }

  return (
    <>
      <figure className="relative h-full w-full rounded-xl overflow-hidden flex flex-col-reverse border-2 border-base-200">
        {props.image ? (
          <img
            className="absolute inset-0 object-cover w-full h-full -z-20 overflow-hidden bg-base-200"
            src={props.image}
            alt=""
            draggable="false"
          />
        ) : (
          <div className="absolute inset-0 bg-base-200 -z-20">
            <div className="flex flex-col gap-2 items-center justify-center h-full text-3xl text-info">
              <div className="">(╥﹏╥) (╥﹏╥)</div>
              <div className="">RIP</div>
              <div className="">Image</div>
            </div>
          </div>
        )}

        <div className="bg-white/75 dark:bg-black/75 pb-4">
          {props.tags && props.tags.length > 0 && (
            <div className="flex gap-1 px-4 py-2">
              {props.tags?.map((tag) => <Tag key={tag.slug} {...tag} />)}
            </div>
          )}
          <div className="flex justify-center space-x-7">
            <button className="btn btn-circle" onClick={handleLike}>
              <Icon icon={Like} solid />
            </button>
            <button className="btn btn-circle">
              <Icon icon={Info} />
            </button>
            <button className="btn btn-circle" onClick={handleDislike}>
              <Icon icon={Dislike} />
            </button>
          </div>
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
              <div className="flex justify-between">{description}</div>
            </figcaption>
          </div>
        </div>
      </figure>
    </>
  );
};

const TinderJobCard: React.FC<CardProps> = (props) => {
  const ref = useRef<{
    swipe: (dir: "left" | "right" | "up" | "down") => Promise<void>;
    restoreCard: () => Promise<void>;
  }>(null);

  const handleLike = () => {
    ref.current?.swipe("left");
  };

  const handleDislike = () => {
    ref.current?.swipe("right");
  };

  return (
    <TinderCard
      className="h-full w-full overflow-hidden select-none"
      preventSwipe={["up", "down"]}
      onSwipe={(dir) => console.log(dir)}
      onCardLeftScreen={(dir) => {
        if (dir === "left" && props.onLike) {
          props.onLike();
        }
        if (dir === "right" && props.onDislike) {
          props.onDislike();
        }
      }}
      ref={ref}
    >
      <Card
        title={props.title}
        description={props.description}
        image={props.image}
        onLike={handleLike}
        onDislike={handleDislike}
        tags={props.tags}
      />
    </TinderCard>
  );
};
export const TinderRoute: React.FC = () => {
  const { data: cards, like } = useVacancies();

  return (
    <>
      <div className="flex flex-col h-full gap-6">
        <div className="h-full w-full stack">
          {cards?.map((card) => (
            <TinderJobCard
              title={card.title}
              description={card.description}
              image={card.images[0]?.imageUrl}
              key={card.id}
              onLike={like}
              tags={card.tags}
            />
          ))}
        </div>
      </div>
    </>
  );
};
