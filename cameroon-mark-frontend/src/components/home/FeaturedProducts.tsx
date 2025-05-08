
import { Link } from 'react-router-dom';
import { getFeaturedProducts } from '@/data/mockData';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Star } from 'lucide-react';
import { useCart } from '@/contexts/CartContext';

const FeaturedProducts = () => {
  const featuredProducts = getFeaturedProducts();
  const { addToCart } = useCart();

  return (
    <section className="py-16 bg-gray-50 dark:bg-gray-900">
      <div className="container-custom mx-auto">
        {/* Section Title */}
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold mb-3 text-gray-900 dark:text-white">Featured Products</h2>
          <p className="text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
            Hand-picked exceptional products from Cameroonian artisans and businesses
          </p>
        </div>
        
        {/* Products Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          {featuredProducts.map((product) => (
            <Card key={product.id} className="overflow-hidden hover-scale border border-gray-200 dark:border-gray-800">
              <Link to={`/product/${product.id}`} className="block">
                <div className="aspect-[4/3] relative overflow-hidden">
                  <img 
                    src={product.images[0]} 
                    alt={product.title} 
                    className="w-full h-full object-cover"
                  />
                  {product.stock < 5 && (
                    <div className="absolute top-2 right-2 bg-red-500 text-white text-xs font-semibold px-2 py-1 rounded-full">
                      Low Stock
                    </div>
                  )}
                </div>
              </Link>
              
              <CardContent className="p-4">
                <Link to={`/product/${product.id}`} className="block">
                  <h3 className="font-semibold text-gray-900 dark:text-white mb-1 line-clamp-1">
                    {product.title}
                  </h3>
                  
                  <div className="flex items-center mb-2">
                    <div className="flex items-center">
                      {[...Array(5)].map((_, i) => (
                        <Star 
                          key={i} 
                          size={14} 
                          className={`${
                            i < Math.floor(product.rating || 0) 
                              ? "text-yellow-400 fill-yellow-400" 
                              : "text-gray-300 dark:text-gray-600"
                          }`} 
                        />
                      ))}
                    </div>
                    <span className="text-xs text-gray-500 dark:text-gray-400 ml-1">
                      ({product.rating})
                    </span>
                  </div>
                  
                  <div className="flex items-center justify-between">
                    <div>
                      <p className="font-bold text-cameroon-green dark:text-cameroon-yellow">
                        {product.price.toLocaleString()} FCFA
                      </p>
                      <p className="text-xs text-gray-500 dark:text-gray-400">{product.location}</p>
                    </div>
                    
                    <Button 
                      size="sm"
                      onClick={(e) => {
                        e.preventDefault();
                        addToCart(product);
                      }}
                      className="bg-cameroon-green hover:bg-cameroon-green/90"
                    >
                      Add to Cart
                    </Button>
                  </div>
                </Link>
              </CardContent>
            </Card>
          ))}
        </div>
        
        {/* View All Button */}
        <div className="text-center mt-10">
          <Link to="/marketplace">
            <Button variant="outline" className="border-cameroon-green text-cameroon-green hover:bg-cameroon-green/10">
              View All Products
            </Button>
          </Link>
        </div>
      </div>
    </section>
  );
};

export default FeaturedProducts;
