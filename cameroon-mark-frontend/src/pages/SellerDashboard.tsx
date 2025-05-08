
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Button } from '@/components/ui/button';
import Layout from '@/components/layout/Layout';
import { useAuth } from '@/contexts/AuthContext';
import { Plus, PackageSearch, MessageSquare, BarChart3 } from 'lucide-react';
import ProductsList from '@/components/seller/ProductsList';
import OrdersList from '@/components/seller/OrdersList';
import MessagesList from '@/components/seller/MessagesList';
import AnalyticsDashboard from '@/components/seller/AnalyticsDashboard';

const SellerDashboard = () => {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("products");
  
  // Redirect to login if not authenticated or not a seller
  if (!user) {
    return (
      <Layout>
        <div className="container mx-auto py-16 px-4">
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-4">Seller Dashboard</h1>
            <p className="mb-6">You need to be logged in as a seller to access this page.</p>
            <Button onClick={() => navigate('/login')}>Login</Button>
          </div>
        </div>
      </Layout>
    );
  }
  
  if (user.role !== 'seller') {
    return (
      <Layout>
        <div className="container mx-auto py-16 px-4">
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-4">Seller Dashboard</h1>
            <p className="mb-6">You need a seller account to access this page.</p>
            <Button onClick={() => navigate('/')}>Back to Home</Button>
          </div>
        </div>
      </Layout>
    );
  }

  return (
    <Layout>
      <div className="container mx-auto py-8 px-4">
        <div className="flex flex-col md:flex-row justify-between items-center mb-6">
          <h1 className="text-2xl font-bold mb-4 md:mb-0">Seller Dashboard</h1>
          <Button 
            onClick={() => navigate('/seller/product/new')}
            className="flex items-center"
          >
            <Plus className="mr-2 h-4 w-4" /> Add New Product
          </Button>
        </div>
        
        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <TabsList className="mb-6">
            <TabsTrigger value="products" className="flex items-center">
              <PackageSearch className="mr-2 h-4 w-4" /> Products
            </TabsTrigger>
            <TabsTrigger value="orders" className="flex items-center">
              <BarChart3 className="mr-2 h-4 w-4" /> Orders
            </TabsTrigger>
            <TabsTrigger value="messages" className="flex items-center">
              <MessageSquare className="mr-2 h-4 w-4" /> Messages
            </TabsTrigger>
            <TabsTrigger value="analytics" className="flex items-center">
              <BarChart3 className="mr-2 h-4 w-4" /> Analytics
            </TabsTrigger>
          </TabsList>
          
          <TabsContent value="products">
            <ProductsList />
          </TabsContent>
          
          <TabsContent value="orders">
            <OrdersList />
          </TabsContent>
          
          <TabsContent value="messages">
            <MessagesList />
          </TabsContent>
          
          <TabsContent value="analytics">
            <AnalyticsDashboard />
          </TabsContent>
        </Tabs>
      </div>
    </Layout>
  );
};

export default SellerDashboard;
