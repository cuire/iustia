import { Link, useLocation } from "wouter";

import { Icon, IconType, Queue, Fire } from "$lib/components/icon";

type NavLink = { href: string; label: string; icon: IconType };

const links: NavLink[] = [
  { href: "/", label: "Latest", icon: Queue },
  { href: "/tinder", label: "Following", icon: Fire },
  { href: "/debug/theme", label: "Favorites", icon: Queue },
];

export const Nav: React.FC = () => {
  const [location] = useLocation();

  return (
    <nav className="sticky bottom-0 left-0 z-50 w-full h-16 border-t border-gray-200 bg-base-200 dark:border-gray-600">
      <div className="grid h-full max-w-lg grid-cols-3 mx-auto">
        {links.map((link) => (
          <Link key={link.href} href={link.href}>
            <button
              type="button"
              className="inline-flex flex-col items-center justify-center font-medium px-5 w-full h-full"
            >
              <Icon
                icon={link.icon}
                className="w-5 h-5 mb-1"
                solid={location === link.href}
              />

              <span className="text-sm text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500">
                {link.label}
              </span>
            </button>
          </Link>
        ))}
      </div>
    </nav>
  );
};
