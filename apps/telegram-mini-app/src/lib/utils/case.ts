import { snakeCase, camelCase } from "change-case";

import type {
  CamelCasedPropertiesDeep,
  SnakeCasedPropertiesDeep,
} from "type-fest";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function convertKeys(obj: any, func: (key: string) => string): any {
  if (Array.isArray(obj)) {
    for (let i = 0; i < obj.length; i++) {
      convertKeys(obj[i], func);
    }
  } else if (typeof obj === "object") {
    for (const key in obj) {
      const newKey = func(key);
      obj[newKey] = obj[key];
      if (key !== newKey) {
        delete obj[key];
      }
      convertKeys(obj[newKey], func);
    }
  }

  return obj;
}
type CaseConvertionObject = Record<string, unknown> | Record<string, unknown>[];

export function toCamelCase<
  T extends CaseConvertionObject = CaseConvertionObject,
>(obj: T): CamelCasedPropertiesDeep<T> {
  return convertKeys(obj, camelCase);
}

export function toSnakeCase<
  T extends CaseConvertionObject = CaseConvertionObject,
>(obj: T): SnakeCasedPropertiesDeep<T> {
  return convertKeys(obj, snakeCase);
}
