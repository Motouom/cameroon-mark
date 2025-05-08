
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { 
  Card, 
  CardContent,
  CardFooter,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Product } from '@/types';
import { Edit, Trash, Search, Eye } from 'lucide-react';
import { useToast } from '@/components/ui/use-toast';

// Mock data for demonstration
const mockProducts: Product[] = [
  {
    id: '1',
    title: 'Traditional Cameroonian Mask',
    description: 'Hand-carved wooden mask from Western Cameroon',
    price: 95.00,
    images: ['https://images.unsplash.com/photo-1649972904349-6e44c42644a7'],
    categoryId: '1',
    sellerId: '1',
    stock: 10,
    location: 'Douala, Cameroon',
    createdAt: new Date(),
    featured: true
  },
  {
    id: '2',
    title: 'Organic Coffee Beans',
    description: '500g of premium organic coffee from Mount Cameroon',
    price: 15.50,
    images: ['https://images.unsplash.com/photo-1488590528505-98d2b5aba04b'],
    categoryId: '2',
    sellerId: '1',
    stock: 45,
    location: 'Bamenda, Cameroon',
    createdAt: new Date(),
  }
];

const ProductsList = () => {
  const navigate = useNavigate();
  const { toast } = useToast();
  const [products, setProducts] = useState<Product[]>(mockProducts);
  const [searchQuery, setSearchQuery] = useState('');
  
  const handleDelete = (productId: string) => {
    // In a real app, you would call an API to delete the product
    setProducts(products.filter(product => product.id !== productId));
    toast({
      title: "Product deleted",
      description: "The product has been successfully deleted.",
    });
  };

  // Filter products based on search query
  const filteredProducts = products.filter(product =>
    product.title.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div>
      <div className="mb-6">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" />
          <Input
            type="text"
            placeholder="Search products..."
            className="pl-10"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
      </div>

      {filteredProducts.length === 0 ? (
        <div className="text-center py-10">
          <p className="text-xl font-medium">No products found</p>
          <p className="text-gray-500 mt-2">
            {searchQuery ? 'Try a different search term' : 'Add a product to get started'}
          </p>
          <Button 
            onClick={() => navigate('/seller/product/new')}
            className="mt-4"
          >
            Add Your First Product
          </Button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredProducts.map(product => (
            <Card key={product.id} className="flex flex-col">
              <div className="relative h-48 bg-gray-100">
                <img
                  src={product.images[0] || '/placeholder.svg'}
                  alt={product.title}
                  className="w-full h-full object-cover"
                />
              </div>
              <CardContent className="flex-grow p-4">
                <h3 className="text-lg font-medium line-clamp-1">{product.title}</h3>
                <p className="text-gray-500 text-sm line-clamp-2 mt-1">
                  {product.description}
                </p>
                <div className="mt-2 flex items-center justify-between">
                  <span className="font-medium">${product.price.toFixed(2)}</span>
                  <span className="text-sm text-gray-500">Stock: {product.stock}</span>
                </div>
              </CardContent>
              <CardFooter className="flex justify-between border-t p-4">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => navigate(`/product/${product.id}`)}
                >
                  <Eye className="h-4 w-4 mr-2" />
                  View
                </Button>
                <div className="flex gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => navigate(`/seller/product/edit/${product.id}`)}
                  >
                    <Edit className="h-4 w-4 mr-2" />
                    Edit
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    className="text-red-500 border-red-200 hover:bg-red-50"
                    onClick={() => handleDelete(product.id)}
                  >
                    <Trash className="h-4 w-4 mr-2" />
                    Delete
                  </Button>
                </div>
              </CardFooter>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
};

export default ProductsList;
