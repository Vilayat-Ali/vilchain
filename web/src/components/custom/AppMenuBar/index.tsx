"use client";

// lib
import { useState, type SetStateAction, type Dispatch } from "react";
import Link from "next/link";

// components
import { Card } from "@/components/ui/card";

// menu items
import MenuList, { type MenuItem } from "./MenuList";

const AppMenuBar = () => {
  const [currentHoveredMenuItem, setHoveredMenuItem]: [
    number | undefined,
    Dispatch<SetStateAction<number | undefined>>
  ] = useState<number | undefined>();

  return (
    <Card className="absolute h-[33.3vh] md:h-fit md:w-fit overflow-scroll scroll-m-0 flex flex-col md:flex-row transition-transform top-[5vh] right-[2vw] md:top-[90%] md:right-0 md:left-1/2 md:transform md:-translate-x-1/2 z-20 p-2 md:p-4 shadow-xl rounded-full">
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
          <Link key={item.href} href={item.href}>
            <Card
              className={`z-50 text-xl transition-all p-2 md:p-4 rounded-full my-2 md:mx-2 md:my-0 cursor-pointer hover:scale-150 hover:transform hover:-translate-y-10 ${
                index === leftItemIdx && "scale-125 translate -translate-y-5"
              }
            ${index === currentHoveredMenuItem && "mx-4 bg-black text-white"}
            ${index === rightItemIdx && "scale-125 translate -translate-y-5"}`}
              onMouseOver={() => setHoveredMenuItem(index)}
              onMouseOut={() => setHoveredMenuItem(undefined)}
            >
              {item.icon({})}
            </Card>
          </Link>
        );
      })}
    </Card>
  );
};

export default AppMenuBar;
