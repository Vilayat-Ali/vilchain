// lib
import moment from "moment";

// assets
import Logo from "../assets/react.svg";

// icons
import { BsGithub } from "react-icons/bs";
import { MdEmail } from "react-icons/md";
import { SlSocialLinkedin } from "react-icons/sl";

const Footer = () => {
  return (
    <div className="w-[100vw] py-[1vw] bg-black flex flex-row justify-between items-center">
      <div className="flex flex-row justify-start items-center p-2">
        <img src={Logo} alt="vilchain-logo" />
        <p className="text-white ml-2">Logo</p>
      </div>

      <div className="p-2">
        <p className="text-white ml-2">
          All Rights Reserved {moment().format("YYYY")}
        </p>
      </div>

      <div className="flex flex-row justify-evenly items-center p-2 w-[10vw]">
        <a
          className="text-white text-2xl"
          href="https://www.github.com/vilayat-ali"
        >
          <BsGithub />
        </a>

        <a className="text-white text-2xl">
          <SlSocialLinkedin />
        </a>

        <a className="text-white text-2xl">
          <MdEmail />
        </a>
      </div>
    </div>
  );
};

export default Footer;
