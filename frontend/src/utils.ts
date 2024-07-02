import { Dispatch, MutableRefObject, SetStateAction } from "react";
import { API_URL } from "./constants";

export function setJsonStateFromApi(
  path: string,
  state: Record<string, string>,
  setter: Dispatch<SetStateAction<Record<string, string>>>,
  isLoadingRef: MutableRefObject<boolean>
) {
  if (!isLoadingRef.current && !Object.keys(state).length) {
    isLoadingRef.current = true;
    fetch(`${API_URL}${path}`)
      .then((res) => res.json())
      .then(setter)
      .finally(() => {
        isLoadingRef.current = false;
      });
  }
}
