import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  Cell,
  ResponsiveContainer,
} from "recharts";

export default function Graph({ data }) {
  const pastelColors = [
    "#FAD9E0", // Pastel Pink
    "#FFDAB9", // Peach Puff
    "#BFD8B8", // Mint
    "#CDB4DB", // Lavender
    "#BFE7FF", // Baby Blue
    "#FFF5BA", // Pastel Yellow
    "#C7F9CC", // Pale Green
    "#FFD1C1", // Light Coral
    "#AEC6CF", // Powder Blue
    "#E6DAF8", // Lilac
    "#FDEBD9", // Pale Peach
    "#D1E8E2", // Seafoam
    "#F7C6C7", // Blush
    "#BFD7EA", // Sky
    "#EFD6FF", // Mauve
    "#FFD8A9", // Apricot
    "#E8F6E7", // Tea Green
    "#F0FFF4", // Honeydew
    "#C7CEEA", // Periwinkle
    "#F6E0B5", // Butter
    "#D6F5F0", // Aqua Mint
    "#F2E9E4", // Ivory Rose
    "#FBE8D3", // Soft Apricot
    "#DAD2E6", // Pale Lavender Gray
    "#E3F3FF", // Misty Blue
  ];

  const getPastelColor = (index) => {
    const safeIndex = index % pastelColors.length;
    return pastelColors[safeIndex];
  };

  return (
    <div className="backdrop-blur-md rounded-xl border border-gray-800 p-5 shadow-lg shadow-emerald-500/5 h-fit">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg text-emerald-300 flex items-center">
          <span className="w-2 h-2 bg-emerald-300 rounded-full mr-2"></span>
          DATA VISUALIZATION
        </h3>
        <div className="flex space-x-2">
          <button className="text-xs px-3 py-1 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
            3D
          </button>
          <button className="text-xs px-3 py-1 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
            HEATMAP
          </button>
          <button className="text-xs px-3 py-1 bg-indigo-500 text-gray-900 rounded-lg">
            GRAPH
          </button>
        </div>
      </div>
      <div className="h-100  bg-gray-900/50 rounded-lg border border-gray-700 flex items-center justify-center">
        <div className="flex flex-col items-center justify-center text-center w-full h-full">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart
              width={500}
              height={300}
              data={data}
              margin={{
                top: 20,
                right: 30,
                left: 20,
                bottom: 5,
              }}
            >
              <XAxis dataKey="name" fontSize={12} />
              <YAxis unit="ms" />
              <Tooltip />
              <Bar
                dataKey="duration"
                fill="#8884d8"
                label={{ position: "top", fontSize: 13 }}
                barSize={30}
                maxBarSize={50}
              >
                {data.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={getPastelColor(index)} />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  );
}
