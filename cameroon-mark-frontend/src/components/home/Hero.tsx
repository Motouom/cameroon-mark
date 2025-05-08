
import { Link } from 'react-router-dom';
import { Button } from '@/components/ui/button';

const Hero = () => {
  return (
    <div className="relative overflow-hidden">
      {/* Background */}
      <div className="absolute inset-0 bg-gradient-to-r from-cameroon-green/90 to-cameroon-forest/80 z-0"></div>
      <div className="absolute inset-0 bg-[url('/images/hero-pattern.png')] opacity-10 z-0"></div>
      
      {/* Hero Content */}
      <div className="container-custom mx-auto relative z-10 py-20 lg:py-28 flex flex-col lg:flex-row items-center">
        {/* Text Content */}
        <div className="w-full lg:w-1/2 text-white space-y-6">
          <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold leading-tight">
            <span className="block">Discover Authentic</span>
            <span className="block text-cameroon-yellow">Cameroonian Products</span>
          </h1>
          
          <p className="text-lg md:text-xl opacity-90 max-w-lg">
            Connect directly with artisans, farmers, and small businesses across Cameroon and shop their unique products.
          </p>
          
          <div className="flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-4 pt-4">
            <Link to="/marketplace">
              <Button size="lg" className="bg-cameroon-yellow text-black hover:bg-cameroon-yellow/90 text-lg px-8">
                Shop Now
              </Button>
            </Link>
            <Link to="/register?role=seller">
              <Button size="lg" variant="outline" className="border-white text-white hover:bg-white/10 text-lg px-8">
                Become a Seller
              </Button>
            </Link>
          </div>
        </div>
        
        {/* Image/Decoration */}
        <div className="w-full lg:w-1/2 mt-12 lg:mt-0">
          <div className="relative">
            {/* Main Image */}
            <div className="relative z-10 rounded-2xl overflow-hidden shadow-2xl transform lg:translate-x-10">
              <img src="/images/hero-main.jpg" alt="Cameroonian marketplace" className="w-full h-auto" />
            </div>
            
            {/* Floating Product Cards */}
            <div className="absolute -left-4 top-1/4 z-20 bg-white dark:bg-gray-800 rounded-lg shadow-xl p-3 transform -rotate-6 animate-fade-in hidden md:block">
              <div className="flex items-center space-x-3">
                <div className="w-12 h-12 rounded-md overflow-hidden">
                  <img src="/images/products/mask1.jpg" alt="Product" className="w-full h-full object-cover" />
                </div>
                <div>
                  <p className="text-sm font-medium text-gray-900 dark:text-white">Traditional Masks</p>
                  <p className="text-xs text-green-600 font-medium">+35% sales</p>
                </div>
              </div>
            </div>
            
            <div className="absolute -right-4 bottom-1/4 z-20 bg-white dark:bg-gray-800 rounded-lg shadow-xl p-3 transform rotate-6 animate-fade-in hidden md:block">
              <div className="flex items-center space-x-3">
                <div className="w-12 h-12 rounded-md overflow-hidden">
                  <img src="/images/products/coffee1.jpg" alt="Product" className="w-full h-full object-cover" />
                </div>
                <div>
                  <p className="text-sm font-medium text-gray-900 dark:text-white">Organic Coffee</p>
                  <p className="text-xs text-green-600 font-medium">Best seller</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      {/* Wave Bottom */}
      <div className="absolute bottom-0 left-0 right-0">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1440 160" className="fill-white dark:fill-gray-950">
          <path fillOpacity="1" d="M0,128L80,117.3C160,107,320,85,480,90.7C640,96,800,128,960,138.7C1120,149,1280,139,1360,133.3L1440,128L1440,320L1360,320C1280,320,1120,320,960,320C800,320,640,320,480,320C320,320,160,320,80,320L0,320Z"></path>
        </svg>
      </div>
    </div>
  );
};

export default Hero;
