// lib
import type { IconType } from "react-icons";
import { MdHomeFilled } from "react-icons/md";
import { BsDiscord, BsGithub, BsMailbox } from "react-icons/bs";
import { SiGoogledocs } from "react-icons/si";
import { FaDev } from "react-icons/fa";

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
    title: "Docs",
    href: "/docs",
    icon: SiGoogledocs,
  },
  {
    title: "Blogs",
    href: "/blogs",
    icon: FaDev,
  },
  {
    title: "Home",
    href: "/g",
    icon: BsDiscord,
  },
  {
    title: "Mail",
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
