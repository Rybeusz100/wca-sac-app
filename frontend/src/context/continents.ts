import { createContext } from "react";

export type Continents = Record<string, string>;

export const ContinentsContext = createContext<Continents>({});
