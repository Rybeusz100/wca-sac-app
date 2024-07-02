import { createContext } from "react";

export type WcaEvents = Record<string, string>;

export const WcaEventsContext = createContext<WcaEvents>({});
