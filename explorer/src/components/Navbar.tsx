// lib
import {
  useState,
  useCallback,
  type SetStateAction,
  type Dispatch,
  useEffect,
} from "react";
import { Link } from "react-router-dom";

// icons
import { MdMenuOpen, MdMenu } from "react-icons/md";

// assets
import Logo from "../assets/react.svg";

interface MenuItem {
  label: string;
  href: string;
}

const MenuList: MenuItem[] = [
  {
    label: "Home",
    href: "/",
  },
  {
    label: "Explorer",
    href: "/explorer",
  },
  {
    label: "Health-Check",
    href: "/health-check",
  },
  {
    label: "About",
    href: "/about",
  },
];

const Navbar = () => {
  const [isMenuOpen, setMenuState]: [
    boolean,
    Dispatch<SetStateAction<boolean>>
  ] = useState<boolean>(false);

  const toggleMenu = useCallback(() => setMenuState(!isMenuOpen), [isMenuOpen]);

  const isMenuLinkActive = (url: string) => window.location.pathname === url;

  return (
    <>
      <nav className="flex flex-row justify-between items-center w-[100vw] h-[7vh] px-[4vw] md:px-[2vw] bg-black">
        {/* Logo */}
        <div className="w-[15vw]">
          <img className="" src={Logo} alt="vilchain-logo" />
        </div>
        {/* Logo */}

        {/* Menu Desktop Items */}
        <div className="w-[30vw] hidden md:flex flex-row justify-evenly items-center text-gray-400">
          {MenuList.map((menu: MenuItem) => (
            <Link to={menu.href} key={menu.href}>
              <div
                className={`hover:text-underline ${
                  isMenuLinkActive(menu.href) && "text-white"
                } hover:text-white`}
              >
                {menu.label}
              </div>
            </Link>
          ))}
        </div>
        {/* Menu Desktop Items */}

        {/* Menu Mobile Menu Button */}
        <div className="md:hidden text-4xl text-gray-800" onClick={toggleMenu}>
          {!isMenuOpen ? <MdMenu /> : <MdMenuOpen />}
        </div>
        {/* Menu Mobile Menu Button */}
      </nav>

      {/* Menu Mobile Menu */}
      {isMenuOpen && (
        <div className="bg-gray-200">
          {MenuList.map((menu: MenuItem) => (
            <div className="p-[4vw]" key={menu.href}>
              <Link to={menu.href}>
                <div className="text-md">{menu.label}</div>
              </Link>
            </div>
          ))}
        </div>
      )}
      {/* Menu Mobile Menu */}
    </>
  );
};

export default Navbar;
