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

  const fetcher = (url: string) =>
    fetch(backendUrl.toString() + url, {
      headers: {
        ...headers,
        "Content-Type": "application/json",
      },
    }).then((res) => res.json());

  const { data } = useSWR<ApiResult<Vacancy>>(
    `jobs/?page=1&page_size=${PAGE_SIZE}`,
    fetcher,
  );

  function like(): void {
    // TODO: implement like
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
