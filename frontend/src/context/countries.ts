import { createContext } from "react";

export type Countries = Record<string, string>;

export const CountriesContext = createContext<Countries>({});
