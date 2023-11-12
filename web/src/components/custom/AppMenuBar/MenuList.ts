// lib
import type { IconType } from "react-icons";
import { MdHomeFilled } from "react-icons/md";
import { BsDiscord, BsGithub, BsMailbox } from "react-icons/bs";

export interface MenuItem {
  title: string;
  href: string;
  icon: IconType;
}

const MenuList: MenuItem[] = [
  {
    title: "Home",
    href: "/",
    icon: MdHomeFilled,
  },
  {
    title: "Home",
    href: "/a",
    icon: MdHomeFilled,
  },
  {
    title: "Home",
    href: "/s",
    icon: MdHomeFilled,
  },
  {
    title: "Home",
    href: "/d",
    icon: MdHomeFilled,
  },
  {
    title: "Home",
    href: "/f",
    icon: MdHomeFilled,
  },
  {
    title: "Home",
    href: "/g",
    icon: BsDiscord,
  },
  {
    title: "Home",
    href: "mailto://vilayatcodemysite@gmail.com",
    icon: BsMailbox,
  },
  {
    title: "Github",
    href: "https://www.github.com/vilayat-ali/vilchain",
    icon: BsGithub,
  },
];

export default MenuList;
