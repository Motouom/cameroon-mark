
import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useToast } from '@/components/ui/use-toast';

const NewsletterSection = () => {
  const [email, setEmail] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { toast } = useToast();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!email) {
      toast({
        title: "Error",
        description: "Please enter your email address",
        variant: "destructive",
      });
      return;
    }
    
    setIsLoading(true);
    
    // Simulate API call
    setTimeout(() => {
      toast({
        title: "Success!",
        description: "You've been subscribed to our newsletter",
      });
      setEmail('');
      setIsLoading(false);
    }, 1000);
  };

  return (
    <section className="py-16 bg-cameroon-green">
      <div className="container-custom mx-auto">
        <div className="flex flex-col lg:flex-row items-center justify-between">
          {/* Text Content */}
          <div className="mb-8 lg:mb-0 lg:mr-8 text-center lg:text-left">
            <h2 className="text-2xl md:text-3xl font-bold text-white mb-2">
              Stay Updated
            </h2>
            <p className="text-white/80 max-w-md">
              Subscribe to our newsletter to get updates on new products, sellers, and special promotions.
            </p>
          </div>
          
          {/* Form */}
          <div className="w-full max-w-md">
            <form onSubmit={handleSubmit} className="flex">
              <Input
                type="email"
                placeholder="Your email address"
                className="bg-white/10 border-white/20 text-white placeholder:text-white/50 focus-visible:ring-white flex-grow"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
              />
              <Button 
                type="submit"
                className="ml-2 bg-cameroon-yellow text-black hover:bg-cameroon-yellow/90 whitespace-nowrap"
                disabled={isLoading}
              >
                {isLoading ? 'Subscribing...' : 'Subscribe'}
              </Button>
            </form>
            <p className="text-sm text-white/60 mt-2">
              We respect your privacy. Unsubscribe at any time.
            </p>
          </div>
        </div>
      </div>
    </section>
  );
};

export default NewsletterSection;
