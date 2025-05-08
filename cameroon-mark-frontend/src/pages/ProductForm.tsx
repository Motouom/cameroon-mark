import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { 
  Form, 
  FormControl, 
  FormField, 
  FormItem, 
  FormLabel, 
  FormMessage 
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { 
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { 
  Card, 
  CardContent, 
  CardFooter, 
  CardHeader, 
  CardTitle 
} from '@/components/ui/card';
import { useToast } from '@/components/ui/use-toast';
import Layout from '@/components/layout/Layout';
import ImageUploader from '@/components/seller/ImageUploader';
import { Product, Category } from '@/types';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';

// Mock categories
const mockCategories: Category[] = [
  { id: '1', name: 'Art & Crafts', description: 'Handmade items and traditional crafts', image: 'https://example.com/art.jpg', slug: 'art-crafts' },
  { id: '2', name: 'Food & Drinks', description: 'Local foods and beverages', image: 'https://example.com/food.jpg', slug: 'food-drinks' },
  { id: '3', name: 'Clothing', description: 'Traditional and modern clothing', image: 'https://example.com/clothing.jpg', slug: 'clothing' },
  { id: '4', name: 'Agriculture', description: 'Farm products and crops', image: 'https://example.com/agriculture.jpg', slug: 'agriculture' },
];

// Mock products
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

// Form validation schema
const productFormSchema = z.object({
  title: z.string().min(3, { message: 'Title must be at least 3 characters' }).max(100),
  description: z.string().min(20, { message: 'Description must be at least 20 characters' }),
  price: z.string().refine((val) => !isNaN(parseFloat(val)) && parseFloat(val) > 0, {
    message: 'Price must be a positive number',
  }),
  categoryId: z.string().min(1, { message: 'Please select a category' }),
  stock: z.string().refine((val) => !isNaN(parseInt(val)) && parseInt(val) >= 0, {
    message: 'Stock must be a non-negative number',
  }),
  location: z.string().min(3, { message: 'Location is required' }),
});

type ProductFormValues = z.infer<typeof productFormSchema>;

const ProductForm = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { toast } = useToast();
  const isEditMode = id !== 'new';
  
  const [images, setImages] = useState<string[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  
  const form = useForm<ProductFormValues>({
    resolver: zodResolver(productFormSchema),
    defaultValues: {
      title: '',
      description: '',
      price: '',
      categoryId: '',
      stock: '',
      location: '',
    },
  });
  
  // Load product data if in edit mode
  useEffect(() => {
    if (isEditMode) {
      const product = mockProducts.find(p => p.id === id);
      
      if (product) {
        form.reset({
          title: product.title,
          description: product.description,
          price: product.price.toString(),
          categoryId: product.categoryId,
          stock: product.stock.toString(),
          location: product.location,
        });
        setImages(product.images || []);
      } else {
        // Product not found
        toast({
          title: "Product not found",
          description: "The requested product could not be found.",
          variant: "destructive",
        });
        navigate('/seller/dashboard');
      }
    }
  }, [id, isEditMode, form, navigate, toast]);

  const onSubmit = async (data: ProductFormValues) => {
    if (images.length === 0) {
      toast({
        title: "Image required",
        description: "Please upload at least one product image.",
        variant: "destructive",
      });
      return;
    }
    
    setIsSubmitting(true);
    
    try {
      // In a real app, you would send this data to your API
      const productData = {
        ...data,
        price: parseFloat(data.price),
        stock: parseInt(data.stock),
        images,
        // Other fields would be handled by the backend
      };
      
      console.log('Product data to be saved:', productData);
      
      // Success message and redirect
      toast({
        title: isEditMode ? "Product updated" : "Product created",
        description: isEditMode 
          ? "Your product has been successfully updated." 
          : "Your product has been successfully created.",
      });
      
      navigate('/seller/dashboard');
    } catch (error) {
      console.error('Error saving product:', error);
      toast({
        title: "Error",
        description: "There was a problem saving your product. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleImageUpload = (uploadedImageUrls: string[]) => {
    setImages(prev => [...prev, ...uploadedImageUrls]);
  };

  const handleImageRemove = (indexToRemove: number) => {
    setImages(images.filter((_, index) => index !== indexToRemove));
  };
  
  return (
    <Layout>
      <div className="container mx-auto py-8 px-4">
        <h1 className="text-2xl font-bold mb-6">
          {isEditMode ? 'Edit Product' : 'Create New Product'}
        </h1>
        
        <Card>
          <CardHeader>
            <CardTitle>Product Information</CardTitle>
          </CardHeader>
          
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)}>
              <CardContent className="space-y-6">
                {/* Image Upload Section */}
                <div className="space-y-2">
                  <FormLabel>Product Images</FormLabel>
                  <ImageUploader 
                    images={images}
                    onUpload={handleImageUpload}
                    onRemove={handleImageRemove}
                    maxImages={6}
                  />
                  <p className="text-sm text-muted-foreground">
                    Upload up to 6 images. First image will be the main product image.
                    Recommended size: 1000x1000px. Supported formats: JPG, PNG.
                  </p>
                </div>
                
                {/* Basic Information */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <FormField
                    control={form.control}
                    name="title"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Product Title</FormLabel>
                        <FormControl>
                          <Input placeholder="Enter product title" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  
                  <FormField
                    control={form.control}
                    name="categoryId"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Category</FormLabel>
                        <Select 
                          onValueChange={field.onChange} 
                          defaultValue={field.value}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue placeholder="Select a category" />
                            </SelectTrigger>
                          </FormControl>
                          <SelectContent>
                            {mockCategories.map(category => (
                              <SelectItem key={category.id} value={category.id}>
                                {category.name}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </Select>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </div>
                
                <FormField
                  control={form.control}
                  name="description"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Description</FormLabel>
                      <FormControl>
                        <Textarea 
                          placeholder="Describe your product in detail" 
                          className="min-h-[120px]"
                          {...field} 
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  <FormField
                    control={form.control}
                    name="price"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Price ($)</FormLabel>
                        <FormControl>
                          <Input type="number" step="0.01" min="0" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  
                  <FormField
                    control={form.control}
                    name="stock"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Stock</FormLabel>
                        <FormControl>
                          <Input type="number" min="0" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  
                  <FormField
                    control={form.control}
                    name="location"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Location</FormLabel>
                        <FormControl>
                          <Input placeholder="City, Country" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </div>
              </CardContent>
              
              <CardFooter className="flex justify-between">
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => navigate('/seller/dashboard')}
                >
                  Cancel
                </Button>
                <Button type="submit" disabled={isSubmitting}>
                  {isSubmitting ? 'Saving...' : isEditMode ? 'Update Product' : 'Create Product'}
                </Button>
              </CardFooter>
            </form>
          </Form>
        </Card>
      </div>
    </Layout>
  );
};

export default ProductForm;
