
import { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import Layout from '@/components/layout/Layout';
import { getProductById, getCategoryById } from '@/data/mockData';
import { Button } from '@/components/ui/button';
import { 
  Card,
  CardContent,
} from '@/components/ui/card';
import { 
  Tabs, 
  TabsContent, 
  TabsList, 
  TabsTrigger 
} from '@/components/ui/tabs';
import { useCart } from '@/contexts/CartContext';
import { useToast } from '@/components/ui/use-toast';
import { 
  Star, 
  ChevronRight, 
  ChevronLeft, 
  Heart, 
  Share, 
  MapPin, 
  Calendar, 
  ShoppingCart,
  Minus,
  Plus,
  Check,
  Store,
  Phone,
  Mail,
  User,
} from 'lucide-react';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import ProductCard from '@/components/product/ProductCard';
import { Product } from '@/types';

const ProductDetail = () => {
  const { id } = useParams<{ id: string }>();
  const product = getProductById(id || '');
  const category = product ? getCategoryById(product.categoryId) : undefined;
  const relatedProducts = product ? getProductById('1') ? [getProductById('1')!, getProductById('3')!, getProductById('5')!] : [] : [];
  
  const [currentImageIndex, setCurrentImageIndex] = useState(0);
  const [quantity, setQuantity] = useState(1);
  const { addToCart } = useCart();
  const { toast } = useToast();
  
  if (!product || !category) {
    return (
      <Layout>
        <div className="container-custom mx-auto py-16 text-center">
          <h1 className="text-2xl font-semibold mb-4">Product Not Found</h1>
          <p className="mb-8">Sorry, the product you're looking for doesn't exist or has been removed.</p>
          <Link to="/marketplace">
            <Button>Return to Marketplace</Button>
          </Link>
        </div>
      </Layout>
    );
  }

  const handlePreviousImage = () => {
    setCurrentImageIndex(prev => 
      prev === 0 ? product.images.length - 1 : prev - 1
    );
  };

  const handleNextImage = () => {
    setCurrentImageIndex(prev => 
      prev === product.images.length - 1 ? 0 : prev + 1
    );
  };

  const increaseQuantity = () => {
    if (quantity < product.stock) {
      setQuantity(prev => prev + 1);
    } else {
      toast({
        title: "Maximum stock reached",
        description: "You've reached the maximum available stock for this product",
        variant: "destructive",
      });
    }
  };

  const decreaseQuantity = () => {
    if (quantity > 1) {
      setQuantity(prev => prev - 1);
    }
  };

  const handleAddToCart = () => {
    addToCart(product, quantity);
  };

  return (
    <Layout>
      <div className="bg-gray-50 dark:bg-gray-900 py-16">
        <div className="container-custom mx-auto">
          {/* Breadcrumbs */}
          <div className="flex items-center text-sm mb-6">
            <Link to="/" className="text-gray-500 dark:text-gray-400 hover:text-cameroon-green">Home</Link>
            <ChevronRight size={14} className="mx-2 text-gray-400" />
            <Link to="/marketplace" className="text-gray-500 dark:text-gray-400 hover:text-cameroon-green">Marketplace</Link>
            <ChevronRight size={14} className="mx-2 text-gray-400" />
            <Link to={`/marketplace/category/${category.slug}`} className="text-gray-500 dark:text-gray-400 hover:text-cameroon-green">
              {category.name}
            </Link>
            <ChevronRight size={14} className="mx-2 text-gray-400" />
            <span className="text-gray-900 dark:text-white">{product.title}</span>
          </div>

          {/* Product Info Section */}
          <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden mb-10">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-8 p-6 md:p-8">
              {/* Product Images */}
              <div className="lg:col-span-2">
                {/* Main Image */}
                <div className="relative aspect-square rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden mb-4">
                  <img 
                    src={product.images[currentImageIndex]} 
                    alt={product.title} 
                    className="w-full h-full object-cover"
                  />
                  {/* Navigation Arrows */}
                  <button 
                    onClick={handlePreviousImage}
                    className="absolute left-2 top-1/2 transform -translate-y-1/2 w-8 h-8 bg-white/80 dark:bg-gray-800/80 rounded-full flex items-center justify-center text-gray-700 dark:text-gray-300 hover:bg-white dark:hover:bg-gray-800"
                  >
                    <ChevronLeft size={18} />
                  </button>
                  <button 
                    onClick={handleNextImage}
                    className="absolute right-2 top-1/2 transform -translate-y-1/2 w-8 h-8 bg-white/80 dark:bg-gray-800/80 rounded-full flex items-center justify-center text-gray-700 dark:text-gray-300 hover:bg-white dark:hover:bg-gray-800"
                  >
                    <ChevronRight size={18} />
                  </button>
                </div>
                
                {/* Thumbnail Gallery */}
                <div className="grid grid-cols-4 gap-2">
                  {product.images.map((image, index) => (
                    <div 
                      key={index}
                      className={`aspect-square rounded-md border cursor-pointer overflow-hidden ${
                        index === currentImageIndex 
                          ? 'border-cameroon-green ring-2 ring-cameroon-green/20' 
                          : 'border-gray-200 dark:border-gray-700'
                      }`}
                      onClick={() => setCurrentImageIndex(index)}
                    >
                      <img 
                        src={image} 
                        alt={`${product.title} - view ${index + 1}`} 
                        className="w-full h-full object-cover"
                      />
                    </div>
                  ))}
                </div>
              </div>
              
              {/* Product Details */}
              <div className="lg:col-span-3 space-y-6">
                <div>
                  <h1 className="text-2xl md:text-3xl font-bold text-gray-900 dark:text-white mb-2">
                    {product.title}
                  </h1>
                  
                  <div className="flex flex-wrap items-center gap-4 mb-4">
                    <div className="flex items-center">
                      <div className="flex">
                        {[...Array(5)].map((_, i) => (
                          <Star 
                            key={i} 
                            size={16} 
                            className={`${
                              i < Math.floor(product.rating || 0) 
                                ? "text-yellow-400 fill-yellow-400" 
                                : "text-gray-300 dark:text-gray-600"
                            }`} 
                          />
                        ))}
                      </div>
                      <span className="ml-2 text-sm text-gray-600 dark:text-gray-400">
                        {product.rating} Rating
                      </span>
                    </div>
                    
                    <div className="text-sm text-gray-600 dark:text-gray-400 flex items-center">
                      <MapPin size={14} className="mr-1" />
                      {product.location}
                    </div>
                    
                    <div className="text-sm text-gray-600 dark:text-gray-400 flex items-center">
                      <Calendar size={14} className="mr-1" />
                      {new Date(product.createdAt).toLocaleDateString()}
                    </div>
                  </div>
                  
                  <div className="bg-cameroon-green/5 dark:bg-cameroon-green/10 rounded-lg p-4 mb-6">
                    <div className="text-3xl font-bold text-cameroon-green dark:text-cameroon-yellow">
                      {product.price.toLocaleString()} FCFA
                    </div>
                    <div className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                      Free shipping to major cities
                    </div>
                  </div>
                </div>
                
                {/* Order Section */}
                <div className="space-y-4">
                  {/* Stock Status */}
                  <div className="flex items-center">
                    <div className={`w-3 h-3 rounded-full mr-2 ${product.stock > 0 ? 'bg-green-500' : 'bg-red-500'}`}></div>
                    <span className="text-sm font-medium">
                      {product.stock > 0 
                        ? `In Stock (${product.stock} available)` 
                        : 'Out of Stock'}
                    </span>
                  </div>
                  
                  {/* Quantity */}
                  <div>
                    <label className="text-sm font-medium mb-2 block">Quantity:</label>
                    <div className="flex items-center w-36">
                      <Button 
                        variant="outline"
                        size="icon"
                        onClick={decreaseQuantity}
                        disabled={quantity <= 1}
                        className="h-10 w-10 rounded-r-none"
                      >
                        <Minus size={16} />
                      </Button>
                      <div className="h-10 w-16 flex items-center justify-center border-y border-gray-200 dark:border-gray-700">
                        {quantity}
                      </div>
                      <Button 
                        variant="outline"
                        size="icon"
                        onClick={increaseQuantity}
                        disabled={quantity >= product.stock}
                        className="h-10 w-10 rounded-l-none"
                      >
                        <Plus size={16} />
                      </Button>
                    </div>
                  </div>
                  
                  {/* Action Buttons */}
                  <div className="flex flex-col sm:flex-row gap-3">
                    <Button 
                      size="lg"
                      className="bg-cameroon-green hover:bg-cameroon-green/90 flex-1"
                      onClick={handleAddToCart}
                      disabled={product.stock === 0}
                    >
                      <ShoppingCart className="mr-2 h-5 w-5" /> Add to Cart
                    </Button>
                    <Button 
                      size="lg"
                      variant="outline"
                      className="flex-1"
                    >
                      <Heart className="mr-2 h-5 w-5" /> Save
                    </Button>
                    <Button 
                      size="icon"
                      variant="outline"
                      className="h-12 w-12"
                    >
                      <Share className="h-5 w-5" />
                    </Button>
                  </div>
                </div>
                
                {/* Product Features */}
                <div className="pt-4 space-y-2 border-t border-gray-200 dark:border-gray-700">
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">Key Features</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                    <div className="flex items-center">
                      <Check size={16} className="mr-2 text-cameroon-green" />
                      <span className="text-gray-700 dark:text-gray-300">100% authentic product</span>
                    </div>
                    <div className="flex items-center">
                      <Check size={16} className="mr-2 text-cameroon-green" />
                      <span className="text-gray-700 dark:text-gray-300">Handmade by local artisans</span>
                    </div>
                    <div className="flex items-center">
                      <Check size={16} className="mr-2 text-cameroon-green" />
                      <span className="text-gray-700 dark:text-gray-300">Sustainable materials</span>
                    </div>
                    <div className="flex items-center">
                      <Check size={16} className="mr-2 text-cameroon-green" />
                      <span className="text-gray-700 dark:text-gray-300">Quality checked</span>
                    </div>
                  </div>
                </div>
                
                {/* Seller Info */}
                <div className="flex items-center pt-4 border-t border-gray-200 dark:border-gray-700">
                  <Avatar className="h-12 w-12 mr-4">
                    <AvatarImage src={product.seller?.avatar} alt={product.seller?.name} />
                    <AvatarFallback>{product.seller?.name.charAt(0)}</AvatarFallback>
                  </Avatar>
                  <div className="flex-1">
                    <div className="font-medium text-gray-900 dark:text-white">
                      {product.seller?.name}
                    </div>
                    <div className="text-sm text-gray-500 dark:text-gray-400">
                      Seller since {product.seller?.createdAt.getFullYear()}
                    </div>
                  </div>
                  <Link to={`/seller/${product.sellerId}`}>
                    <Button variant="outline" size="sm">View Shop</Button>
                  </Link>
                </div>
              </div>
            </div>
            
            {/* Product Details Tabs */}
            <Tabs defaultValue="description" className="border-t border-gray-200 dark:border-gray-700">
              <TabsList className="border-b border-gray-200 dark:border-gray-700 w-full justify-start rounded-none px-6 md:px-8">
                <TabsTrigger value="description">Description</TabsTrigger>
                <TabsTrigger value="specifications">Specifications</TabsTrigger>
                <TabsTrigger value="sellerInfo">Seller Info</TabsTrigger>
              </TabsList>
              <div className="p-6 md:p-8">
                <TabsContent value="description" className="mt-0 space-y-4">
                  <p className="text-gray-700 dark:text-gray-300">
                    {product.description}
                  </p>
                  <p className="text-gray-700 dark:text-gray-300">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla facilisi. Sed euismod, nisl sit amet aliquam lacinia, nisl nisl aliquet nisl, vel aliquet nisl nisl sit amet lorem. Nulla facilisi. Sed euismod, nisl sit amet aliquam lacinia, nisl nisl aliquet nisl, vel aliquet nisl nisl sit amet lorem.
                  </p>
                  <p className="text-gray-700 dark:text-gray-300">
                    Nulla facilisi. Sed euismod, nisl sit amet aliquam lacinia, nisl nisl aliquet nisl, vel aliquet nisl nisl sit amet lorem.
                  </p>
                </TabsContent>
                <TabsContent value="specifications" className="mt-0">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div>
                      <h3 className="font-semibold mb-3 text-gray-900 dark:text-white">Product Specifications</h3>
                      <div className="space-y-2">
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Material</div>
                          <div className="text-gray-900 dark:text-white">Handcrafted Wood</div>
                        </div>
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Dimensions</div>
                          <div className="text-gray-900 dark:text-white">25 cm x 15 cm</div>
                        </div>
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Weight</div>
                          <div className="text-gray-900 dark:text-white">0.5 kg</div>
                        </div>
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Region</div>
                          <div className="text-gray-900 dark:text-white">Western Cameroon</div>
                        </div>
                      </div>
                    </div>
                    <div>
                      <h3 className="font-semibold mb-3 text-gray-900 dark:text-white">Shipping Information</h3>
                      <div className="space-y-2">
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Shipping From</div>
                          <div className="text-gray-900 dark:text-white">{product.location}</div>
                        </div>
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Processing Time</div>
                          <div className="text-gray-900 dark:text-white">1-2 business days</div>
                        </div>
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Estimated Delivery</div>
                          <div className="text-gray-900 dark:text-white">3-7 business days</div>
                        </div>
                        <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700 py-2">
                          <div className="text-gray-600 dark:text-gray-400">Return Policy</div>
                          <div className="text-gray-900 dark:text-white">14 days</div>
                        </div>
                      </div>
                    </div>
                  </div>
                </TabsContent>
                <TabsContent value="sellerInfo" className="mt-0">
                  <div className="flex flex-col md:flex-row gap-8">
                    <div className="md:w-64">
                      <Card className="overflow-hidden">
                        <div className="h-32 bg-gradient-to-r from-cameroon-green to-cameroon-forest"></div>
                        <CardContent className="pt-0">
                          <div className="flex justify-center -mt-12">
                            <Avatar className="h-24 w-24 border-4 border-white dark:border-gray-800">
                              <AvatarImage src={product.seller?.avatar} alt={product.seller?.name} />
                              <AvatarFallback className="text-2xl">{product.seller?.name.charAt(0)}</AvatarFallback>
                            </Avatar>
                          </div>
                          <div className="text-center mt-3">
                            <h3 className="font-semibold text-lg text-gray-900 dark:text-white mb-1">
                              {product.seller?.name}
                            </h3>
                            <div className="flex items-center justify-center mb-3">
                              <div className="flex">
                                {[...Array(5)].map((_, i) => (
                                  <Star 
                                    key={i} 
                                    size={14} 
                                    className={`${
                                      i < 4
                                        ? "text-yellow-400 fill-yellow-400" 
                                        : "text-gray-300 dark:text-gray-600"
                                    }`} 
                                  />
                                ))}
                              </div>
                              <span className="ml-1 text-xs text-gray-600 dark:text-gray-400">
                                (4.8)
                              </span>
                            </div>
                            <div className="text-sm text-gray-500 dark:text-gray-400 mb-4">
                              Joined {product.seller?.createdAt.toLocaleDateString()}
                            </div>
                            <Button className="w-full bg-cameroon-green hover:bg-cameroon-green/90 mb-2">
                              <Store className="mr-2 h-4 w-4" /> Visit Shop
                            </Button>
                            <Button variant="outline" className="w-full">
                              <Mail className="mr-2 h-4 w-4" /> Contact
                            </Button>
                          </div>
                        </CardContent>
                      </Card>
                    </div>
                    <div className="flex-1">
                      <h3 className="font-semibold text-lg text-gray-900 dark:text-white mb-4">About the Seller</h3>
                      <p className="text-gray-700 dark:text-gray-300 mb-6">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla facilisi. Sed euismod, nisl sit amet aliquam lacinia, nisl nisl aliquet nisl, vel aliquet nisl nisl sit amet lorem. We take pride in our craftsmanship and stand behind every product we sell.
                      </p>
                      
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                        <div className="bg-gray-50 dark:bg-gray-900 p-4 rounded-lg">
                          <div className="flex items-center">
                            <div className="bg-cameroon-green/10 p-2 rounded-full mr-3">
                              <MapPin className="h-5 w-5 text-cameroon-green" />
                            </div>
                            <div>
                              <div className="text-sm text-gray-500 dark:text-gray-400">Location</div>
                              <div className="font-medium text-gray-900 dark:text-white">{product.seller?.location}</div>
                            </div>
                          </div>
                        </div>
                        
                        <div className="bg-gray-50 dark:bg-gray-900 p-4 rounded-lg">
                          <div className="flex items-center">
                            <div className="bg-cameroon-green/10 p-2 rounded-full mr-3">
                              <Phone className="h-5 w-5 text-cameroon-green" />
                            </div>
                            <div>
                              <div className="text-sm text-gray-500 dark:text-gray-400">Contact</div>
                              <div className="font-medium text-gray-900 dark:text-white">{product.seller?.phone}</div>
                            </div>
                          </div>
                        </div>
                        
                        <div className="bg-gray-50 dark:bg-gray-900 p-4 rounded-lg">
                          <div className="flex items-center">
                            <div className="bg-cameroon-green/10 p-2 rounded-full mr-3">
                              <Mail className="h-5 w-5 text-cameroon-green" />
                            </div>
                            <div>
                              <div className="text-sm text-gray-500 dark:text-gray-400">Email</div>
                              <div className="font-medium text-gray-900 dark:text-white">{product.seller?.email}</div>
                            </div>
                          </div>
                        </div>
                        
                        <div className="bg-gray-50 dark:bg-gray-900 p-4 rounded-lg">
                          <div className="flex items-center">
                            <div className="bg-cameroon-green/10 p-2 rounded-full mr-3">
                              <User className="h-5 w-5 text-cameroon-green" />
                            </div>
                            <div>
                              <div className="text-sm text-gray-500 dark:text-gray-400">Products</div>
                              <div className="font-medium text-gray-900 dark:text-white">24 products</div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </TabsContent>
              </div>
            </Tabs>
          </div>
          
          {/* Related Products */}
          <div className="mb-10">
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-6">Related Products</h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
              {relatedProducts.map((product) => (
                <ProductCard key={product.id} product={product} />
              ))}
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default ProductDetail;
