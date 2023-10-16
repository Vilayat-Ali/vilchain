// lib
import Link from "next/link";

// components
import Heading from "@/components/Heading";

const page = () => {
  return (
    <div>
      {/* Hero Section */}
      <div
        className="relative overflow-hidden bg-cover bg-no-repeat"
        style={{
          backgroundPosition: "50%",
          backgroundImage:
            "url('https://mdbcdn.b-cdn.net/img/new/slides/146.webp')",
          height: "500px",
        }}
      >
        <div className="absolute top-0 right-0 bottom-0 left-0 h-full w-full overflow-hidden bg-[hsla(0,0%,0%,0.75)] bg-fixed">
          <div className="flex h-full items-center justify-center">
            <div className="px-6 text-center text-gray-300 md:px-12">
              <h1 className="mt-2 mb-16 text-4xl font-bold tracking-tight md:text-6xl xl:text-7xl">
                Discover the Power of Blockchain
                <br />
                <span className="text-xl md:text-xl xl:text-2xl text-gray-400">
                  A Product of a{" "}
                  <Link href="https://www.linkedin.com/in/syed-vilayat-ali-rizvi/">
                    <i className="underline">Jerk</i>
                  </Link>
                  &apos;s mind
                </span>
              </h1>
              <button
                type="button"
                className="rounded border-2 border-neutral-50 px-[46px] pt-[14px] pb-[12px] text-sm font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-100 hover:bg-neutral-100 hover:bg-opacity-10 hover:text-neutral-100 focus:border-neutral-100 focus:text-neutral-100 focus:outline-none focus:ring-0 active:border-neutral-200 active:text-neutral-200"
                data-te-ripple-init
                data-te-ripple-color="light"
              >
                Witness the Future of Blockchain
              </button>
            </div>
          </div>
        </div>
      </div>
      {/* Hero Section */}

      <main className="px-0 md:px-[6vw]">
        {/* Testimonial */}
        <section className="overflow-x-hidden rounded-md px-0 py-6 text-center shadow-lg md:p-12 md:text-left">
          <Heading text="Features" />
        </section>
        {/* Testimonial */}

        {/* Testimonial */}
        <section className="overflow-x-hidden rounded-md px-0 py-6 text-center shadow-lg md:p-12 md:text-left">
          <Heading text="Testimonials" />
          <div className="flex justify-center">
            <div className="max-w-3xl">
              {[...Array(3)].map((testimonial: any) => (
                <div
                  className="block rounded-lg bg-white w-[90%] mx-auto my-4 p-6 shadow-lg dark:bg-neutral-800 dark:shadow-black/20"
                  key={testimonial}
                >
                  <div className="md:flex md:flex-row">
                    <div className="mx-auto mb-6 flex w-36 items-center justify-center md:mx-0 md:w-96 lg:mb-0">
                      <img
                        src="https://tecdn.b-cdn.net/img/Photos/Avatars/img%20%2810%29.jpg"
                        className="rounded-full shadow-md dark:shadow-black/30"
                        alt="woman avatar"
                      />
                    </div>
                    <div className="md:ml-6">
                      <p className="mb-6 font-light text-neutral-500 dark:text-neutral-300">
                        Lorem ipsum dolor, sit amet consectetur adipisicing
                        elit. Id quam sapiente molestiae numquam quas,
                        voluptates omnis nulla ea odio quia similique corrupti
                        magnam.
                      </p>
                      <p className="mb-2 text-xl font-semibold text-neutral-800 dark:text-neutral-200">
                        Anna Smith
                      </p>
                      <p className="mb-0 font-semibold text-neutral-500 dark:text-neutral-400">
                        Product manager
                      </p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </section>
        {/* Testimonial */}
      </main>
    </div>
  );
};

export default page;
