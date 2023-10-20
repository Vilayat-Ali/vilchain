type Props = {
  title: string;
  heading: string[];
  data: string[][];
};

const Table = ({ title, heading, data }: Props) => {
  return (
    <div className="rounded-sm border border-stroke bg-white px-5 pt-6 pb-2.5 shadow-default dark:border-strokedark dark:bg-boxdark sm:px-7.5 xl:pb-1">
      <h4 className="mb-6 text-xl font-semibold text-black dark:text-white">
        {title}
      </h4>

      <div className="flex flex-col">
        <div className="grid grid-cols-3 rounded-sm bg-gray-2 dark:bg-meta-4 sm:grid-cols-5">
          {heading.map((header_text: string) => (
            <div className="p-2.5 xl:p-5">
              <h5
                className="text-sm font-medium uppercase xsm:text-base"
                key={header_text}
              >
                {header_text}
              </h5>
            </div>
          ))}
        </div>

        {data.map((row: string[]) => (
          <div className="grid grid-cols-3 border-b border-stroke dark:border-strokedark sm:grid-cols-5">
            {row.map((row_data: string) => (
              <div
                className="flex items-center justify-center p-2.5 xl:p-5"
                key={row_data}
              >
                <p className="text-black dark:text-white text-center">
                  {row_data}
                </p>
              </div>
            ))}
          </div>
        ))}
      </div>
    </div>
  );
};

export default Table;
