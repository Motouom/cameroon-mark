
import { Category, Product, Testimonial, User } from '../types';

// Mock Categories
export const categories: Category[] = [
  {
    id: '1',
    name: 'Art & Crafts',
    description: 'Handcrafted artwork and traditional crafts',
    image: '/images/categories/art.jpg',
    slug: 'art-crafts',
  },
  {
    id: '2',
    name: 'Clothing & Textiles',
    description: 'Traditional and modern Cameroonian clothing',
    image: '/images/categories/clothing.jpg',
    slug: 'clothing-textiles',
  },
  {
    id: '3',
    name: 'Agriculture',
    description: 'Fresh produce and agricultural products',
    image: '/images/categories/agriculture.jpg',
    slug: 'agriculture',
  },
  {
    id: '4',
    name: 'Food & Spices',
    description: 'Traditional Cameroonian foods, spices, and ingredients',
    image: '/images/categories/food.jpg',
    slug: 'food-spices',
  },
  {
    id: '5',
    name: 'Home & Decor',
    description: 'Home decoration items with Cameroonian influence',
    image: '/images/categories/home.jpg',
    slug: 'home-decor',
  },
  {
    id: '6',
    name: 'Jewelry',
    description: 'Traditional and contemporary Cameroonian jewelry',
    image: '/images/categories/jewelry.jpg',
    slug: 'jewelry',
  },
];

// Mock Sellers
export const sellers: User[] = [
  {
    id: '1',
    name: 'Amina Nkongho',
    email: 'amina@example.com',
    role: 'seller',
    avatar: '/images/avatars/seller1.jpg',
    phone: '+237 6XX XXX XXX',
    location: 'Douala',
    createdAt: new Date('2022-05-15'),
  },
  {
    id: '2',
    name: 'Emmanuel Fomba',
    email: 'emmanuel@example.com',
    role: 'seller',
    avatar: '/images/avatars/seller2.jpg',
    phone: '+237 6XX XXX XXX',
    location: 'Yaoundé',
    createdAt: new Date('2022-07-22'),
  },
  {
    id: '3',
    name: 'Marie Eteki',
    email: 'marie@example.com',
    role: 'seller',
    avatar: '/images/avatars/seller3.jpg',
    phone: '+237 6XX XXX XXX',
    location: 'Bamenda',
    createdAt: new Date('2022-03-10'),
  },
];

