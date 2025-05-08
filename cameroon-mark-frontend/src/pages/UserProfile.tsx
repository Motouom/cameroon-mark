
import React, { useState } from 'react';
import Layout from '@/components/layout/Layout';
import { useAuth } from '@/contexts/AuthContext';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useToast } from '@/components/ui/use-toast';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { User, Package, CreditCard, Eye, EyeOff } from 'lucide-react';

const UserProfile = () => {
  const { user } = useAuth();
  const { toast } = useToast();
  const [activeTab, setActiveTab] = useState('profile');
  
  // Profile state
  const [name, setName] = useState(user?.name || '');
  const [email, setEmail] = useState(user?.email || '');
  const [phone, setPhone] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Password state
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  
  const handleProfileUpdate = (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    
    // Simulate profile update
    setTimeout(() => {
      toast({
        title: "Profile updated",
        description: "Your profile information has been updated successfully.",
      });
      setIsSubmitting(false);
    }, 1000);
  };
  
  const handlePasswordUpdate = (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    
    // Check if passwords match
    if (newPassword !== confirmPassword) {
      toast({
        title: "Passwords don't match",
        description: "New password and confirm password must match.",
        variant: "destructive",
      });
      setIsSubmitting(false);
      return;
    }
    
    // Simulate password update
    setTimeout(() => {
      toast({
        title: "Password updated",
        description: "Your password has been updated successfully.",
      });
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');
      setIsSubmitting(false);
    }, 1000);
  };

  return (
    <Layout>
      <div className="container mx-auto py-16 px-4">
        <div className="max-w-6xl mx-auto">
          <div className="flex flex-col md:flex-row gap-8">
            {/* Sidebar */}
            <div className="w-full md:w-80 flex-shrink-0">
              <Card className="mb-6">
                <CardContent className="p-6 flex flex-col items-center text-center">
                  <Avatar className="h-24 w-24 mb-4">
                    <AvatarImage src="https://github.com/shadcn.png" alt={user?.name} />
                    <AvatarFallback>{user?.name?.substring(0, 2).toUpperCase()}</AvatarFallback>
                  </Avatar>
                  <h2 className="text-xl font-bold">{user?.name}</h2>
                  <p className="text-gray-500 dark:text-gray-400">{user?.email}</p>
                  <div className="mt-4 px-3 py-1 bg-cameroon-green/10 text-cameroon-green rounded-full text-sm">
                    {user?.role === 'seller' ? 'Seller Account' : 'Buyer Account'}
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardContent className="p-0">
                  <div className="flex flex-col items-stretch h-auto border-r w-full rounded-none">
                    <button
                      onClick={() => setActiveTab('profile')}
                      className={`flex items-center justify-start px-6 py-3 text-left ${
                        activeTab === 'profile' 
                          ? 'border-r-2 border-cameroon-green text-cameroon-green font-medium' 
                          : 'text-gray-600 dark:text-gray-300'
                      }`}
                    >
                      <User className="mr-2 h-4 w-4" />
                      Profile
                    </button>
                    <button
                      onClick={() => setActiveTab('orders')}
                      className={`flex items-center justify-start px-6 py-3 text-left ${
                        activeTab === 'orders' 
                          ? 'border-r-2 border-cameroon-green text-cameroon-green font-medium' 
                          : 'text-gray-600 dark:text-gray-300'
                      }`}
                    >
                      <Package className="mr-2 h-4 w-4" />
                      Orders
                    </button>
                    <button
                      onClick={() => setActiveTab('payment')}
                      className={`flex items-center justify-start px-6 py-3 text-left ${
                        activeTab === 'payment' 
                          ? 'border-r-2 border-cameroon-green text-cameroon-green font-medium' 
                          : 'text-gray-600 dark:text-gray-300'
                      }`}
                    >
                      <CreditCard className="mr-2 h-4 w-4" />
                      Payment Methods
                    </button>
                  </div>
                </CardContent>
              </Card>
            </div>

            {/* Main Content */}
            <div className="flex-1">
              {activeTab === 'profile' && (
                <Card>
                  <CardHeader>
                    <CardTitle>Profile Information</CardTitle>
                  </CardHeader>
                  <CardContent className="p-6">
                    <form onSubmit={handleProfileUpdate} className="space-y-6">
                      <div className="space-y-2">
                        <label htmlFor="name" className="text-sm font-medium">Full Name</label>
                        <Input 
                          id="name" 
                          value={name} 
                          onChange={(e) => setName(e.target.value)} 
                          placeholder="Your full name"
                        />
                      </div>
                      
                      <div className="space-y-2">
                        <label htmlFor="email" className="text-sm font-medium">Email</label>
                        <Input 
                          id="email" 
                          type="email" 
                          value={email} 
                          onChange={(e) => setEmail(e.target.value)} 
                          placeholder="Your email"
                        />
                      </div>
                      
                      <div className="space-y-2">
                        <label htmlFor="phone" className="text-sm font-medium">Phone Number</label>
                        <Input 
                          id="phone" 
                          value={phone} 
                          onChange={(e) => setPhone(e.target.value)} 
                          placeholder="Your phone number"
                        />
                      </div>
                      
                      <Button 
                        type="submit" 
                        className="bg-cameroon-green hover:bg-cameroon-green/90"
                        disabled={isSubmitting}
                      >
                        {isSubmitting ? 'Saving...' : 'Save Changes'}
                      </Button>
                    </form>

                    <div className="border-t mt-8 pt-8">
                      <h3 className="text-lg font-medium mb-4">Password</h3>
                      <form onSubmit={handlePasswordUpdate} className="space-y-6">
                        <div className="space-y-2">
                          <label htmlFor="current-password" className="text-sm font-medium">Current Password</label>
                          <div className="relative">
                            <Input 
                              id="current-password" 
                              type={showPassword ? 'text' : 'password'} 
                              value={currentPassword} 
                              onChange={(e) => setCurrentPassword(e.target.value)} 
                              placeholder="Your current password"
                              className="pr-10"
                            />
                            <button
                              type="button"
                              className="absolute right-2 top-1/2 transform -translate-y-1/2 text-gray-500"
                              onClick={() => setShowPassword(!showPassword)}
                            >
                              {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
                            </button>
                          </div>
                        </div>
                        
                        <div className="space-y-2">
                          <label htmlFor="new-password" className="text-sm font-medium">New Password</label>
                          <Input 
                            id="new-password" 
                            type={showPassword ? 'text' : 'password'} 
                            value={newPassword} 
                            onChange={(e) => setNewPassword(e.target.value)} 
                            placeholder="New password"
                          />
                        </div>
                        
                        <div className="space-y-2">
                          <label htmlFor="confirm-password" className="text-sm font-medium">Confirm New Password</label>
                          <Input 
                            id="confirm-password" 
                            type={showPassword ? 'text' : 'password'} 
                            value={confirmPassword} 
                            onChange={(e) => setConfirmPassword(e.target.value)} 
                            placeholder="Confirm new password"
                          />
                        </div>
                        
                        <Button 
                          type="submit" 
                          className="bg-cameroon-green hover:bg-cameroon-green/90"
                          disabled={isSubmitting}
                        >
                          {isSubmitting ? 'Updating...' : 'Update Password'}
                        </Button>
                      </form>
                    </div>
                  </CardContent>
                </Card>
              )}
              
              {activeTab === 'orders' && (
                <Card>
                  <CardHeader>
                    <CardTitle>Order History</CardTitle>
                  </CardHeader>
                  <CardContent className="p-6">
                    <div className="text-center py-8">
                      <h3 className="text-lg font-medium mb-2">No orders yet</h3>
                      <p className="text-gray-500 dark:text-gray-400 mb-4">You haven't made any purchases yet.</p>
                      <Button
                        onClick={() => window.location.href = '/marketplace'}
                        className="bg-cameroon-green hover:bg-cameroon-green/90"
                      >
                        Start Shopping
                      </Button>
                    </div>
                  </CardContent>
                </Card>
              )}
              
              {activeTab === 'payment' && (
                <Card>
                  <CardHeader>
                    <CardTitle>Payment Methods</CardTitle>
                  </CardHeader>
                  <CardContent className="p-6">
                    <div className="text-center py-8">
                      <h3 className="text-lg font-medium mb-2">No payment methods</h3>
                      <p className="text-gray-500 dark:text-gray-400 mb-4">You haven't added any payment methods yet.</p>
                      <Button
                        className="bg-cameroon-green hover:bg-cameroon-green/90"
                      >
                        Add Payment Method
                      </Button>
                    </div>
                  </CardContent>
                </Card>
              )}
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default UserProfile;
