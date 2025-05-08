
import React, { useState } from 'react';
import Layout from '@/components/layout/Layout';
import { useAuth } from '@/contexts/AuthContext';
import { 
  Card, 
  CardContent, 
  CardHeader, 
  CardTitle 
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { 
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Search, Filter } from 'lucide-react';

// Mock order data
const mockOrders = [
  {
    id: 'ORD-001',
    date: '2023-05-15T10:30:00',
    status: 'delivered',
    items: [
      { id: 1, name: 'Handcrafted Bamboo Chair', quantity: 1, price: 45000 }
    ],
    total: 45000
  },
  {
    id: 'ORD-002',
    date: '2023-05-10T14:22:00',
    status: 'processing',
    items: [
      { id: 2, name: 'Traditional Cameroonian Coffee', quantity: 2, price: 5000 },
      { id: 3, name: 'Handwoven Market Basket', quantity: 1, price: 7500 }
    ],
    total: 17500
  },
  {
    id: 'ORD-003',
    date: '2023-04-28T09:15:00',
    status: 'delivered',
    items: [
      { id: 4, name: 'Organic Shea Butter', quantity: 3, price: 3000 }
    ],
    total: 9000
  }
];

type OrderStatus = 'all' | 'pending' | 'processing' | 'delivered' | 'cancelled';

const Orders = () => {
  const { user, isAuthenticated } = useAuth();
  const [orders] = useState(mockOrders);
  const [searchQuery, setSearchQuery] = useState('');
  const [statusFilter, setStatusFilter] = useState<OrderStatus>('all');

  // Filter orders based on search query and status
  const filteredOrders = orders.filter(order => {
    const matchesSearch = order.id.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesStatus = statusFilter === 'all' || order.status === statusFilter;
    return matchesSearch && matchesStatus;
  });

  // Helper function to get badge color based on status
  const getStatusBadgeColor = (status: string) => {
    switch (status) {
      case 'pending':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'processing':
        return 'bg-blue-100 text-blue-800 border-blue-200';
      case 'delivered':
        return 'bg-green-100 text-green-800 border-green-200';
      case 'cancelled':
        return 'bg-red-100 text-red-800 border-red-200';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  // If not logged in, redirect to login
  if (!isAuthenticated) {
    return (
      <Layout>
        <div className="container mx-auto py-16 px-4">
          <div className="max-w-md mx-auto text-center">
            <h1 className="text-2xl font-bold mb-4">My Orders</h1>
            <p className="mb-6">Please log in to view your orders.</p>
            <Button onClick={() => window.location.href = '/login'}>
              Log In
            </Button>
          </div>
        </div>
      </Layout>
    );
  }

  return (
    <Layout>
      <div className="container mx-auto py-16 px-4">
        <div className="max-w-4xl mx-auto">
          <h1 className="text-2xl font-bold mb-6">My Orders</h1>
          
          {orders.length === 0 ? (
            <Card>
              <CardContent className="p-6">
                <div className="text-center py-8">
                  <h3 className="text-lg font-medium mb-2">No orders yet</h3>
                  <p className="text-gray-500 dark:text-gray-400 mb-4">
                    You haven't made any purchases yet.
                  </p>
                  <Button
                    onClick={() => window.location.href = '/marketplace'}
                    className="bg-cameroon-green hover:bg-cameroon-green/90"
                  >
                    Start Shopping
                  </Button>
                </div>
              </CardContent>
            </Card>
          ) : (
            <>
              {/* Filters */}
              <Card className="mb-6">
                <CardContent className="p-4">
                  <div className="flex flex-col md:flex-row gap-4">
                    <div className="relative flex-1">
                      <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" />
                      <Input
                        type="text"
                        placeholder="Search orders by ID..."
                        className="pl-10"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                      />
                    </div>
                    
                    <div className="w-full md:w-48">
                      <Select value={statusFilter} onValueChange={(value) => setStatusFilter(value as OrderStatus)}>
                        <SelectTrigger>
                          <div className="flex items-center">
                            <Filter className="mr-2 h-4 w-4" />
                            <SelectValue placeholder="Status" />
                          </div>
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="all">All Statuses</SelectItem>
                          <SelectItem value="pending">Pending</SelectItem>
                          <SelectItem value="processing">Processing</SelectItem>
                          <SelectItem value="delivered">Delivered</SelectItem>
                          <SelectItem value="cancelled">Cancelled</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                  </div>
                </CardContent>
              </Card>

              {/* Orders List */}
              {filteredOrders.length === 0 ? (
                <div className="text-center py-10">
                  <p className="text-xl font-medium">No orders found</p>
                  <p className="text-gray-500 mt-2">
                    Try a different search term or filter
                  </p>
                </div>
              ) : (
                <div className="space-y-6">
                  {filteredOrders.map((order) => (
                    <Card key={order.id} className="overflow-hidden">
                      <CardHeader className="bg-gray-50 dark:bg-gray-800 py-4">
                        <div className="flex flex-col md:flex-row justify-between">
                          <div className="flex flex-col md:flex-row md:items-center gap-2 md:gap-6">
                            <CardTitle className="text-base">{order.id}</CardTitle>
                            <span className="text-sm text-gray-500">
                              {new Date(order.date).toLocaleDateString('en-US', {
                                year: 'numeric',
                                month: 'long',
                                day: 'numeric',
                              })}
                            </span>
                          </div>
                          <Badge className={`mt-2 md:mt-0 ${getStatusBadgeColor(order.status)}`}>
                            {order.status.charAt(0).toUpperCase() + order.status.slice(1)}
                          </Badge>
                        </div>
                      </CardHeader>
                      <CardContent className="p-6">
                        <div className="space-y-4">
                          {order.items.map((item) => (
                            <div key={item.id} className="flex justify-between items-center">
                              <div>
                                <p className="font-medium">{item.name}</p>
                                <p className="text-sm text-gray-500">Quantity: {item.quantity}</p>
                              </div>
                              <p className="font-medium">{item.price.toLocaleString()} FCFA</p>
                            </div>
                          ))}
                          <div className="pt-4 border-t border-gray-200 dark:border-gray-700">
                            <div className="flex justify-between">
                              <span className="font-bold">Total</span>
                              <span className="font-bold">{order.total.toLocaleString()} FCFA</span>
                            </div>
                          </div>
                        </div>
                        <div className="mt-6">
                          <Button variant="outline" className="w-full md:w-auto">
                            View Order Details
                          </Button>
                        </div>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              )}
            </>
          )}
        </div>
      </div>
    </Layout>
  );
};

export default Orders;
