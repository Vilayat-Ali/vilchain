// lib
import moment from "moment";
import Link from "next/link";
import { SocialLink, socialLinks } from "../Navbar/MenuList";

const index = () => {
  return (
    <div className="flex flex-col md:flex-row justify-between items-center px-4 py-4 bg-black">
      {/* Footer Text */}
      <div className="text-white text-center md:text-justify">
        Built by{" "}
        <a
          className="text-gray-100 md:text-gray-300 hover:text-white italic underline"
          href="https://www.github.com/vilayat-ali"
        >
          Vilayat
        </a>{" "}
        with ❤️ and{" "}
        <a
          className="underline"
          href="https://www.rust-lang.org/https://www.rust-lang.org/"
        >
          Rust
        </a>{" "}
        🦀. All Rights Reserved. {moment().format("YYYY")}
      </div>
      {/* Footer Text */}

      {/* Footer Social */}
      <div className="flex flex-row justify-between items-center mt-[6vw] md:mt-0">
        {socialLinks.map((social: SocialLink) => (
          <Link href={social.href} key={social.href}>
            <div className="text-white hover:text-white mx-2 text-3xl">
              {social.icon({})}
            </div>
          </Link>
        ))}
      </div>
      {/* Footer Social */}
    </div>
  );
};

export default index;
