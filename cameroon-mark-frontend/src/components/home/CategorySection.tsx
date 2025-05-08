
import { Link } from 'react-router-dom';
import { categories } from '@/data/mockData';
import { ChevronRight } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';

const CategorySection = () => {
  return (
    <section className="py-16 bg-white dark:bg-gray-950">
      <div className="container-custom mx-auto">
        {/* Section Title */}
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold mb-3 text-gray-900 dark:text-white">Explore Categories</h2>
          <p className="text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
            Discover the rich variety of authentic Cameroonian products across multiple categories
          </p>
        </div>
        
        {/* Categories Grid */}
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6">
          {categories.map((category) => (
            <Link to={`/marketplace/category/${category.slug}`} key={category.id}>
              <Card className="h-full hover-scale overflow-hidden border border-gray-200 dark:border-gray-800">
                <CardContent className="p-0">
                  <div className="aspect-square w-full relative overflow-hidden">
                    <img 
                      src={category.image} 
                      alt={category.name} 
                      className="w-full h-full object-cover"
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-black/70 to-transparent flex items-end">
                      <div className="p-4 w-full">
                        <h3 className="text-white font-medium text-sm md:text-base truncate">
                          {category.name}
                        </h3>
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </Link>
          ))}
        </div>
        
        {/* View All Button */}
        <div className="text-center mt-10">
          <Link 
            to="/marketplace" 
            className="inline-flex items-center text-cameroon-green hover:text-cameroon-green/80 font-medium"
          >
            View All Categories
            <ChevronRight size={18} className="ml-1" />
          </Link>
        </div>
      </div>
    </section>
  );
};

export default CategorySection;
