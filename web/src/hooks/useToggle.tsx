// lib
import {
  useState,
  useCallback,
  type Dispatch,
  type SetStateAction,
} from "react";

const useToggle = (
  initialState: boolean = false
): [boolean, () => void, Dispatch<SetStateAction<boolean>>] => {
  const [componentState, setComponentState]: [
    boolean,
    Dispatch<SetStateAction<boolean>>
  ] = useState<boolean>(initialState);

  const Toggle = useCallback(
    () => setComponentState(!componentState),
    [componentState]
  );

  return [componentState, Toggle, setComponentState] as const;
};

export default useToggle;
