import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: "VilChain",
      social: {
        github: "https://github.com/vilayat-ali/vilchain",
        email: "mailto://vilayatcodemysite@gmail.com",
      },
      sidebar: [
        {
          label: "Concepts",
          items: [
            { label: "Example Guide", link: "/guides/example/" },
            { label: "Example Guide", link: "/guides/example/" },
            { label: "Example Guide", link: "/guides/example/" },
            { label: "Example Guide", link: "/guides/example/" },
          ],
        },
        {
          label: "Documentation",
          items: [
            { label: "Example Guide", link: "/guides/example/" },
            { label: "Example Guide", link: "/guides/example/" },
            { label: "Example Guide", link: "/guides/example/" },
            { label: "Example Guide", link: "/guides/example/" },
          ],
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
      ],
    }),
  ],
});
