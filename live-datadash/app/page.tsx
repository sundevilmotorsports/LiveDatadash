import Image from "next/image";
import Greet from './greet'
import NumberTest from "./number";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          Get started by editing&nbsp;
            <a href = "https://duckduckgo.com/?q=mass+of+sun&ia=web" target="_blank">
              <code className="font-mono font-bold">Ur mom</code>
            </a>
        </p>
        
        <div className="fixed bottom-0 left-0 flex h-48 w-full items-end justify-center bg-gradient-to-t from-white via-white dark:from-black dark:via-black lg:static lg:h-auto lg:w-auto lg:bg-none">
          <a
            className="pointer-events-none flex place-items-center gap-2 p-8 lg:pointer-events-auto lg:p-0"
            href="https://github.com/sundevilmotorsports/LiveDatadash"
            target="_blank"
            rel="noopener noreferrer"
          >
            By{" SDM24 DAQ"}
            <Image
              src="/fsae.svg"
              alt="DAQ'ed"
              className="DAQ"
              width={100}
              height={24}
              priority
            />
          </a>
        </div>
      </div>

      <div className="relative flex place-items-center before:absolute">
        <Image
          className="Big Playa"
          src="/kaden.svg"
          alt="A handsome fellow"
          width={300}
          height={300}
        />
        <NumberTest />
      </div>

      <div className="mb-32 grid text-center lg:max-w-5xl lg:w-full lg:mb-0 lg:grid-cols-4 lg:text-center">
      <a
          href="https://www.asu.edu/"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            me when whoa{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <Image
            className="Josh dab"
            src = "./josh.svg"
            alt = ":P"
            width = {200}
            height = {200}
          />
        </a>

        <a
          href="https://www.asu.edu/"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Chick fil a BBQ Enthusiest{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <Image
            className="Even + Dog"
            src = "./evan.svg"
            alt = "Dog"
            width = {200}
            height = {200}
          />
        </a>

        <a
          href="https://ryan-leigh.com/"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Git Stalker{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <Image
            className="Ryan"
            src = "./ryan.svg"
            alt = "Knows all your projects"
            width = {200}
            height = {200}
          />
        </a>

        <a
          href="https://www.asu.edu/"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Sanest Rust Dev{""}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <Image
            className="Smaller DAQ"
            src = "./dens.svg"
            alt = ":P"
            width = {200}
            height = {200}
          />
        </a>
      </div>
    </main>
  );
}
