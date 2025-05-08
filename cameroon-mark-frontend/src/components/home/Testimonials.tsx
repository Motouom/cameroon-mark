
import { useState } from 'react';
import { testimonials } from '@/data/mockData';
import { Star, ChevronLeft, ChevronRight } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';

const Testimonials = () => {
  const [activeIndex, setActiveIndex] = useState(0);

  const nextTestimonial = () => {
    setActiveIndex((prev) => (prev + 1) % testimonials.length);
  };

  const prevTestimonial = () => {
    setActiveIndex((prev) => (prev - 1 + testimonials.length) % testimonials.length);
  };

  return (
    <section className="py-16 bg-cameroon-green/5 dark:bg-gray-900">
      <div className="container-custom mx-auto">
        {/* Section Title */}
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold mb-3 text-gray-900 dark:text-white">What People Say</h2>
          <p className="text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
            Hear from our sellers and buyers about their experience with Cameroon Mark
          </p>
        </div>
        
        {/* Desktop - Grid Layout */}
        <div className="hidden lg:grid grid-cols-3 gap-6">
          {testimonials.map((testimonial) => (
            <Card key={testimonial.id} className="border border-gray-200 dark:border-gray-800 hover-scale">
              <CardContent className="p-6">
                {/* Stars */}
                <div className="flex mb-4">
                  {[...Array(5)].map((_, i) => (
                    <Star 
                      key={i} 
                      size={16} 
                      className={`${
                        i < testimonial.rating 
                          ? "text-yellow-400 fill-yellow-400" 
                          : "text-gray-300 dark:text-gray-600"
                      }`} 
                    />
                  ))}
                </div>
                
                {/* Content */}
                <p className="text-gray-700 dark:text-gray-300 mb-4 line-clamp-4">"{testimonial.content}"</p>
                
                {/* User */}
                <div className="flex items-center">
                  <Avatar className="h-10 w-10 mr-3">
                    <AvatarImage src={testimonial.avatar} alt={testimonial.name} />
                    <AvatarFallback>{testimonial.name.charAt(0)}</AvatarFallback>
                  </Avatar>
                  <div>
                    <p className="font-medium text-gray-900 dark:text-white">{testimonial.name}</p>
                    <p className="text-sm text-gray-500 dark:text-gray-400">{testimonial.role}</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
        
        {/* Mobile & Tablet - Slider */}
        <div className="lg:hidden">
          <Card className="border border-gray-200 dark:border-gray-800">
            <CardContent className="p-6">
              {/* Stars */}
              <div className="flex mb-4">
                {[...Array(5)].map((_, i) => (
                  <Star 
                    key={i} 
                    size={16} 
                    className={`${
                      i < testimonials[activeIndex].rating 
                        ? "text-yellow-400 fill-yellow-400" 
                        : "text-gray-300 dark:text-gray-600"
                    }`} 
                  />
                ))}
              </div>
              
              {/* Content */}
              <p className="text-gray-700 dark:text-gray-300 mb-8">"{testimonials[activeIndex].content}"</p>
              
              {/* User */}
              <div className="flex items-center">
                <Avatar className="h-12 w-12 mr-4">
                  <AvatarImage src={testimonials[activeIndex].avatar} alt={testimonials[activeIndex].name} />
                  <AvatarFallback>{testimonials[activeIndex].name.charAt(0)}</AvatarFallback>
                </Avatar>
                <div>
                  <p className="font-medium text-gray-900 dark:text-white">{testimonials[activeIndex].name}</p>
                  <p className="text-sm text-gray-500 dark:text-gray-400">{testimonials[activeIndex].role}</p>
                </div>
              </div>
            </CardContent>
          </Card>
          
          {/* Navigation Buttons */}
          <div className="flex justify-center mt-6 space-x-2">
            <Button variant="outline" size="icon" onClick={prevTestimonial}>
              <ChevronLeft size={18} />
            </Button>
            {testimonials.map((_, index) => (
              <Button 
                key={index} 
                variant={index === activeIndex ? "default" : "outline"} 
                size="sm" 
                className={`w-8 h-8 p-0 ${index === activeIndex ? "bg-cameroon-green" : ""}`}
                onClick={() => setActiveIndex(index)}
              >
                {index + 1}
              </Button>
            ))}
            <Button variant="outline" size="icon" onClick={nextTestimonial}>
              <ChevronRight size={18} />
            </Button>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Testimonials;
