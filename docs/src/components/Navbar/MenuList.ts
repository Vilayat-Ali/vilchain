// lib
import {type IconType} from "react-icons";

// icons
import {AiOutlineGithub, AiFillLinkedin} from "react-icons/ai";
import {SiGmail} from "react-icons/si";

export interface MenuItem {
    label: string;
    href: string;
}

export const menuList: MenuItem[] = [
    {
        label: 'Home',
        href: '/',
    },
    {
        label: 'Docs',
        href: '/docs',
    },
    {
        label: 'Blogs',
        href: '/blogs',
    },
    {
        label: 'Meet the Jerk',
        href: '/jerk',
    },
    {
        label: '',
        href: '',
    }
];

export interface SocialLink {
    icon: IconType;
    href: string;
}

export const socialLinks: SocialLink[] = [
    {
        icon: AiOutlineGithub,
        href: "https://www.github.com/vilayat-ali/vilchain"
    },
    {
        icon: AiFillLinkedin,
        href: "https://www.linkedin.com/in/syed-vilayat-ali-rizvi/"
    },
    {
        icon: SiGmail,
        href: "mailto://vilayatcodemysite@gmail.com"
    },
]