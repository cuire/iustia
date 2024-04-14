import useSWR from "swr";

import { toCamelCase } from "$lib/utils/case";
import { retrieveLaunchParams } from "@tma.js/sdk";

const backendUrl = new URL(import.meta.env.VITE_IUSTIA_BACKEND_URL);

const PAGE_SIZE = 30;

type ApiResult<T> = {
  count: number;
  next: string | null;
  previous: string | null;
  results: T[];
};

export type Tag = {
  tag: string;
  slug: string;
};

export type Vacancy = {
  id: number;
  title: string;
  description: string;
  company: number;
  location: string | null;
  job_type: "full_time" | "part_time" | "contract" | "internship";
  is_active: boolean;
  images: { image_url: string }[];
  tags: Tag[];
};

export function useVacancies() {
  const headers = useAuthHeaders();

  const fetcher = (url: string, method = "GET") =>
    fetch(backendUrl.toString() + url, {
      method,
      headers: {
        ...headers,
        "Content-Type": "application/json",
      },
    }).then((res) => res.json());

  const { data, mutate } = useSWR<ApiResult<Vacancy>>(
    `jobs/?page=1&page_size=${PAGE_SIZE}`,
    fetcher,
  );

  function like(): void {
    // remove top item from the list
    if (!data) return;

    const vacancies = data.results;
    // mutate pop last item
    const vacancy = vacancies[vacancies.length - 1];

    mutate({ ...data, results: vacancies.slice(0, -1) });

    console.log("Liked", data);

    fetcher(`jobs/${vacancy.id}/apply/`, "POST");
  }

  const vacancies = data?.results ?? [];

  return {
    data: toCamelCase(vacancies),
    like,
  } as const;
}

export const useAuthHeaders = () => {
  const { initDataRaw } = retrieveLaunchParams();

  const headers = {
    Authorization: `tma ${initDataRaw}`,
  };

  return headers;
};
