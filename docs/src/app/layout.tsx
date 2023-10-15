// lib
import type { Metadata } from "next";
import { Inter } from "next/font/google";

// components
import Navbar from "@/components/Navbar";

// styles
import "./styles/globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "VilChain | A miniature blockchain by a jerk!",
  description:
    "Dive into the world of blockchain innovation with my Rust-powered miniature blockchain project. Explore comprehensive documentation and a captivating project showcase. Uncover the future of decentralized technology.",
  keywords: [
    "blockchain",
    "rust",
    "next.js",
    "projects",
    "Syed Vilayat Ali Rizvi",
    "systems-engineering",
    "rustacean",
  ],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Navbar />
        {children}
      </body>
    </html>
  );
}
