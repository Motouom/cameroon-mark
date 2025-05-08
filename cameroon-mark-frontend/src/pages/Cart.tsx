
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import Layout from '@/components/layout/Layout';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { useCart } from '@/contexts/CartContext';
import { useAuth } from '@/contexts/AuthContext';
import { Plus, Minus, X, ShoppingCart, ArrowRight } from 'lucide-react';

const Cart = () => {
  const navigate = useNavigate();
  const { cartItems, removeFromCart, updateQuantity, clearCart, totalItems, totalPrice } = useCart();
  const { isAuthenticated } = useAuth();
  const [promoCode, setPromoCode] = useState('');
  const [isApplyingPromo, setIsApplyingPromo] = useState(false);
  
  const handleApplyPromoCode = (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!promoCode) return;
    
    setIsApplyingPromo(true);
    
    // Simulate promo code check
    setTimeout(() => {
      setPromoCode('');
      setIsApplyingPromo(false);
      
      // Mock response - no valid promo code
      alert('Invalid or expired promo code');
    }, 1000);
  };

  const handleProceedToCheckout = () => {
    if (!isAuthenticated) {
      navigate('/login?redirect=checkout');
    } else {
      navigate('/checkout');
    }
  };

  return (
    <Layout>
      <div className="bg-gray-50 dark:bg-gray-900 min-h-screen py-16">
        <div className="container-custom mx-auto">
          <h1 className="text-3xl font-bold mb-3 text-gray-900 dark:text-white">
            Shopping Cart
          </h1>
          <div className="text-sm text-gray-500 dark:text-gray-400 mb-8">
            {totalItems} {totalItems === 1 ? 'item' : 'items'} in your cart
          </div>
          
          {cartItems.length === 0 ? (
            <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-12 text-center">
              <div className="w-16 h-16 rounded-full bg-gray-100 dark:bg-gray-700 flex items-center justify-center mx-auto mb-4">
                <ShoppingCart className="h-8 w-8 text-gray-500 dark:text-gray-400" />
              </div>
              <h2 className="text-2xl font-semibold mb-2 text-gray-900 dark:text-white">Your cart is empty</h2>
              <p className="text-gray-500 dark:text-gray-400 mb-6">
                Looks like you haven't added any products to your cart yet.
              </p>
              <Link to="/marketplace">
                <Button className="bg-cameroon-green hover:bg-cameroon-green/90">
                  Continue Shopping
                </Button>
              </Link>
            </div>
          ) : (
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
              {/* Cart Items */}
              <div className="lg:col-span-2">
                <Card className="border border-gray-200 dark:border-gray-700">
                  <CardHeader className="border-b border-gray-200 dark:border-gray-700">
                    <CardTitle>Cart Items</CardTitle>
                  </CardHeader>
                  <CardContent className="p-0">
                    {cartItems.map((item) => (
                      <div 
                        key={item.productId} 
                        className="flex flex-col sm:flex-row border-b border-gray-200 dark:border-gray-700 last:border-0 p-4 sm:p-6"
                      >
                        {/* Product Image */}
                        <div className="sm:w-24 h-24 rounded-md overflow-hidden flex-shrink-0 mb-4 sm:mb-0">
                          <img 
                            src={item.product.images[0]} 
                            alt={item.product.title} 
                            className="w-full h-full object-cover"
                          />
                        </div>
                        
                        {/* Product Details */}
                        <div className="flex-1 sm:ml-4 flex flex-col sm:flex-row justify-between">
                          <div>
                            <Link to={`/product/${item.productId}`}>
                              <h3 className="font-medium text-gray-900 dark:text-white hover:text-cameroon-green mb-1">
                                {item.product.title}
                              </h3>
                            </Link>
                            <div className="text-sm text-gray-500 dark:text-gray-400 mb-3">
                              Seller: {item.product.seller?.name}
                            </div>
                            
                            {/* Mobile Price */}
                            <div className="sm:hidden text-cameroon-green dark:text-cameroon-yellow font-semibold mb-3">
                              {(item.product.price * item.quantity).toLocaleString()} FCFA
                            </div>
                            
                            {/* Quantity Controls */}
                            <div className="flex items-center">
                              <Button 
                                variant="outline"
                                size="icon"
                                onClick={() => updateQuantity(item.productId, item.quantity - 1)}
                                disabled={item.quantity <= 1}
                                className="h-8 w-8 rounded-r-none"
                              >
                                <Minus size={14} />
                              </Button>
                              <div className="h-8 w-12 flex items-center justify-center border-y border-gray-200 dark:border-gray-700 text-sm">
                                {item.quantity}
                              </div>
                              <Button 
                                variant="outline"
                                size="icon"
                                onClick={() => updateQuantity(item.productId, item.quantity + 1)}
                                disabled={item.quantity >= item.product.stock}
                                className="h-8 w-8 rounded-l-none"
                              >
                                <Plus size={14} />
                              </Button>
                            </div>
                          </div>
                          
                          <div className="mt-4 sm:mt-0 flex sm:flex-col items-center sm:items-end justify-between">
                            {/* Desktop Price */}
                            <div className="hidden sm:block text-cameroon-green dark:text-cameroon-yellow font-semibold mb-2">
                              {(item.product.price * item.quantity).toLocaleString()} FCFA
                            </div>
                            
                            {/* Remove Button */}
                            <button
                              onClick={() => removeFromCart(item.productId)}
                              className="text-gray-500 dark:text-gray-400 hover:text-red-500 dark:hover:text-red-400 text-sm flex items-center"
                            >
                              <X size={14} className="mr-1" />
                              Remove
                            </button>
                          </div>
                        </div>
                      </div>
                    ))}
                  </CardContent>
                  <CardFooter className="flex justify-between border-t border-gray-200 dark:border-gray-700 p-4 sm:p-6">
                    <Button 
                      variant="ghost" 
                      onClick={clearCart}
                      className="text-red-500 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-950/50"
                    >
                      Clear Cart
                    </Button>
                    <Link to="/marketplace">
                      <Button variant="outline">
                        Continue Shopping
                      </Button>
                    </Link>
                  </CardFooter>
                </Card>
              </div>
              
              {/* Order Summary */}
              <div className="lg:col-span-1">
                <Card className="border border-gray-200 dark:border-gray-700 sticky top-4">
                  <CardHeader className="border-b border-gray-200 dark:border-gray-700">
                    <CardTitle>Order Summary</CardTitle>
                  </CardHeader>
                  <CardContent className="p-6 space-y-4">
                    <div className="flex justify-between text-sm text-gray-600 dark:text-gray-400">
                      <span>Subtotal ({totalItems} items)</span>
                      <span>{totalPrice.toLocaleString()} FCFA</span>
                    </div>
                    <div className="flex justify-between text-sm text-gray-600 dark:text-gray-400">
                      <span>Shipping Fee</span>
                      <span>2,000 FCFA</span>
                    </div>
                    
                    {/* Promo Code Form */}
                    <div className="pt-4 border-t border-gray-200 dark:border-gray-700">
                      <form onSubmit={handleApplyPromoCode} className="flex items-center">
                        <Input
                          type="text"
                          placeholder="Promo code"
                          value={promoCode}
                          onChange={(e) => setPromoCode(e.target.value)}
                          className="flex-1"
                        />
                        <Button 
                          type="submit" 
                          variant="outline" 
                          disabled={isApplyingPromo || !promoCode}
                          className="ml-2"
                        >
                          Apply
                        </Button>
                      </form>
                    </div>
                    
                    {/* Total */}
                    <div className="pt-4 border-t border-gray-200 dark:border-gray-700 flex justify-between">
                      <span className="font-medium text-gray-900 dark:text-white">Total</span>
                      <span className="font-bold text-xl text-cameroon-green dark:text-cameroon-yellow">
                        {(totalPrice + 2000).toLocaleString()} FCFA
                      </span>
                    </div>
                  </CardContent>
                  <CardFooter className="p-6 pt-0">
                    <Button 
                      className="w-full bg-cameroon-green hover:bg-cameroon-green/90"
                      onClick={handleProceedToCheckout}
                    >
                      {isAuthenticated ? (
                        <>Proceed to Checkout <ArrowRight className="ml-2 h-4 w-4" /></>
                      ) : (
                        <>Log In to Checkout <ArrowRight className="ml-2 h-4 w-4" /></>
                      )}
                    </Button>
                  </CardFooter>
                </Card>
              </div>
            </div>
          )}
        </div>
      </div>
    </Layout>
  );
};

export default Cart;