// Mock Products
export const products: Product[] = [
  {
    id: '1',
    title: 'Hand-carved Wooden Mask',
    description: 'Traditional Cameroonian mask, hand-carved from local hardwood by skilled artisans.',
    price: 12000,
    images: ['/images/products/mask1.jpg', '/images/products/mask2.jpg'],
    categoryId: '1',
    sellerId: '1',
    seller: sellers[0],
    rating: 4.8,
    stock: 5,
    location: 'Douala',
    createdAt: new Date('2023-01-15'),
    featured: true,
  },
  {
    id: '2',
    title: 'Traditional Ankara Dress',
    description: 'Vibrant Ankara print dress, handmade with authentic fabric. Perfect for special occasions.',
    price: 15000,
    images: ['/images/products/dress1.jpg', '/images/products/dress2.jpg'],
    categoryId: '2',
    sellerId: '2',
    seller: sellers[1],
    rating: 4.5,
    stock: 10,
    location: 'Yaoundé',
    createdAt: new Date('2023-02-20'),
    featured: true,
  },
  {
    id: '3',
    title: 'Organic Coffee Beans',
    description: 'Premium organic coffee beans from the highlands of Cameroon. Rich, aromatic flavor.',
    price: 3500,
    images: ['/images/products/coffee1.jpg', '/images/products/coffee2.jpg'],
    categoryId: '3',
    sellerId: '3',
    seller: sellers[2],
    rating: 4.9,
    stock: 25,
    location: 'Bamenda',
    createdAt: new Date('2023-01-05'),
    featured: true,
  },
  {
    id: '4',
    title: 'Ndolé Spice Mix',
    description: 'Authentic spice mix for preparing traditional Cameroonian Ndolé dish.',
    price: 2000,
    images: ['/images/products/spice1.jpg', '/images/products/spice2.jpg'],
    categoryId: '4',
    sellerId: '1',
    seller: sellers[0],
    rating: 4.7,
    stock: 30,
    location: 'Douala',
    createdAt: new Date('2023-03-10'),
  },
  {
    id: '5',
    title: 'Hand-woven Basket',
    description: 'Beautiful hand-woven storage basket made with traditional techniques.',
    price: 8000,
    images: ['/images/products/basket1.jpg', '/images/products/basket2.jpg'],
    categoryId: '5',
    sellerId: '2',
    seller: sellers[1],
    rating: 4.6,
    stock: 15,
    location: 'Yaoundé',
    createdAt: new Date('2023-02-15'),
  },
  {
    id: '6',
    title: 'Beaded Necklace Set',
    description: 'Handcrafted beaded necklace and earring set, using traditional Cameroonian designs.',
    price: 7500,
    images: ['/images/products/jewelry1.jpg', '/images/products/jewelry2.jpg'],
    categoryId: '6',
    sellerId: '3',
    seller: sellers[2],
    rating: 4.8,
    stock: 20,
    location: 'Bamenda',
    createdAt: new Date('2023-01-25'),
  },
  {
    id: '7',
    title: 'Bamboo Sculpture',
    description: 'Contemporary sculpture crafted from sustainable bamboo.',
    price: 20000,
    images: ['/images/products/sculpture1.jpg', '/images/products/sculpture2.jpg'],
    categoryId: '1',
    sellerId: '1',
    seller: sellers[0],
    rating: 4.5,
    stock: 3,
    location: 'Douala',
    createdAt: new Date('2023-03-20'),
  },
  {
    id: '8',
    title: 'Traditional Headwrap',
    description: 'Colorful headwrap made from authentic Cameroonian fabric.',
    price: 4000,
    images: ['/images/products/headwrap1.jpg', '/images/products/headwrap2.jpg'],
    categoryId: '2',
    sellerId: '2',
    seller: sellers[1],
    rating: 4.7,
    stock: 25,
    location: 'Yaoundé',
    createdAt: new Date('2023-04-05'),
  },
];

// Mock Testimonials
export const testimonials: Testimonial[] = [
  {
    id: '1',
    name: 'Sophie Mbango',
    role: 'Fashion Designer',
    content: 'Cameroon Mark has transformed my business, allowing me to reach customers internationally. The platform is intuitive, and the support team is incredibly helpful.',
    avatar: '/images/testimonials/testimonial1.jpg',
    rating: 5,
  },
  {
    id: '2',
    name: 'Jean-Paul Nkoudou',
    role: 'Coffee Farmer',
    content: 'I\'ve been able to sell my organic coffee directly to consumers without middlemen. This has significantly improved my profit margins and let me grow my farm.',
    avatar: '/images/testimonials/testimonial2.jpg',
    rating: 5,
  },
  {
    id: '3',
    name: 'Rebecca Tamba',
    role: 'International Buyer',
    content: 'The quality of Cameroonian products available on this marketplace is outstanding. I appreciate the direct connection with artisans and the authentic experience.',
    avatar: '/images/testimonials/testimonial3.jpg',
    rating: 4,
  },
];

// Function to get products by category
export const getProductsByCategory = (categoryId: string): Product[] => {
  return products.filter(product => product.categoryId === categoryId);
};

// Function to get featured products
export const getFeaturedProducts = (): Product[] => {
  return products.filter(product => product.featured);
};

// Function to get product by ID
export const getProductById = (id: string): Product | undefined => {
  return products.find(product => product.id === id);
};

// Function to get category by ID
export const getCategoryById = (id: string): Category | undefined => {
  return categories.find(category => category.id === id);
};

// Function to search products
export const searchProducts = (query: string): Product[] => {
  query = query.toLowerCase();
  return products.filter(
    product => 
      product.title.toLowerCase().includes(query) ||
      product.description.toLowerCase().includes(query) ||
      product.location.toLowerCase().includes(query)
  );
};
