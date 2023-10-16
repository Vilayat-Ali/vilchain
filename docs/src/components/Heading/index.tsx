// lib

type Props = {
  text: string;
};

const Heading = ({ text }: Props) => {
  return (
    <div className="w-[100vw] border-0 md:border-l-2 border-gray-300 pl-0 md:pl-4 hover:transition-all hover:border-l-4 md:hover:border-white cursor-pointer">
      <p className="text-white text-3xl md:text-4xl font-bold">{text}</p>
    </div>
  );
};

export default Heading;
