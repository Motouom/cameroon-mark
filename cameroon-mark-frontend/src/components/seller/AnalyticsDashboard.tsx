
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

// Mock data for demonstration
const salesData = [
  { month: 'Jan', sales: 120000 },
  { month: 'Feb', sales: 180000 },
  { month: 'Mar', sales: 250000 },
  { month: 'Apr', sales: 200000 },
  { month: 'May', sales: 300000 },
  { month: 'Jun', sales: 320000 },
];

// Mock summary data
const summaryData = {
  totalSales: 1370000,
  totalOrders: 142,
  averageOrderValue: 9648,
  topSellingProduct: 'Traditional Cameroonian Coffee'
};

const CustomTooltip = ({ active, payload, label }: any) => {
  if (active && payload && payload.length) {
    return (
      <div className="bg-white p-3 shadow-md border rounded">
        <p className="font-medium">{`${label}`}</p>
        <p className="text-cameroon-green">{`${payload[0].value.toLocaleString()} FCFA`}</p>
      </div>
    );
  }

  return null;
};

const AnalyticsDashboard = () => {
  return (
    <div className="space-y-6">
      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">Total Sales</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{summaryData.totalSales.toLocaleString()} FCFA</p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">Total Orders</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{summaryData.totalOrders}</p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">Average Order Value</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{summaryData.averageOrderValue.toLocaleString()} FCFA</p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">Top Selling Product</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-xl font-bold truncate">{summaryData.topSellingProduct}</p>
          </CardContent>
        </Card>
      </div>
      
      {/* Sales Chart */}
      <Card>
        <CardHeader>
          <CardTitle>Monthly Sales (FCFA)</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart
                data={salesData}
                margin={{
                  top: 5,
                  right: 30,
                  left: 20,
                  bottom: 5,
                }}
              >
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="month" />
                <YAxis 
                  tickFormatter={(value) => `${(value / 1000)}k`} 
                />
                <Tooltip content={<CustomTooltip />} />
                <Bar dataKey="sales" fill="#2D5F4E" />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default AnalyticsDashboard;
