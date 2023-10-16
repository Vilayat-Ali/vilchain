"use client";

// lib
import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";

// hooks
import useToggle from "@/hooks/useToggle";

// items
import {
  menuList,
  socialLinks,
  type MenuItem,
  type SocialLink,
} from "./MenuList";

// icons
import { MdMenu, MdMenuOpen } from "react-icons/md";

const Navbar = () => {
  const pathname: string = usePathname();
  const [isMenuOpen, toggleMenuState] = useToggle(false);

  return (
    <>
      {/* Nav */}
      <nav className="flex flex-row justify-between items-center px-4 py-4 shadow bg-black">
        {/* Logo */}
        <div className="flex flex-row justify-between items-center">
          <Image
            className=""
            src="/vite.svg"
            width={40}
            height={40}
            alt="vilchain-logo"
          />
          <p className="hidden md:block text-2xl">VilChain</p>
        </div>
        {/* Logo */}

        {/* Desktop Menu */}
        <div className="hidden md:flex flex-row justify-between items-center">
          {/* Menu Items */}
          {menuList.map((menu: MenuItem) => (
            <Link href={menu.href} key={menu.href}>
              <div
                className={`${
                  pathname === menu.href ? "text-white" : "text-gray-400"
                } hover:text-white mx-4`}
              >
                {menu.label}
              </div>
            </Link>
          ))}
          {/* Menu Items */}
        </div>
        {/* Desktop Menu */}

        {/* Social Links */}
        <div className="hidden md:flex flex-row justify-between items-center">
          {/* Menu Items */}
          {socialLinks.map((social: SocialLink) => (
            <Link href={social.href} key={social.href}>
              <div className="text-white hover:text-white mx-2 text-3xl">
                {social.icon({})}
              </div>
            </Link>
          ))}
          {/* Menu Items */}
        </div>
        {/* Social Links */}

        {/* Mobile Menu Icon */}
        <div
          className="block md:hidden text-white text-4xl"
          onClick={toggleMenuState}
        >
          {!isMenuOpen ? <MdMenu /> : <MdMenuOpen />}
        </div>
        {/* Mobile Menu Icon */}
      </nav>
      {/* Nav */}

      {/* Mobile Menu */}
      {isMenuOpen && (
        <div
          className={`block md:hidden absolute top-[8.25vh] left-0 w-[100vw] z-50 bg-white text-md`}
        >
          {menuList.map((menu: MenuItem) => (
            <Link href={menu.href}>
              <div
                className={`${
                  pathname === menu.href ? "text-black" : "text-gray-600"
                } hover:text-black mx-4 p-2`}
              >
                {menu.label}
              </div>
            </Link>
          ))}
        </div>
      )}
      {/* Mobile Menu */}
    </>
  );
};

export default Navbar;
