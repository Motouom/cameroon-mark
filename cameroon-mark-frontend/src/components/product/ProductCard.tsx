
import { Link } from 'react-router-dom';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Star, Heart } from 'lucide-react';
import { useCart } from '@/contexts/CartContext';
import { Product } from '@/types';

interface ProductCardProps {
  product: Product;
}

const ProductCard = ({ product }: ProductCardProps) => {
  const { addToCart } = useCart();

  return (
    <Card className="overflow-hidden hover-scale border border-gray-200 dark:border-gray-800">
      <Link to={`/product/${product.id}`} className="block">
        <div className="aspect-[4/3] relative overflow-hidden">
          <img 
            src={product.images[0]} 
            alt={product.title} 
            className="w-full h-full object-cover"
          />
          <button className="absolute top-2 right-2 bg-white dark:bg-gray-800 p-1.5 rounded-full text-gray-500 hover:text-red-500">
            <Heart size={18} />
          </button>
          {product.stock < 5 && (
            <div className="absolute top-2 left-2 bg-red-500 text-white text-xs font-semibold px-2 py-1 rounded-full">
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
  );
};

export default ProductCard;
