"use client";

// lib
import {
  useState,
  useCallback,
  useEffect,
  type SetStateAction,
  type Dispatch,
} from "react";

// components
import { Card } from "@/components/ui/card";

// menu items
import MenuList, { type MenuItem } from "./MenuList";

const AppMenuBar = () => {
  const [currentHoveredMenuItem, setHoveredMenuItem]: [
    number | undefined,
    Dispatch<SetStateAction<number | undefined>>
  ] = useState<number | undefined>();

  useEffect(() => {
    console.log(currentHoveredMenuItem);
  }, [currentHoveredMenuItem]);

  return (
    <Card className="absolute h-[33.5vh] md:h-full overflow-scroll scroll-m-0 flex flex-col md:flex-row transition-transform top-[5vh] right-[2vw] md:top-0 md:right-0 md:bottom-[5vh] md:left-1/2 md:transform md:-translate-x-1/2 z-50 p-2 md:p-4 shadow rounded-full">
      {MenuList.map((item: MenuItem, index: number) => {
        const leftItemIdx: number | undefined =
          currentHoveredMenuItem === 0
            ? undefined
            : (currentHoveredMenuItem as number) - 1;
        const rightItemIdx: number | undefined =
          currentHoveredMenuItem === MenuList.length - 1
            ? undefined
            : (currentHoveredMenuItem as number) + 1;

        return (
          <Card
            className={`text-xl transition-all p-2 md:p-4 rounded-full my-2 md:mx-2 md:my-0 cursor-pointer hover:scale-150 hover:transform hover:-translate-y-10 ${
              index === leftItemIdx && "scale-125 translate -translate-y-5"
            }
            ${index === currentHoveredMenuItem && "mx-4 bg-black text-white"}
            ${index === rightItemIdx && "scale-125 translate -translate-y-5"}`}
            onMouseOver={() => setHoveredMenuItem(index)}
            onMouseOut={() => setHoveredMenuItem(undefined)}
            key={item.href}
          >
            {item.icon({})}
          </Card>
        );
      })}
    </Card>
  );
};

export default AppMenuBar;
