// lib
import {
  useState,
  useCallback,
  type SetStateAction,
  type Dispatch,
} from "react";

const useToggle = (
  initialState: boolean = false
): readonly [boolean, () => void, Dispatch<SetStateAction<boolean>>] => {
  const [componentState, setComponentState]: [
    boolean,
    Dispatch<SetStateAction<boolean>>
  ] = useState<boolean>(initialState);

  const toggle = useCallback(
    () => setComponentState(!componentState),
    [componentState]
  );

  return [componentState, toggle, setComponentState] as const;
};

export default useToggle;
