
import { useState, useEffect } from 'react';
import { Link, useSearchParams } from 'react-router-dom';
import Layout from '@/components/layout/Layout';
import ProductCard from '@/components/product/ProductCard';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Slider } from '@/components/ui/slider';
import { 
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Search, Filter, X, ChevronRight, Grid, List } from 'lucide-react';
import { categories, products, searchProducts } from '@/data/mockData';
import { Product } from '@/types';

const Marketplace = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const initialQuery = searchParams.get('query') || '';
  const initialCategoryId = searchParams.get('category') || '';
  
  const [searchQuery, setSearchQuery] = useState(initialQuery);
  const [selectedCategory, setSelectedCategory] = useState(initialCategoryId);
  const [priceRange, setPriceRange] = useState<[number, number]>([0, 50000]);
  const [sortBy, setSortBy] = useState('latest');
  const [filteredProducts, setFilteredProducts] = useState<Product[]>([]);
  const [isFilterOpen, setIsFilterOpen] = useState(false);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');

  // Filtering products based on search, category, and price
  useEffect(() => {
    let results = [...products];
    
    // Filter by search query
    if (searchQuery) {
      results = searchProducts(searchQuery);
    }
    
    // Filter by category
    if (selectedCategory) {
      results = results.filter(product => product.categoryId === selectedCategory);
    }
    
    // Filter by price range
    results = results.filter(
      product => product.price >= priceRange[0] && product.price <= priceRange[1]
    );
    
    // Sort products
    switch (sortBy) {
      case 'price-low':
        results.sort((a, b) => a.price - b.price);
        break;
      case 'price-high':
        results.sort((a, b) => b.price - a.price);
        break;
      case 'rating':
        results.sort((a, b) => (b.rating || 0) - (a.rating || 0));
        break;
      case 'latest':
      default:
        results.sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime());
        break;
    }
    
    setFilteredProducts(results);
    
    // Update URL params
    const params = new URLSearchParams();
    if (searchQuery) params.set('query', searchQuery);
    if (selectedCategory) params.set('category', selectedCategory);
    setSearchParams(params);
  }, [searchQuery, selectedCategory, priceRange, sortBy, setSearchParams]);

  const resetFilters = () => {
    setSearchQuery('');
    setSelectedCategory('');
    setPriceRange([0, 50000]);
    setSortBy('latest');
    setSearchParams({});
  };

  const toggleFilterSidebar = () => {
    setIsFilterOpen(!isFilterOpen);
  };

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    // Search is already handled by the useEffect
  };

  return (
    <Layout>
      <div className="bg-gray-50 dark:bg-gray-900 min-h-screen">
        {/* Hero Section */}
        <div className="bg-cameroon-green py-10 px-4">
          <div className="container-custom mx-auto">
            <div className="max-w-3xl mx-auto text-center text-white">
              <h1 className="text-3xl md:text-4xl font-bold mb-4">Marketplace</h1>
              <p className="text-lg mb-6 text-white/90">
                Explore our collection of authentic Cameroonian products from trusted sellers
              </p>
              
              {/* Search Bar */}
              <form onSubmit={handleSearch} className="relative">
                <Input
                  type="text"
                  placeholder="Search for products, categories, or sellers..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="py-6 pl-12 pr-4 rounded-full bg-white/10 border-white/20 placeholder:text-white/50 text-white"
                />
                <Search className="absolute left-4 top-1/2 transform -translate-y-1/2 h-5 w-5 text-white" />
                {searchQuery && (
                  <button 
                    type="button" 
                    className="absolute right-4 top-1/2 transform -translate-y-1/2 text-white/70 hover:text-white"
                    onClick={() => setSearchQuery('')}
                  >
                    <X size={18} />
                  </button>
                )}
              </form>
            </div>
          </div>
        </div>
        
        {/* Breadcrumbs */}
        <div className="container-custom mx-auto py-4">
          <div className="flex items-center text-sm">
            <Link to="/" className="text-gray-500 dark:text-gray-400 hover:text-cameroon-green">Home</Link>
            <ChevronRight size={14} className="mx-2 text-gray-400" />
            <span className="text-gray-900 dark:text-white">Marketplace</span>
            {selectedCategory && (
              <>
                <ChevronRight size={14} className="mx-2 text-gray-400" />
                <span className="text-gray-900 dark:text-white">
                  {categories.find(c => c.id === selectedCategory)?.name || 'Category'}
                </span>
              </>
            )}
          </div>
        </div>

        {/* Main Content */}
        <div className="container-custom mx-auto pb-16">
          <div className="flex flex-col md:flex-row gap-6">
            {/* Filter Sidebar - Desktop */}
            <div className="hidden md:block w-64 flex-shrink-0">
              <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-5 sticky top-20">
                <div className="flex justify-between items-center mb-5">
                  <h2 className="font-semibold text-gray-900 dark:text-white">Filters</h2>
                  <Button 
                    variant="ghost" 
                    size="sm" 
                    onClick={resetFilters}
                    className="text-gray-500 hover:text-cameroon-green text-xs"
                  >
                    Reset All
                  </Button>
                </div>
                
                {/* Categories */}
                <div className="mb-6">
                  <h3 className="font-medium text-sm mb-3">Categories</h3>
                  <div className="space-y-2">
                    <div 
                      className={`px-2 py-1.5 rounded-md cursor-pointer flex items-center ${
                        selectedCategory === '' 
                          ? 'bg-cameroon-green/10 text-cameroon-green' 
                          : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
                      }`}
                      onClick={() => setSelectedCategory('')}
                    >
                      <span>All Categories</span>
                      {selectedCategory === '' && <div className="w-1.5 h-1.5 ml-auto bg-cameroon-green rounded-full" />}
                    </div>
                    
                    {categories.map(category => (
                      <div 
                        key={category.id}
                        className={`px-2 py-1.5 rounded-md cursor-pointer flex items-center ${
                          selectedCategory === category.id 
                            ? 'bg-cameroon-green/10 text-cameroon-green' 
                            : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
                        }`}
                        onClick={() => setSelectedCategory(category.id)}
                      >
                        <span>{category.name}</span>
                        {selectedCategory === category.id && <div className="w-1.5 h-1.5 ml-auto bg-cameroon-green rounded-full" />}
                      </div>
                    ))}
                  </div>
                </div>
                
                {/* Price Range */}
                <div className="mb-6">
                  <h3 className="font-medium text-sm mb-4">Price Range</h3>
                  <div className="px-2">
                    <Slider
                      defaultValue={priceRange}
                      min={0}
                      max={50000}
                      step={1000}
                      value={priceRange}
                      onValueChange={(value) => setPriceRange(value as [number, number])}
                      className="mb-6"
                    />
                    <div className="flex items-center justify-between text-sm">
                      <span>{priceRange[0].toLocaleString()} FCFA</span>
                      <span>{priceRange[1].toLocaleString()} FCFA</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Products Section */}
            <div className="flex-1">
              {/* Mobile Filter Button */}
              <div className="flex items-center justify-between mb-6 md:hidden">
                <Button 
                  variant="outline" 
                  size="sm" 
                  onClick={toggleFilterSidebar}
                  className="flex items-center"
                >
                  <Filter size={16} className="mr-2" />
                  Filters
                </Button>
                
                {/* Sort By (Mobile) */}
                <div className="flex-1 ml-4">
                  <Select value={sortBy} onValueChange={setSortBy}>
                    <SelectTrigger className="text-sm h-9">
                      <SelectValue placeholder="Sort by" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="latest">Latest</SelectItem>
                      <SelectItem value="price-low">Price: Low to High</SelectItem>
                      <SelectItem value="price-high">Price: High to Low</SelectItem>
                      <SelectItem value="rating">Highest Rated</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
              
              {/* Mobile Filter Sidebar */}
              {isFilterOpen && (
                <div className="fixed inset-0 bg-black/50 z-50 md:hidden" onClick={toggleFilterSidebar}>
                  <div 
                    className="absolute top-0 left-0 h-full w-80 max-w-full bg-white dark:bg-gray-800 shadow-xl p-5 animate-slide-in"
                    onClick={e => e.stopPropagation()}
                  >
                    <div className="flex justify-between items-center mb-5">
                      <h2 className="font-semibold text-gray-900 dark:text-white">Filters</h2>
                      <Button 
                        variant="ghost" 
                        size="icon"
                        onClick={toggleFilterSidebar}
                      >
                        <X size={18} />
                      </Button>
                    </div>
                    
                    {/* Categories */}
                    <div className="mb-6">
                      <h3 className="font-medium text-sm mb-3">Categories</h3>
                      <div className="space-y-2">
                        <div 
                          className={`px-2 py-1.5 rounded-md cursor-pointer flex items-center ${
                            selectedCategory === '' 
                              ? 'bg-cameroon-green/10 text-cameroon-green' 
                              : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
                          }`}
                          onClick={() => {
                            setSelectedCategory('');
                            toggleFilterSidebar();
                          }}
                        >
                          <span>All Categories</span>
                          {selectedCategory === '' && <div className="w-1.5 h-1.5 ml-auto bg-cameroon-green rounded-full" />}
                        </div>
                        
                        {categories.map(category => (
                          <div 
                            key={category.id}
                            className={`px-2 py-1.5 rounded-md cursor-pointer flex items-center ${
                              selectedCategory === category.id 
                                ? 'bg-cameroon-green/10 text-cameroon-green' 
                                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
                            }`}
                            onClick={() => {
                              setSelectedCategory(category.id);
                              toggleFilterSidebar();
                            }}
                          >
                            <span>{category.name}</span>
                            {selectedCategory === category.id && <div className="w-1.5 h-1.5 ml-auto bg-cameroon-green rounded-full" />}
                          </div>
                        ))}
                      </div>
                    </div>
                    
                    {/* Price Range */}
                    <div className="mb-6">
                      <h3 className="font-medium text-sm mb-4">Price Range</h3>
                      <div className="px-2">
                        <Slider
                          defaultValue={priceRange}
                          min={0}
                          max={50000}
                          step={1000}
                          value={priceRange}
                          onValueChange={(value) => setPriceRange(value as [number, number])}
                          className="mb-6"
                        />
                        <div className="flex items-center justify-between text-sm">
                          <span>{priceRange[0].toLocaleString()} FCFA</span>
                          <span>{priceRange[1].toLocaleString()} FCFA</span>
                        </div>
                      </div>
                    </div>
                    
                    <div className="flex space-x-3 mt-8">
                      <Button 
                        variant="outline" 
                        className="flex-1"
                        onClick={resetFilters}
                      >
                        Reset
                      </Button>
                      <Button 
                        className="flex-1 bg-cameroon-green hover:bg-cameroon-green/90"
                        onClick={toggleFilterSidebar}
                      >
                        Apply
                      </Button>
                    </div>
                  </div>
                </div>
              )}
              
              {/* Results Header */}
              <div className="flex flex-col md:flex-row items-start md:items-center justify-between mb-6">
                <div className="mb-4 md:mb-0">
                  <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-1">
                    {filteredProducts.length} Products
                  </h2>
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    {selectedCategory 
                      ? `Showing ${categories.find(c => c.id === selectedCategory)?.name} products` 
                      : 'Showing all products'}
                  </p>
                </div>
                
                {/* Sort and View Controls - Desktop */}
                <div className="flex items-center space-x-4 self-stretch md:self-auto w-full md:w-auto">
                  {/* Sort By */}
                  <div className="hidden md:block">
                    <Select value={sortBy} onValueChange={setSortBy}>
                      <SelectTrigger className="text-sm h-9 w-40">
                        <SelectValue placeholder="Sort by" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="latest">Latest</SelectItem>
                        <SelectItem value="price-low">Price: Low to High</SelectItem>
                        <SelectItem value="price-high">Price: High to Low</SelectItem>
                        <SelectItem value="rating">Highest Rated</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  
                  {/* View Mode */}
                  <div className="hidden md:flex items-center bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md">
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => setViewMode('grid')}
                      className={`h-9 w-9 rounded-none rounded-l-md ${viewMode === 'grid' ? 'bg-gray-100 dark:bg-gray-700' : ''}`}
                    >
                      <Grid size={16} />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => setViewMode('list')}
                      className={`h-9 w-9 rounded-none rounded-r-md ${viewMode === 'list' ? 'bg-gray-100 dark:bg-gray-700' : ''}`}
                    >
                      <List size={16} />
                    </Button>
                  </div>
                </div>
              </div>
              
              {/* Products Grid */}
              {filteredProducts.length === 0 ? (
                <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-10 text-center">
                  <div className="text-gray-400 dark:text-gray-500 mb-4">
                    <Search size={48} className="mx-auto" />
                  </div>
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                    No products found
                  </h3>
                  <p className="text-gray-600 dark:text-gray-400 mb-6">
                    Try adjusting your search or filters to find what you're looking for.
                  </p>
                  <Button 
                    variant="outline" 
                    onClick={resetFilters}
                  >
                    Reset Filters
                  </Button>
                </div>
              ) : (
                <div className={`grid ${viewMode === 'grid' ? 'grid-cols-2 sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-3' : 'grid-cols-1'} gap-6`}>
                  {filteredProducts.map(product => (
                    <ProductCard key={product.id} product={product} />
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default Marketplace;
