// components
import Card from "../../components/Card";
import ChartOne from "../../components/ChartOne.js";
import ChartTwo from "../../components/ChartTwo.js";
import TableOne from "../../components/Table.js";

const ECommerce = () => {
  return (
    <>
      <div className="grid grid-cols-1 gap-4 md:grid-cols-2 md:gap-6 xl:grid-cols-4 2xl:gap-7.5">
        <Card title="Example" value="12.33" changePercentage={2.3} />
        <Card title="Example" value="12.33" changePercentage={2.3} />
        <Card title="Example" value="12.33" changePercentage={2.3} />
        <Card title="Example" value="12.33" changePercentage={2.3} />
      </div>

      <div className="mt-4 grid grid-cols-12 gap-4 md:mt-6 md:gap-6 2xl:mt-7.5 2xl:gap-7.5">
        <ChartOne />
        <ChartTwo />
        <div className="col-span-12 xl:col-span-8">
          <TableOne
            title="Example"
            heading={["name", "age", "sex", "age", "sex"]}
            data={[
              ["Ali", "20", "MALE", "20", "MALE"],
              ["Ali", "20", "MALE", "20", "MALE"],
              ["Ali", "20", "MALE", "20", "MALE"],
              ["Ali", "20", "MALE", "20", "MALE"],
            ]}
          />
        </div>
      </div>
    </>
  );
};

export default ECommerce;
